//! The turn-by-turn market: two independent rows of up to 4 choices each —
//! a terrain-shape row (always populated) and an animal-placement row
//! (populated once at least one species is unlocked, empty before that),
//! plus a shuffle option that discards both rows for a fresh set.

use crate::balance::{MARKET_ROW_SIZE, TERRAIN_SHAPE_MAX_DISTINCT, TERRAIN_SHAPE_MIN_DISTINCT, TERRAIN_SHAPE_SIZE};
use crate::board::{is_species_unlocked, Board};
use crate::hex::{grow_region, Hex};
use crate::species::{self, FoodWebEdge, Species};
use crate::terrain::Terrain;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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
/// terrains. Each terrain is grown as its own connected region (chained
/// onto the previous ones, seeded from their border) rather than colored
/// in after the fact — guarantees every terrain's hexes are contiguous
/// and the whole piece stays connected, which a post-hoc coloring pass
/// can't guarantee for all shape topologies (e.g. a hex with three
/// mutually non-adjacent neighbors has no valid 2-coloring at all).
pub fn random_terrain_shape(rng: &mut impl Rng) -> MarketOption {
    let num_distinct = rng.gen_range(TERRAIN_SHAPE_MIN_DISTINCT..=TERRAIN_SHAPE_MAX_DISTINCT);
    let mut chosen: Vec<Terrain> = Terrain::ALL.to_vec();
    chosen.shuffle(rng);
    chosen.truncate(num_distinct);
    let sizes = random_partition_sizes(TERRAIN_SHAPE_SIZE, num_distinct, rng);

    let mut offsets: Vec<Hex> = Vec::new();
    let mut terrains: Vec<Terrain> = Vec::new();
    let mut occupied: HashSet<Hex> = HashSet::new();

    for (&terrain, &size) in chosen.iter().zip(sizes.iter()) {
        let seed = if offsets.is_empty() {
            Hex::new(0, 0)
        } else {
            let candidates: Vec<Hex> = offsets
                .iter()
                .flat_map(|h| h.neighbors())
                .filter(|n| !occupied.contains(n))
                .collect();
            *candidates
                .choose(rng)
                .expect("a finite shape on an unbounded hex plane always has an open neighbor")
        };
        let region = grow_region(seed, size, &occupied, rng);
        occupied.extend(region.iter().copied());
        terrains.extend(std::iter::repeat(terrain).take(region.len()));
        offsets.extend(region);
    }

    MarketOption::TerrainShape {
        id: Uuid::new_v4(),
        offsets,
        terrains,
    }
}

/// `parts` positive integers summing to `total`, in random order —
/// e.g. (4, 2) might give [1,3], [2,2], or [3,1].
fn random_partition_sizes(total: usize, parts: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut sizes = vec![1usize; parts];
    let mut remaining = total - parts;
    while remaining > 0 {
        let i = rng.gen_range(0..parts);
        sizes[i] += 1;
        remaining -= 1;
    }
    sizes.shuffle(rng);
    sizes
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
/// `origin`) is in-bounds and currently bare, and that at least
/// `PLACEMENT_MIN_MATCHING_SEAMS` of its hexes are "seam-safe" — touching no
/// existing terrain, or matching whatever they do touch. Not all 4 hexes
/// need to be seam-safe: the piece's internal terrain layout isn't chosen
/// with the board in mind, so requiring a perfect match everywhere made
/// placement nearly impossible once the board had any real shape. Mixing
/// within the new piece itself was never restricted (that's the piece's own
/// design) and still isn't.
pub fn can_place_shape(board: &Board, origin: Hex, offsets: &[Hex], terrains: &[Terrain], rotation: i32) -> bool {
    let placements: Vec<(Hex, Terrain)> = offsets
        .iter()
        .zip(terrains.iter())
        .map(|(o, t)| {
            let rotated = crate::hex::rotate(*o, rotation);
            (Hex::new(origin.q + rotated.q, origin.r + rotated.r), *t)
        })
        .collect();
    let new_hexes: HashSet<Hex> = placements.iter().map(|(h, _)| *h).collect();

    if !placements.iter().all(|(hex, _)| board.is_empty(hex)) {
        return false;
    }

    let seam_safe_count = placements
        .iter()
        .filter(|(hex, terrain)| {
            hex.neighbors().iter().all(|n| {
                new_hexes.contains(n) || board.terrain.get(n).map(|existing| existing == terrain).unwrap_or(true)
            })
        })
        .count();

    seam_safe_count >= crate::balance::PLACEMENT_MIN_MATCHING_SEAMS.min(placements.len())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn is_contiguous(hexes: &[Hex]) -> bool {
        if hexes.is_empty() {
            return true;
        }
        let set: HashSet<Hex> = hexes.iter().copied().collect();
        let mut seen = HashSet::new();
        let mut stack = vec![hexes[0]];
        seen.insert(hexes[0]);
        while let Some(h) = stack.pop() {
            for n in h.neighbors() {
                if set.contains(&n) && seen.insert(n) {
                    stack.push(n);
                }
            }
        }
        seen.len() == set.len()
    }

    #[test]
    fn generated_pieces_are_fully_connected_and_terrain_regions_contiguous() {
        let mut rng = rand::thread_rng();
        for _ in 0..5000 {
            let MarketOption::TerrainShape { offsets, terrains, .. } = random_terrain_shape(&mut rng) else {
                unreachable!()
            };
            assert!(is_contiguous(&offsets), "whole piece not connected: {offsets:?}");

            let mut by_terrain: HashMap<Terrain, Vec<Hex>> = HashMap::new();
            for (o, t) in offsets.iter().zip(terrains.iter()) {
                by_terrain.entry(*t).or_default().push(*o);
            }
            for (terrain, hexes) in &by_terrain {
                assert!(
                    is_contiguous(hexes),
                    "non-contiguous {terrain:?} region: {hexes:?} in piece {:?}",
                    offsets.iter().zip(terrains.iter()).collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn placement_rejects_mismatched_seam_but_allows_matching_seam() {
        let mut board = Board::new(4);
        // One pre-existing Forest tile at the origin.
        board.place_terrain_shape(&[(Hex::new(0, 0), Terrain::Forest)]);

        // A single-hex "shape" placed directly adjacent to it.
        let offsets = vec![Hex::new(0, 0)];
        let touching_origin = Hex::new(1, 0); // neighbor of (0,0)

        assert!(
            !can_place_shape(&board, touching_origin, &offsets, &[Terrain::River], 0),
            "mismatched terrain touching an existing tile should be rejected"
        );
        assert!(
            can_place_shape(&board, touching_origin, &offsets, &[Terrain::Forest], 0),
            "matching terrain touching an existing tile should be allowed"
        );

        // Far away (but still in-bounds), touching nothing existing — any
        // terrain is fine.
        let far_away = Hex::new(3, 0);
        assert!(can_place_shape(&board, far_away, &offsets, &[Terrain::River], 0));
    }

    #[test]
    fn placement_allows_internal_mixing_within_the_new_piece() {
        let board = Board::new(4);
        // A 2-hex piece with two different terrains touching each other —
        // this is the new piece's own internal seam, not against existing
        // board terrain, so it must stay allowed.
        let offsets = vec![Hex::new(0, 0), Hex::new(1, 0)];
        let terrains = vec![Terrain::Forest, Terrain::River];
        assert!(can_place_shape(&board, Hex::new(0, 0), &offsets, &terrains, 0));
    }

    #[test]
    fn placement_allows_a_piece_that_meets_the_minimum_matching_seams_but_not_less() {
        let mut board = Board::new(10);
        // Piece hexes spaced far enough apart (mutual distance >= 5) that
        // none of them accidentally share a neighbor with each other.
        let offsets = vec![Hex::new(0, 0), Hex::new(5, 0), Hex::new(0, 5), Hex::new(5, 5)];
        let terrains = vec![Terrain::Forest; 4];

        // Two pre-existing River tiles, each adjacent to a different one of
        // the incoming (all-Forest) piece's hexes — two real mismatches.
        board.place_terrain_shape(&[(Hex::new(1, 0), Terrain::River), (Hex::new(6, 0), Terrain::River)]);

        // 2 of the 4 hexes touch a mismatch, the other 2 ((0,5) and (5,5))
        // touch nothing existing — exactly PLACEMENT_MIN_MATCHING_SEAMS (2)
        // are seam-safe, so this should be allowed.
        assert!(can_place_shape(&board, Hex::new(0, 0), &offsets, &terrains, 0));

        // Add a third mismatch, adjacent to (0,5): now only 1 of the 4
        // hexes ((5,5)) is seam-safe, below the minimum — should be rejected.
        board.place_terrain_shape(&[(Hex::new(0, 4), Terrain::River)]);
        assert!(!can_place_shape(&board, Hex::new(0, 0), &offsets, &terrains, 0));
    }
}
