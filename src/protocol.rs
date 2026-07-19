//! Wire messages between browser client and server. Every outgoing `State`
//! snapshot is built per-recipient (`StateSnapshot::for_player`) so a
//! player's own secret objective is included but everyone else's stays off
//! the wire entirely — the server is the sole holder of that information,
//! per the brief's core design constraint.

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

#[derive(Debug, Clone, Serialize)]
pub struct StateSnapshot {
    pub phase: GamePhase,
    pub players: Vec<PlayerSummary>,
    pub current_player: Option<PlayerId>,
    pub turns_taken: u32,
    pub total_turns: u32,
    pub board_radius: i32,
    pub terrain: Vec<(Hex, Terrain)>,
    pub animals: Vec<(Hex, Species)>,
    pub market_row: Vec<MarketOption>,
    pub my_objective: Option<SecretObjective>,
    pub last_growth: Option<Vec<(Species, usize)>>,
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
            animals: session.board.animals.iter().map(|(h, s)| (*h, *s)).collect(),
            market_row: session.market_row.clone(),
            my_objective: session.my_objective(player_id),
            last_growth: session
                .last_growth
                .as_ref()
                .map(|r| r.spawned.iter().map(|(s, n)| (*s, *n)).collect()),
        }
    }
}
