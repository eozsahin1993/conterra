use crate::hex::Hex;
use crate::species::{self, FoodWebEdge, Species};
use crate::terrain::Terrain;
use rand::seq::SliceRandom;
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
    /// rotated+translated absolute hex positions. Caller has already
    /// verified every hex is a legal placement (see `market::can_place_shape`).
    pub fn place_terrain_shape(&mut self, absolute_hexes: &[Hex], terrain: Terrain) {
        for hex in absolute_hexes {
            self.terrain.insert(*hex, terrain);
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

/// Seeds a random starting cluster before the first turn — 1 to
/// `START_CLUSTER_MAX_TERRAINS` distinct terrain patches, grown outward from
/// the origin so they end up touching, giving players an immediate foothold
/// of up to 3 different ecosystems instead of a fully blank grid.
pub fn seed_starting_terrain(board: &mut Board, rng: &mut impl Rng) {
    use crate::balance::{SHAPE_SIZE_MAX, SHAPE_SIZE_MIN, START_CLUSTER_MAX_TERRAINS};
    use crate::hex::grow_shape;

    let num_terrains = rng.gen_range(1..=START_CLUSTER_MAX_TERRAINS);
    let mut terrains: Vec<Terrain> = Terrain::ALL.to_vec();
    terrains.shuffle(rng);
    terrains.truncate(num_terrains);

    let mut frontier: Vec<Hex> = vec![Hex::new(0, 0)];
    for terrain in terrains {
        frontier.retain(|h| board.is_empty(h));
        let seed = if frontier.is_empty() {
            Hex::new(0, 0)
        } else {
            *frontier.choose(rng).unwrap()
        };
        let size = rng.gen_range(SHAPE_SIZE_MIN..=SHAPE_SIZE_MAX);
        let shape = grow_shape(size, rng);
        let absolute: Vec<Hex> = shape
            .iter()
            .map(|o| Hex::new(seed.q + o.q, seed.r + o.r))
            .filter(|h| board.is_empty(h))
            .collect();
        for h in &absolute {
            frontier.extend(board.neighbors_in_bounds(h));
        }
        board.place_terrain_shape(&absolute, terrain);
    }
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
