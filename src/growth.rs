//! Role-based, fully stateless growth pass (brief: "computed statelessly —
//! recomputed fresh from the current board state every growth/scoring pass,
//! no per-relationship duration or history tracked anywhere"). Every call
//! looks only at the current board; nothing here persists between passes
//! except the resulting token placements themselves.

use crate::balance::{
    PREDATOR_GROWTH_CAP, PREDATOR_GROWTH_PER_ADJACENT_PREY, PREY_CONTENTION_PENALTY,
    PREY_GROWTH_PER_OPEN_ADJACENT, PREY_PREDATOR_SUPPRESSION,
};
use crate::board::Board;
use crate::species::{self, FoodWebEdge, Species, Tier};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};

/// Per-token growth score for one existing animal, evaluated against its
/// current terrain-scoped neighborhood only.
fn token_growth_score(board: &Board, edges: &[FoodWebEdge], hex: crate::hex::Hex, species: Species) -> f32 {
    let terrain = *board
        .terrain
        .get(&hex)
        .expect("animal token always sits on a placed terrain tile");
    let tier = species::tier(edges, species);
    let neighbors = board.neighbors_in_bounds(&hex);

    let mut score = 0.0f32;

    if tier != Tier::Apex {
        // Prey-role component: open space grows it, contending prey and
        // adjacent predators slow it (boom-bust, not flat suppression).
        let predators = species::predators_of(edges, terrain, species);
        let mut n_open = 0u32;
        let mut n_predator = 0u32;
        let mut n_contention = 0u32;
        for n in &neighbors {
            match board.animals.get(n) {
                None => {
                    if board.terrain.contains_key(n) {
                        n_open += 1;
                    }
                }
                Some(&occupant) if occupant == species => {}
                Some(&occupant) if predators.contains(&occupant) => n_predator += 1,
                Some(&occupant) if species::tier(edges, occupant) != Tier::Apex => {
                    // A different, non-apex (i.e. itself prey-role) species
                    // adjacent = competing for the same open space.
                    n_contention += 1;
                }
                Some(_) => {}
            }
        }
        let prey_component = n_open as f32 * PREY_GROWTH_PER_OPEN_ADJACENT
            - n_contention as f32 * PREY_CONTENTION_PENALTY
            - n_predator as f32 * PREY_PREDATOR_SUPPRESSION;
        score += prey_component.max(0.0);
    }

    if tier != Tier::Base {
        let prey = species::prey_of(edges, terrain, species);
        let n_prey_adjacent = neighbors
            .iter()
            .filter(|n| board.animals.get(*n).map(|s| prey.contains(s)).unwrap_or(false))
            .count() as f32;
        let predator_component =
            (n_prey_adjacent * PREDATOR_GROWTH_PER_ADJACENT_PREY).min(PREDATOR_GROWTH_CAP);
        score += predator_component;
    }

    score
}

#[derive(Debug, Default, Clone)]
pub struct GrowthReport {
    pub spawned: HashMap<Species, usize>,
}

/// Runs one growth pass over the whole board: scores every existing token,
/// then spawns new tokens of each species onto eligible adjacent tiles.
pub fn run_growth_pass(board: &mut Board, edges: &[FoodWebEdge], rng: &mut impl Rng) -> GrowthReport {
    let mut tokens_by_species: HashMap<Species, Vec<crate::hex::Hex>> = HashMap::new();
    for (&hex, &sp) in board.animals.iter() {
        tokens_by_species.entry(sp).or_default().push(hex);
    }

    let mut report = GrowthReport::default();
    let mut all_spawns: Vec<(crate::hex::Hex, Species)> = Vec::new();

    for (species, positions) in tokens_by_species.iter() {
        let species = *species;
        let total_score: f32 = positions
            .iter()
            .map(|&hex| token_growth_score(board, edges, hex, species))
            .sum();
        let spawn_budget = total_score.floor() as usize;
        if spawn_budget == 0 {
            continue;
        }

        let habitat_terrains = species::species_terrains(edges, species);
        let mut candidates: HashSet<crate::hex::Hex> = HashSet::new();
        for &hex in positions {
            for n in board.neighbors_in_bounds(&hex) {
                if board.animals.contains_key(&n) {
                    continue;
                }
                if let Some(t) = board.terrain.get(&n) {
                    if habitat_terrains.contains(t) {
                        candidates.insert(n);
                    }
                }
            }
        }

        let mut candidates: Vec<crate::hex::Hex> = candidates.into_iter().collect();
        candidates.shuffle(rng);
        let take = spawn_budget.min(candidates.len());
        for hex in candidates.into_iter().take(take) {
            all_spawns.push((hex, species));
        }
        if take > 0 {
            report.spawned.insert(species, take);
        }
    }

    for (hex, species) in all_spawns {
        board.animals.insert(hex, species);
    }

    report
}
