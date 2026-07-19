//! Secret objectives, dealt privately at game start (brief item 5:
//! "RESOLVED: mixed per-card" — each card is randomly a population target or
//! a positional/adjacency goal). The server is the sole holder of these; see
//! `game::GameSession` for the invisibility-to-other-players enforcement.

use crate::balance::{SECRET_OBJECTIVE_ADJACENT_TERRAIN_COUNT, SECRET_OBJECTIVE_POPULATION_TARGET};
use crate::board::Board;
use crate::scoring::meets_minimum_population;
use crate::species::Species;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SecretObjective {
    PopulationTarget { species: Species, target: u32 },
    AdjacencyGoal { species: Species, distinct_terrains: usize },
}

pub fn deal_objective(rng: &mut impl Rng) -> SecretObjective {
    let species = *Species::ALL.choose(rng).unwrap();
    if rng.gen_bool(0.5) {
        SecretObjective::PopulationTarget {
            species,
            target: SECRET_OBJECTIVE_POPULATION_TARGET,
        }
    } else {
        SecretObjective::AdjacencyGoal {
            species,
            distinct_terrains: SECRET_OBJECTIVE_ADJACENT_TERRAIN_COUNT,
        }
    }
}

pub struct ObjectiveOutcome {
    pub met: bool,
    /// Comparable progress value used to rank players within a successful
    /// group ("only among a successful group does the secret-objective
    /// comparison determine a winner").
    pub score: u32,
}

pub fn evaluate_objective(board: &Board, objective: SecretObjective) -> ObjectiveOutcome {
    match objective {
        SecretObjective::PopulationTarget { species, target } => {
            if !meets_minimum_population(board, species) {
                return ObjectiveOutcome { met: false, score: 0 };
            }
            let count = board.animal_count(species) as u32;
            ObjectiveOutcome {
                met: count >= target,
                score: count,
            }
        }
        SecretObjective::AdjacencyGoal {
            species,
            distinct_terrains,
        } => {
            if !meets_minimum_population(board, species) {
                return ObjectiveOutcome { met: false, score: 0 };
            }
            let mut qualifying = 0u32;
            for hex in board.animal_positions(species) {
                let terrains: HashSet<_> = board
                    .neighbors_in_bounds(&hex)
                    .into_iter()
                    .filter_map(|n| board.terrain.get(&n).copied())
                    .collect();
                if terrains.len() >= distinct_terrains {
                    qualifying += 1;
                }
            }
            ObjectiveOutcome {
                met: qualifying > 0,
                score: qualifying,
            }
        }
    }
}
