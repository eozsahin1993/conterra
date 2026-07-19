//! The turn-by-turn market row: 4 choices blending procedurally-generated
//! terrain shapes with any currently-unlocked animal placements (brief:
//! "RESOLVED: Mixed ... never switches to animal-only"), plus a shuffle
//! option to discard all 4 for a fresh set.

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

/// Generates a fresh 4-option market row. Roughly 40% of slots offer an
/// unlocked animal placement when any exist; the rest are terrain shapes.
/// One slot is reserved as a guaranteed terrain shape so the row can never
/// end up all-animal, per the brief's resolved market-row design ("never
/// switches to animal-only") — four independent 40% coin flips would
/// otherwise land on all-animal a small but real fraction of the time,
/// which gets more likely as more species unlock over a game.
pub fn generate_market_row(board: &Board, edges: &[FoodWebEdge], rng: &mut impl Rng) -> Vec<MarketOption> {
    let unlocked = currently_unlocked_species(board, edges);
    let guaranteed_terrain_slot = rng.gen_range(0..MARKET_ROW_SIZE);
    (0..MARKET_ROW_SIZE)
        .map(|slot| {
            if slot != guaranteed_terrain_slot && !unlocked.is_empty() && rng.gen_bool(0.4) {
                let species = *unlocked.choose(rng).unwrap();
                MarketOption::AnimalPlacement {
                    id: Uuid::new_v4(),
                    species,
                }
            } else {
                random_terrain_shape(rng)
            }
        })
        .collect()
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
