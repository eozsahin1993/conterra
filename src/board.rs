use crate::hex::Hex;
use crate::species::{self, FoodWebEdge, Species};
use crate::terrain::Terrain;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub radius: i32,
    valid_hexes: HashSet<Hex>,
    pub terrain: HashMap<Hex, Terrain>,
    pub animals: HashMap<Hex, Species>,
}

impl Board {
    pub fn new(radius: i32) -> Self {
        let valid_hexes: HashSet<Hex> = Hex::spiral_from_origin(radius).into_iter().collect();
        Board {
            radius,
            valid_hexes,
            terrain: HashMap::new(),
            animals: HashMap::new(),
        }
    }

    pub fn in_bounds(&self, hex: &Hex) -> bool {
        self.valid_hexes.contains(hex)
    }

    pub fn is_empty(&self, hex: &Hex) -> bool {
        self.in_bounds(hex) && !self.terrain.contains_key(hex)
    }

    /// Places a whole procedurally-grown shape at once, given already
    /// rotated+translated absolute hex positions paired with their per-hex
    /// terrain (parallel to a `MarketOption::TerrainShape`'s `terrains`).
    /// Caller has already verified every hex is a legal placement (see
    /// `market::can_place_shape`).
    pub fn place_terrain_shape(&mut self, placements: &[(Hex, Terrain)]) {
        for (hex, terrain) in placements {
            self.terrain.insert(*hex, *terrain);
        }
    }

    pub fn place_animal(&mut self, hex: Hex, species: Species) -> Result<(), String> {
        if !self.in_bounds(&hex) {
            return Err("hex out of bounds".into());
        }
        let Some(&terrain) = self.terrain.get(&hex) else {
            return Err("no terrain placed on that tile yet".into());
        };
        let edges = species::food_web();
        if !species::species_terrains(&edges, species).contains(&terrain) {
            return Err(format!(
                "{} cannot live on {:?}",
                species.name(),
                terrain
            ));
        }
        if self.animals.contains_key(&hex) {
            return Err("tile already has an animal".into());
        }
        self.animals.insert(hex, species);
        Ok(())
    }

    pub fn terrain_tile_count(&self, terrain: Terrain) -> usize {
        self.terrain.values().filter(|t| **t == terrain).count()
    }

    pub fn animal_count(&self, species: Species) -> usize {
        self.animals.values().filter(|s| **s == species).count()
    }

    pub fn animal_positions(&self, species: Species) -> Vec<Hex> {
        self.animals
            .iter()
            .filter(|(_, s)| **s == species)
            .map(|(h, _)| *h)
            .collect()
    }

    pub fn neighbors_in_bounds(&self, hex: &Hex) -> Vec<Hex> {
        hex.neighbors()
            .into_iter()
            .filter(|n| self.in_bounds(n))
            .collect()
    }

    /// Longest run of contiguous same-terrain tiles, Cascadia-style corridor
    /// scoring — brief item 3.
    pub fn longest_terrain_corridor(&self) -> (Terrain, usize) {
        let mut best = (Terrain::Forest, 0usize);
        let mut visited: HashSet<Hex> = HashSet::new();
        for (&hex, &terrain) in self.terrain.iter() {
            if visited.contains(&hex) {
                continue;
            }
            let mut stack = vec![hex];
            let mut component = Vec::new();
            visited.insert(hex);
            while let Some(h) = stack.pop() {
                component.push(h);
                for n in h.neighbors() {
                    if self.terrain.get(&n) == Some(&terrain) && !visited.contains(&n) {
                        visited.insert(n);
                        stack.push(n);
                    }
                }
            }
            if component.len() > best.1 {
                best = (terrain, component.len());
            }
        }
        best
    }

    /// Habitat maturity check: is `terrain` established enough (tile count)
    /// to meet `threshold`? Used to gate animal-placement eligibility.
    pub fn terrain_meets_threshold(&self, terrain: Terrain, threshold: usize) -> bool {
        self.terrain_tile_count(terrain) >= threshold
    }
}

/// Seeds the board with one random starting tile before the first turn —
/// the same fixed 4-hex, 2-3-distinct-terrain piece the market row offers,
/// placed at the origin. Gives players an immediate foothold of up to 3
/// different ecosystems instead of a fully blank grid.
pub fn seed_starting_terrain(board: &mut Board, rng: &mut impl Rng) {
    use crate::market::{random_terrain_shape, MarketOption};

    let MarketOption::TerrainShape { offsets, terrains, .. } = random_terrain_shape(rng) else {
        unreachable!("random_terrain_shape always returns a TerrainShape option");
    };
    let rotation = rng.gen_range(0..6);
    let origin = Hex::new(0, 0);
    let placements: Vec<(Hex, Terrain)> = offsets
        .iter()
        .zip(terrains.iter())
        .map(|(&o, &t)| {
            let rotated = crate::hex::rotate(o, rotation);
            (Hex::new(origin.q + rotated.q, origin.r + rotated.r), t)
        })
        .collect();
    board.place_terrain_shape(&placements);
}

/// Which of a species' habitat terrains, if any, are currently mature enough
/// to unlock it for placement (brief: "gated by habitat maturity").
pub fn is_species_unlocked(board: &Board, edges: &[FoodWebEdge], species: Species) -> bool {
    use crate::balance::{HABITAT_THRESHOLD_APEX, HABITAT_THRESHOLD_BASE, HABITAT_THRESHOLD_MID};
    use crate::species::Tier;

    let threshold = match species::tier(edges, species) {
        Tier::Apex => HABITAT_THRESHOLD_APEX,
        Tier::Mid => HABITAT_THRESHOLD_MID,
        Tier::Base => HABITAT_THRESHOLD_BASE,
    };
    species::species_terrains(edges, species)
        .into_iter()
        .any(|t| board.terrain_meets_threshold(t, threshold))
}
