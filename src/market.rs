//! The turn-by-turn market: two independent rows of up to 4 choices each —
//! a terrain-shape row (always populated) and an animal-placement row
//! (populated once at least one species is unlocked, empty before that),
//! plus a shuffle option that discards both rows for a fresh set.

use crate::balance::{MARKET_ROW_SIZE, TERRAIN_SHAPE_MAX_DISTINCT, TERRAIN_SHAPE_MIN_DISTINCT, TERRAIN_SHAPE_SIZE};
use crate::board::{is_species_unlocked, Board};
use crate::hex::{grow_shape, Hex};
use crate::species::{self, FoodWebEdge, Species};
use crate::terrain::Terrain;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MarketOption {
    TerrainShape {
        id: Uuid,
        /// Offsets relative to a seed hex at (0,0), pre-rotation.
        offsets: Vec<Hex>,
        /// Per-hex terrain, parallel to `offsets` (same index = same hex).
        /// Always TERRAIN_SHAPE_SIZE long, mixing 2-3 distinct terrain
        /// types across the piece — never one uniform terrain.
        terrains: Vec<Terrain>,
    },
    AnimalPlacement {
        id: Uuid,
        species: Species,
    },
}

impl MarketOption {
    pub fn id(&self) -> Uuid {
        match self {
            MarketOption::TerrainShape { id, .. } => *id,
            MarketOption::AnimalPlacement { id, .. } => *id,
        }
    }
}

/// Generates one fixed-size terrain-shape piece mixing 2-3 distinct
/// terrains — each chosen terrain is guaranteed >=1 hex, the rest filled
/// randomly from the same set.
pub fn random_terrain_shape(rng: &mut impl Rng) -> MarketOption {
    let offsets = grow_shape(TERRAIN_SHAPE_SIZE, rng);

    let num_distinct = rng.gen_range(TERRAIN_SHAPE_MIN_DISTINCT..=TERRAIN_SHAPE_MAX_DISTINCT);
    let mut chosen: Vec<Terrain> = Terrain::ALL.to_vec();
    chosen.shuffle(rng);
    chosen.truncate(num_distinct);

    // Guarantee every chosen terrain covers >=1 hex, then fill the rest
    // randomly from the same set, then shuffle so coverage isn't always
    // on the first `num_distinct` offsets.
    let mut terrains: Vec<Terrain> = chosen.clone();
    while terrains.len() < offsets.len() {
        terrains.push(*chosen.choose(rng).unwrap());
    }
    terrains.shuffle(rng);

    MarketOption::TerrainShape {
        id: Uuid::new_v4(),
        offsets,
        terrains,
    }
}

fn currently_unlocked_species(board: &Board, edges: &[FoodWebEdge]) -> Vec<Species> {
    Species::ALL
        .iter()
        .copied()
        .filter(|&s| is_species_unlocked(board, edges, s))
        .collect()
}

/// The terrain-shape row: always exactly `MARKET_ROW_SIZE` procedurally
/// generated shapes.
pub fn generate_terrain_row(rng: &mut impl Rng) -> Vec<MarketOption> {
    (0..MARKET_ROW_SIZE).map(|_| random_terrain_shape(rng)).collect()
}

/// The animal-placement row: up to `MARKET_ROW_SIZE` unlocked species,
/// sampled with replacement (a single unlocked species can fill more than
/// one slot). Empty until at least one species is unlocked.
pub fn generate_animal_row(board: &Board, edges: &[FoodWebEdge], rng: &mut impl Rng) -> Vec<MarketOption> {
    let unlocked = currently_unlocked_species(board, edges);
    if unlocked.is_empty() {
        return Vec::new();
    }
    (0..MARKET_ROW_SIZE)
        .map(|_| {
            let species = *unlocked.choose(rng).unwrap();
            MarketOption::AnimalPlacement {
                id: Uuid::new_v4(),
                species,
            }
        })
        .collect()
}

/// A single fresh terrain-shape option, used to refill one consumed slot in
/// the terrain row without regenerating the whole row.
pub fn generate_one_terrain_option(rng: &mut impl Rng) -> MarketOption {
    random_terrain_shape(rng)
}

/// A single fresh animal-placement option, used to refill one consumed slot
/// in the animal row. Returns `None` if nothing is unlocked yet (the slot
/// is then simply dropped rather than refilled).
pub fn generate_one_animal_option(board: &Board, edges: &[FoodWebEdge], rng: &mut impl Rng) -> Option<MarketOption> {
    let unlocked = currently_unlocked_species(board, edges);
    let species = *unlocked.choose(rng)?;
    Some(MarketOption::AnimalPlacement {
        id: Uuid::new_v4(),
        species,
    })
}

/// Checks every hex a shape would occupy (after rotation + translation to
/// `origin`) is in-bounds and currently bare (no terrain placed yet).
pub fn can_place_shape(board: &Board, origin: Hex, offsets: &[Hex], rotation: i32) -> bool {
    offsets.iter().all(|o| {
        let rotated = crate::hex::rotate(*o, rotation);
        let hex = Hex::new(origin.q + rotated.q, origin.r + rotated.r);
        board.is_empty(&hex)
    })
}

pub fn rotated_translated(offsets: &[Hex], origin: Hex, rotation: i32) -> Vec<Hex> {
    offsets
        .iter()
        .map(|o| {
            let rotated = crate::hex::rotate(*o, rotation);
            Hex::new(origin.q + rotated.q, origin.r + rotated.r)
        })
        .collect()
}

pub fn species_edges() -> Vec<FoodWebEdge> {
    species::food_web()
}
