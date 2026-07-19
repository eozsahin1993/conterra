//! The turn-by-turn market: two independent rows of up to 4 choices each —
//! a terrain-shape row (always populated) and an animal-placement row
//! (populated once at least one species is unlocked, empty before that),
//! plus a shuffle option that discards both rows for a fresh set.

use crate::balance::{MARKET_ROW_SIZE, SHAPE_SIZE_MAX, SHAPE_SIZE_MIN};
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
        terrain: Terrain,
        /// Offsets relative to a seed hex at (0,0), pre-rotation.
        offsets: Vec<Hex>,
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

fn random_terrain_shape(rng: &mut impl Rng) -> MarketOption {
    let terrain = *Terrain::ALL.choose(rng).unwrap();
    let size = rng.gen_range(SHAPE_SIZE_MIN..=SHAPE_SIZE_MAX);
    let offsets = grow_shape(size, rng);
    MarketOption::TerrainShape {
        id: Uuid::new_v4(),
        terrain,
        offsets,
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
