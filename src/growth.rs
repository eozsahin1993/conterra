//! Unified per-tile-colony growth/starvation/spillover pass. Adjacent
//! same-species tiles are one colony sharing a single counter
//! (`board::animal_colonies`); colonies have no persistent identity, so
//! merges and splits just fall out of recomputing connected components
//! fresh every pass.

use crate::balance::{
    COLONY_SPILLOVER_THRESHOLD, COLONY_STARVATION_THRESHOLD, GROWTH_NONLINEAR_ACCEL_FACTOR,
    GROWTH_RATE_CAP, PREDATOR_FALL_RATE_AT_ZERO_PREY, PREDATOR_MIN_ADJACENT_PREY_THRESHOLD,
    PREDATOR_RISE_RATE_PER_EXCESS_PREY, PREY_CONTENTION_PENALTY, PREY_GROWTH_PER_OPEN_ADJACENT,
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

/// Pressure compounds rather than scaling linearly, capped so one extreme
/// neighborhood can't cause an unbounded single-pass jump.
fn nonlinear_rate(pressure: f32) -> f32 {
    let rate = pressure * (1.0 + GROWTH_NONLINEAR_ACCEL_FACTOR * pressure.abs());
    rate.clamp(-GROWTH_RATE_CAP, GROWTH_RATE_CAP)
}

/// One colony's raw pressure this pass. A Mid-tier species combines both
/// components (it's simultaneously prey-role and predator-role within the
/// same terrain); Apex only ever computes the predator component, Base
/// only ever the prey component.
fn colony_pressure(board: &Board, edges: &[FoodWebEdge], colony: &Colony, border: &[Hex]) -> f32 {
    let tier = species::tier(edges, colony.species);
    let mut pressure = 0.0f32;

    if tier != Tier::Apex {
        let mut n_open = 0u32;
        let mut n_predator = 0u32;
        let mut n_contention = 0u32;
        for &b in border {
            match board.animals.get(&b) {
                None => {
                    if board.terrain.contains_key(&b) {
                        n_open += 1;
                    }
                }
                Some(&occupant) => {
                    let Some(&terrain) = board.terrain.get(&b) else {
                        continue;
                    };
                    let predators = species::predators_of(edges, terrain, colony.species);
                    if predators.contains(&occupant) {
                        n_predator += 1;
                    } else if species::tier(edges, occupant) != Tier::Apex {
                        n_contention += 1;
                    }
                }
            }
        }
        pressure += n_open as f32 * PREY_GROWTH_PER_OPEN_ADJACENT
            - n_contention as f32 * PREY_CONTENTION_PENALTY
            - n_predator as f32 * PREY_PREDATOR_SUPPRESSION;
    }

    if tier != Tier::Base {
        let mut n_prey = 0u32;
        for &b in border {
            if let Some(&occupant) = board.animals.get(&b) {
                let Some(&terrain) = board.terrain.get(&b) else {
                    continue;
                };
                if species::prey_of(edges, terrain, colony.species).contains(&occupant) {
                    n_prey += 1;
                }
            }
        }
        pressure += if n_prey == 0 {
            PREDATOR_FALL_RATE_AT_ZERO_PREY
        } else if n_prey < PREDATOR_MIN_ADJACENT_PREY_THRESHOLD {
            0.0
        } else {
            (n_prey - PREDATOR_MIN_ADJACENT_PREY_THRESHOLD + 1) as f32 * PREDATOR_RISE_RATE_PER_EXCESS_PREY
        };
    }

    pressure
}

struct Outcome {
    species: Species,
    tiles: Vec<Hex>,
    new_counter: f32,
    direction: Direction,
    spill_target: Option<Hex>,
    starve_tile: Option<Hex>,
}

/// Runs one growth pass: advances every colony's counter, spills over one
/// tile per pass at/above threshold, starves one tile per pass at/below
/// threshold.
pub fn run_growth_pass(board: &mut Board, edges: &[FoodWebEdge], rng: &mut impl Rng) -> GrowthReport {
    let colonies = board.animal_colonies();
    let mut report = GrowthReport::default();

    // Compute against the pre-pass board first; nothing is mutated here, so
    // colonies can't see each other's changes mid-pass.
    let mut outcomes = Vec::with_capacity(colonies.len());
    for colony in &colonies {
        let border = colony_border(board, &colony.tiles);
        let pressure = colony_pressure(board, edges, colony, &border);
        let rate = nonlinear_rate(pressure);
        let prev = board.colony_counter(&colony.tiles);
        let new_counter = prev + rate;
        let direction = if rate > 0.01 {
            Direction::Rising
        } else if rate < -0.01 {
            Direction::Falling
        } else {
            Direction::Flat
        };

        let spill_target = if new_counter >= COLONY_SPILLOVER_THRESHOLD {
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
            board.set_colony_state(&surviving_tiles, outcome.new_counter, outcome.direction);
        }
        if let Some(spill_hex) = outcome.spill_target {
            if claimed_spill_targets.insert(spill_hex) {
                // Direct insert, not `Board::place_animal` — that would
                // re-run eligibility checks against a board already
                // mutated by this phase.
                board.animals.insert(spill_hex, outcome.species);
                board.animal_counters.insert(spill_hex, outcome.new_counter);
                board.animal_directions.insert(spill_hex, outcome.direction);
                *report.spillovers.entry(outcome.species).or_insert(0) += 1;
            }
        }
    }

    report
}
