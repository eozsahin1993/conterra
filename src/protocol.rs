//! Wire messages between browser client and server. Every outgoing `State`
//! snapshot is built per-recipient (`StateSnapshot::for_player`) so a
//! player's own secret objective is included but everyone else's stays off
//! the wire entirely.

use crate::balance::{COLONY_SPILLOVER_THRESHOLD, COLONY_STARVATION_THRESHOLD};
use crate::board::Direction;
use crate::game::{GamePhase, GameResult, GameSession, PlacementInput, PlayerId};
use crate::hex::Hex;
use crate::market::MarketOption;
use crate::objectives::SecretObjective;
use crate::species::Species;
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Join { name: String },
    Start,
    Select { option_id: Uuid, placement: PlacementInput },
    Shuffle,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Joined { player_id: PlayerId },
    State { snapshot: StateSnapshot },
    Result { result: GameResult },
    Error { message: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerSummary {
    pub id: PlayerId,
    pub name: String,
}

/// One animal-occupied tile, including its colony's shared counter,
/// trajectory, and current size (see `board::animal_colonies`) — every
/// tile in a colony reports the same counter/direction/size.
#[derive(Debug, Clone, Serialize)]
pub struct AnimalTileInfo {
    pub hex: Hex,
    pub species: Species,
    pub counter: f32,
    pub direction: Direction,
    pub colony_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct StateSnapshot {
    pub phase: GamePhase,
    pub players: Vec<PlayerSummary>,
    pub current_player: Option<PlayerId>,
    pub turns_taken: u32,
    pub total_turns: u32,
    pub board_radius: i32,
    pub terrain: Vec<(Hex, Terrain)>,
    pub animals: Vec<AnimalTileInfo>,
    pub terrain_row: Vec<MarketOption>,
    pub animal_row: Vec<MarketOption>,
    pub my_objective: Option<SecretObjective>,
    pub last_spillover: Option<Vec<(Species, usize)>>,
    pub last_starvation: Option<Vec<(Species, usize)>>,
    /// Sent so the frontend's distance-to-threshold hint always matches
    /// whatever these are actually tuned to server-side.
    pub colony_spillover_threshold: f32,
    pub colony_starvation_threshold: f32,
}

impl StateSnapshot {
    pub fn for_player(session: &GameSession, player_id: PlayerId) -> Self {
        StateSnapshot {
            phase: session.phase,
            players: session
                .players
                .iter()
                .map(|p| PlayerSummary {
                    id: p.id,
                    name: p.name.clone(),
                })
                .collect(),
            current_player: session.current_player(),
            turns_taken: session.turns_taken,
            total_turns: session.total_turns,
            board_radius: session.board.radius,
            terrain: session.board.terrain.iter().map(|(h, t)| (*h, *t)).collect(),
            animals: {
                // Recomputed fresh (not read off `last_growth`) so colony_size
                // is accurate even before the next growth pass runs.
                let mut colony_size_by_hex: std::collections::HashMap<Hex, usize> = std::collections::HashMap::new();
                for colony in session.board.animal_colonies() {
                    for h in &colony.tiles {
                        colony_size_by_hex.insert(*h, colony.tiles.len());
                    }
                }
                session
                    .board
                    .animals
                    .iter()
                    .map(|(h, s)| AnimalTileInfo {
                        hex: *h,
                        species: *s,
                        counter: session.board.animal_counters.get(h).copied().unwrap_or(0.0),
                        direction: session.board.animal_directions.get(h).copied().unwrap_or(Direction::Flat),
                        colony_size: colony_size_by_hex.get(h).copied().unwrap_or(1),
                    })
                    .collect()
            },
            terrain_row: session.terrain_row.clone(),
            animal_row: session.animal_row.clone(),
            my_objective: session.my_objective(player_id),
            last_spillover: session
                .last_growth
                .as_ref()
                .map(|r| r.spillovers.iter().map(|(s, n)| (*s, *n)).collect()),
            last_starvation: session
                .last_growth
                .as_ref()
                .map(|r| r.starvations.iter().map(|(s, n)| (*s, *n)).collect()),
            colony_spillover_threshold: COLONY_SPILLOVER_THRESHOLD,
            colony_starvation_threshold: COLONY_STARVATION_THRESHOLD,
        }
    }
}
