//! Unified per-tile-colony growth/starvation/spillover pass. Adjacent
//! same-species tiles are one colony sharing a single counter
//! (`board::animal_colonies`); colonies have no persistent identity, so
//! merges and splits just fall out of recomputing connected components
//! fresh every pass.
//!
//! Growth is Fibonacci-shaped: a colony can gain its own *previous*
//! population as new growth each pass (`next = current + previous`), but
//! that growth potential is scaled down by how crowded/threatened its
//! border is, and direct losses from contention, predation, or lack of
//! prey subtract off on top — so a colony in ideal conditions genuinely
//! runs the Fibonacci sequence, while a crowded or heavily-predated one
//! dampens toward flat or outright declines.

use crate::balance::{
    COLONY_SPILLOVER_THRESHOLD_PER_TILE, COLONY_STARVATION_THRESHOLD, GROWTH_RATE_CAP,
    PREDATOR_FALL_RATE_AT_ZERO_PREY, PREDATOR_FULL_GROWTH_PREY_COUNT, PREY_CONTENTION_PENALTY,
    PREY_PREDATOR_SUPPRESSION,
};
use crate::board::{Board, Colony, Direction};
use crate::hex::Hex;
use crate::species::{self, FoodWebEdge, Species, Tier};
use crate::terrain::Terrain;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct GrowthReport {
    pub spillovers: HashMap<Species, usize>,
    pub starvations: HashMap<Species, usize>,
}

/// Every in-bounds neighbor of any colony member tile that isn't itself a
/// member — the colony's shared border.
fn colony_border(board: &Board, tiles: &[Hex]) -> Vec<Hex> {
    let members: HashSet<Hex> = tiles.iter().copied().collect();
    let mut border: HashSet<Hex> = HashSet::new();
    for &hex in tiles {
        for n in board.neighbors_in_bounds(&hex) {
            if !members.contains(&n) {
                border.insert(n);
            }
        }
    }
    border.into_iter().collect()
}

/// The raw counts a colony's rate is built from — surfaced (not just the
/// blended rate number) so a player can see *why* a colony is growing or
/// shrinking: how much open space it has, how many predators or competing
/// prey border it, how much prey a predator colony can reach.
#[derive(Debug, Clone, Copy, Default)]
pub struct ColonyFactors {
    /// Empty adjacent hexes whose terrain matches one the colony already
    /// occupies — an empty hex of a terrain this species can't settle on
    /// doesn't count as room to grow into.
    pub open_adjacent: u32,
    pub contending_adjacent: u32,
    pub predator_adjacent: u32,
    pub prey_adjacent: u32,
}

/// One colony's border factors and the population change (`rate`) they
/// produce this pass, given its current `previous_counter` (the Fibonacci
/// term). A Mid-tier species combines both components (it's simultaneously
/// prey-role and predator-role within the same terrain); Apex only ever
/// computes the predator component, Base only ever the prey component.
fn colony_factors_and_rate(
    board: &Board,
    edges: &[FoodWebEdge],
    colony: &Colony,
    border: &[Hex],
    previous_counter: f32,
) -> (ColonyFactors, f32) {
    let tier = species::tier(edges, colony.species);
    let mut factors = ColonyFactors::default();
    let growth_potential = previous_counter.max(0.0);
    let mut rate = 0.0f32;
    let habitat_terrains: HashSet<Terrain> =
        colony.tiles.iter().filter_map(|h| board.terrain.get(h).copied()).collect();

    if tier != Tier::Apex {
        for &b in border {
            match board.animals.get(&b) {
                None => {
                    if board.terrain.get(&b).map(|t| habitat_terrains.contains(t)).unwrap_or(false) {
                        factors.open_adjacent += 1;
                    }
                }
                Some(&occupant) => {
                    let Some(&terrain) = board.terrain.get(&b) else {
                        continue;
                    };
                    let predators = species::predators_of(edges, terrain, colony.species);
                    let prey = species::prey_of(edges, terrain, colony.species);
                    if predators.contains(&occupant) {
                        factors.predator_adjacent += 1;
                    } else if prey.contains(&occupant) {
                        // Own food, not a competitor — counted as
                        // `prey_adjacent` by the predator-role block below.
                    } else if species::tier(edges, occupant) != Tier::Apex {
                        factors.contending_adjacent += 1;
                    }
                }
            }
        }
        let threat = (factors.contending_adjacent + factors.predator_adjacent) as f32;
        let total = factors.open_adjacent as f32 + threat;
        let room_factor = if total > 0.0 { (factors.open_adjacent as f32 / total).clamp(0.0, 1.0) } else { 0.0 };
        let losses = factors.contending_adjacent as f32 * PREY_CONTENTION_PENALTY
            + factors.predator_adjacent as f32 * PREY_PREDATOR_SUPPRESSION;
        rate += growth_potential * room_factor - losses;
    }

    if tier != Tier::Base {
        for &b in border {
            if let Some(&occupant) = board.animals.get(&b) {
                let Some(&terrain) = board.terrain.get(&b) else {
                    continue;
                };
                if species::prey_of(edges, terrain, colony.species).contains(&occupant) {
                    factors.prey_adjacent += 1;
                }
            }
        }
        let prey_factor = (factors.prey_adjacent as f32 / PREDATOR_FULL_GROWTH_PREY_COUNT as f32).clamp(0.0, 1.0);
        let zero_prey_penalty = if factors.prey_adjacent == 0 { PREDATOR_FALL_RATE_AT_ZERO_PREY } else { 0.0 };
        rate += growth_potential * prey_factor + zero_prey_penalty;
    }

    (factors, rate.clamp(-GROWTH_RATE_CAP, GROWTH_RATE_CAP))
}

/// This colony's spillover threshold — scales with how many tiles it
/// already has, so a bigger colony needs proportionally more population to
/// justify spreading further rather than a flat number regardless of size.
pub fn colony_spillover_threshold(colony_size: usize) -> f32 {
    COLONY_SPILLOVER_THRESHOLD_PER_TILE * colony_size.max(1) as f32
}

/// This colony's current factors and the population change a growth pass
/// would apply right now — used to preview info (e.g. a hover tooltip)
/// without waiting for or mutating anything.
pub fn colony_preview(board: &Board, edges: &[FoodWebEdge], colony: &Colony) -> (ColonyFactors, f32) {
    let border = colony_border(board, &colony.tiles);
    let previous = board.colony_previous_counter(&colony.tiles);
    colony_factors_and_rate(board, edges, colony, &border, previous)
}

struct Outcome {
    species: Species,
    tiles: Vec<Hex>,
    new_counter: f32,
    new_previous: f32,
    direction: Direction,
    spill_target: Option<Hex>,
    starve_tile: Option<Hex>,
}

/// Runs one growth pass: advances every colony's population (Fibonacci-
/// shaped, dampened/reversed by crowding and predation), spills over one
/// tile per pass once population is at/above `colony_spillover_threshold`,
/// starves one tile per pass at/below `COLONY_STARVATION_THRESHOLD`.
pub fn run_growth_pass(board: &mut Board, edges: &[FoodWebEdge], rng: &mut impl Rng) -> GrowthReport {
    let colonies = board.animal_colonies();
    let mut report = GrowthReport::default();

    // Compute against the pre-pass board first; nothing is mutated here, so
    // colonies can't see each other's changes mid-pass.
    let mut outcomes = Vec::with_capacity(colonies.len());
    for colony in &colonies {
        let border = colony_border(board, &colony.tiles);
        let current = board.colony_counter(&colony.tiles);
        let previous = board.colony_previous_counter(&colony.tiles);
        let (_, rate) = colony_factors_and_rate(board, edges, colony, &border, previous);

        let new_counter = (current + rate).round();
        let new_previous = current.round();
        let direction = if rate > 0.01 {
            Direction::Rising
        } else if rate < -0.01 {
            Direction::Falling
        } else {
            Direction::Flat
        };

        let threshold = colony_spillover_threshold(colony.tiles.len());
        let spill_target = if new_counter >= threshold {
            let habitat_terrains: HashSet<Terrain> =
                colony.tiles.iter().filter_map(|h| board.terrain.get(h).copied()).collect();
            let mut candidates: Vec<Hex> = border
                .iter()
                .copied()
                .filter(|b| {
                    !board.animals.contains_key(b)
                        && board.terrain.get(b).map(|t| habitat_terrains.contains(t)).unwrap_or(false)
                })
                .collect();
            candidates.shuffle(rng);
            candidates.into_iter().next()
        } else {
            None
        };

        let starve_tile = if new_counter <= COLONY_STARVATION_THRESHOLD {
            let mut members = colony.tiles.clone();
            members.shuffle(rng);
            members.into_iter().next()
        } else {
            None
        };

        outcomes.push(Outcome {
            species: colony.species,
            tiles: colony.tiles.clone(),
            new_counter,
            new_previous,
            direction,
            spill_target,
            starve_tile,
        });
    }

    // Apply. Spill targets are deduped here — two colonies could have
    // independently picked the same empty border hex; first wins, the
    // other just tries again next pass.
    let mut claimed_spill_targets: HashSet<Hex> = HashSet::new();
    for outcome in outcomes {
        if let Some(starve_hex) = outcome.starve_tile {
            board.remove_animal(&starve_hex);
            *report.starvations.entry(outcome.species).or_insert(0) += 1;
        }
        let surviving_tiles: Vec<Hex> =
            outcome.tiles.into_iter().filter(|h| Some(*h) != outcome.starve_tile).collect();
        if !surviving_tiles.is_empty() {
            board.set_colony_state(&surviving_tiles, outcome.new_counter, outcome.new_previous, outcome.direction);
        }
        if let Some(spill_hex) = outcome.spill_target {
            if claimed_spill_targets.insert(spill_hex) {
                // Direct insert, not `Board::place_animal` — that would
                // re-run eligibility checks against a board already
                // mutated by this phase.
                board.animals.insert(spill_hex, outcome.species);
                board.animal_counters.insert(spill_hex, outcome.new_counter);
                board.animal_previous_counters.insert(spill_hex, outcome.new_previous);
                board.animal_directions.insert(spill_hex, outcome.direction);
                *report.spillovers.entry(outcome.species).or_insert(0) += 1;
            }
        }
    }

    report
}
