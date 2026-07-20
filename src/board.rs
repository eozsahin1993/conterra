use crate::hex::Hex;
use crate::species::{self, FoodWebEdge, Species};
use crate::terrain::Terrain;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A token's current trajectory, persisted so it's visible between growth
/// passes rather than only in the transient per-pass report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Rising,
    Flat,
    Falling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub radius: i32,
    valid_hexes: HashSet<Hex>,
    pub terrain: HashMap<Hex, Terrain>,
    pub animals: HashMap<Hex, Species>,
    /// Persistent per-tile colony counter. Every tile in a colony
    /// (connected component of same-species tiles, see `animal_colonies`)
    /// holds the same value, replicated across members so colonies can
    /// merge/split freely without a separate stable colony id.
    pub animal_counters: HashMap<Hex, f32>,
    pub animal_directions: HashMap<Hex, Direction>,
}

impl Board {
    pub fn new(radius: i32) -> Self {
        let valid_hexes: HashSet<Hex> = Hex::spiral_from_origin(radius).into_iter().collect();
        Board {
            radius,
            valid_hexes,
            terrain: HashMap::new(),
            animals: HashMap::new(),
            animal_counters: HashMap::new(),
            animal_directions: HashMap::new(),
        }
    }

    pub fn in_bounds(&self, hex: &Hex) -> bool {
        self.valid_hexes.contains(hex)
    }

    pub fn is_empty(&self, hex: &Hex) -> bool {
        self.in_bounds(hex) && !self.terrain.contains_key(hex)
    }

    /// Places an already rotated+translated shape, paired with its per-hex
    /// terrain. Caller has already verified legality (`market::can_place_shape`).
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
        // Synced to any adjacent colony's shared value on the next pass.
        self.animal_counters.insert(hex, 0.0);
        self.animal_directions.insert(hex, Direction::Flat);
        Ok(())
    }

    /// Removes a token along with its counter/direction state.
    pub fn remove_animal(&mut self, hex: &Hex) {
        self.animals.remove(hex);
        self.animal_counters.remove(hex);
        self.animal_directions.remove(hex);
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

    /// Groups animal tiles into colonies: connected components of adjacent
    /// same-species tiles, recomputed fresh every call.
    pub fn animal_colonies(&self) -> Vec<Colony> {
        let mut visited: HashSet<Hex> = HashSet::new();
        let mut colonies = Vec::new();
        for (&start, &species) in self.animals.iter() {
            if visited.contains(&start) {
                continue;
            }
            let mut stack = vec![start];
            let mut tiles = Vec::new();
            visited.insert(start);
            while let Some(hex) = stack.pop() {
                tiles.push(hex);
                for n in hex.neighbors() {
                    if self.animals.get(&n) == Some(&species) && !visited.contains(&n) {
                        visited.insert(n);
                        stack.push(n);
                    }
                }
            }
            colonies.push(Colony { species, tiles });
        }
        colonies
    }

    /// The colony's shared counter. Taking the max across member tiles
    /// only matters the one moment two differently-valued colonies just
    /// merged this pass; otherwise every member already agrees. Can be
    /// negative (starving) — never clamp to zero.
    pub fn colony_counter(&self, tiles: &[Hex]) -> f32 {
        tiles
            .iter()
            .filter_map(|h| self.animal_counters.get(h).copied())
            .fold(f32::NEG_INFINITY, f32::max)
    }

    /// Writes the same counter and direction to every member tile.
    pub fn set_colony_state(&mut self, tiles: &[Hex], counter: f32, direction: Direction) {
        for h in tiles {
            self.animal_counters.insert(*h, counter);
            self.animal_directions.insert(*h, direction);
        }
    }
}

pub struct Colony {
    pub species: Species,
    pub tiles: Vec<Hex>,
}

/// Seeds the board with one random starting tile — the same kind of piece
/// the market row offers — placed at the origin.
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
