// Mirrors src/protocol.rs, src/game.rs, src/market.rs, src/objectives.rs on
// the Rust side. Enum wire format is `{ "type": "Variant", ...fields }` for
// every tagged enum (see the `#[serde(tag = "type")]` attributes there).

export type Terrain = "Forest" | "River" | "Ocean" | "Savanna" | "Mountain";

// Wire values are the Rust enum variant identifiers (PascalCase, no spaces),
// e.g. "GreatWhiteShark" — see `prettySpecies` in format.ts for display text.
export type Species = string;

export interface Hex {
  q: number;
  r: number;
}

export type GamePhase = "Lobby" | "InProgress" | "Ended";

export type Direction = "Rising" | "Flat" | "Falling";

// One animal-occupied tile. Adjacent same-species tiles form one "colony"
// sharing a single counter/direction/size (src/growth.rs).
export interface AnimalTileInfo {
  hex: Hex;
  species: Species;
  counter: number;
  direction: Direction;
  colony_size: number;
}

export type MarketOption =
  // `terrains` is parallel to `offsets` (same index = same hex) — a piece
  // mixes 2-3 distinct terrain types across its fixed 4 hexes, never one
  // uniform terrain.
  | { type: "TerrainShape"; id: string; offsets: Hex[]; terrains: Terrain[] }
  | { type: "AnimalPlacement"; id: string; species: Species };

export type SecretObjective =
  | { type: "PopulationTarget"; species: Species; target: number }
  | { type: "AdjacencyGoal"; species: Species; distinct_terrains: number };

export interface PlayerSummary {
  id: string;
  name: string;
}

export interface StateSnapshot {
  phase: GamePhase;
  players: PlayerSummary[];
  current_player: string | null;
  turns_taken: number;
  total_turns: number;
  board_radius: number;
  terrain: [Hex, Terrain][];
  animals: AnimalTileInfo[];
  terrain_row: MarketOption[];
  animal_row: MarketOption[];
  my_objective: SecretObjective | null;
  last_spillover: [Species, number][] | null;
  last_starvation: [Species, number][] | null;
  colony_spillover_threshold: number;
  colony_starvation_threshold: number;
}

export interface PlayerResult {
  player_id: string;
  name: string;
  objective: SecretObjective;
  met: boolean;
  score: number;
}

export interface GameResult {
  group_threshold_met: boolean;
  total_population: number;
  longest_corridor: number;
  players: PlayerResult[];
  winners: string[];
}

export type PlacementInput =
  | { type: "Terrain"; origin: Hex; rotation: number }
  | { type: "Animal"; hex: Hex };

export type ClientMessage =
  | { type: "Join"; name: string }
  | { type: "Start" }
  | { type: "Select"; option_id: string; placement: PlacementInput }
  | { type: "Shuffle" };

export type ServerMessage =
  | { type: "Joined"; player_id: string }
  | { type: "State"; snapshot: StateSnapshot }
  | { type: "Result"; result: GameResult }
  | { type: "Error"; message: string };
