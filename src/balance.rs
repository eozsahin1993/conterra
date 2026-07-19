//! Every numeric constant here corresponds to an item in the brief's "Open
//! questions still to resolve" or the content doc's "Open / to design later"
//! list — magnitudes the design explicitly deferred until there was something
//! playable to balance against. Picked to be sane defaults, not final values;
//! retune from actual playtests rather than from first principles.

/// Board radius (see `Hex::spiral_from_origin`) — "generous grid size as the
/// primary lever against placement difficulty." Radius 8 is ~217 tiles,
/// generous for a 1-6 player game.
pub const BOARD_RADIUS: i32 = 8;

/// Market row size — fixed by the brief ("4 selections"), not tunable.
pub const MARKET_ROW_SIZE: usize = 4;

/// Every terrain-shape piece — both market-row options and the game's
/// starting tile — is a fixed 4-hex cluster (requirement, not tunable).
pub const TERRAIN_SHAPE_SIZE: usize = 4;

/// Each terrain-shape piece mixes 2-3 distinct terrain types across its 4
/// hexes (requirement) — never a single uniform terrain, and never all 4
/// hexes different. This is also what makes the game's one random starting
/// tile ("up to 3 unique different ecosystems") and a market terrain option
/// the exact same kind of piece.
pub const TERRAIN_SHAPE_MIN_DISTINCT: usize = 2;
pub const TERRAIN_SHAPE_MAX_DISTINCT: usize = 3;

/// Shuffling the market row costs the player their placement action for that
/// turn — simplest possible cost that needed no new resource, kept stateless.
pub const SHUFFLE_COSTS_TURN: bool = true;

/// Habitat maturity thresholds: minimum count of a species' home-terrain
/// tiles already on the board before that species becomes eligible to place.
/// Scaled by tier — apex predators need a more established habitat than
/// base prey. (Brief: "gated by habitat maturity"; content doc: "exact
/// numeric habitat thresholds ... explicitly left untuned.")
pub const HABITAT_THRESHOLD_BASE: usize = 4;
pub const HABITAT_THRESHOLD_MID: usize = 6;
pub const HABITAT_THRESHOLD_APEX: usize = 9;

/// Role-based growth magnitudes (brief: "exact magnitudes ... explicitly
/// untuned"). All growth is stateless — recomputed fresh from board state
/// every pass, nothing here accumulates across passes except the resulting
/// on-map token count.
///
/// Prey growth per unoccupied adjacent hex.
pub const PREY_GROWTH_PER_OPEN_ADJACENT: f32 = 1.0;
/// Prey growth penalty per adjacent contending prey (a different prey
/// species sharing adjacent space).
pub const PREY_CONTENTION_PENALTY: f32 = 0.5;
/// Prey growth penalty per adjacent predator (boom-bust, not flat
/// suppression: this scales with predator count, doesn't just zero growth).
pub const PREY_PREDATOR_SUPPRESSION: f32 = 2.0;
/// Predator growth per adjacent prey token.
pub const PREDATOR_GROWTH_PER_ADJACENT_PREY: f32 = 0.75;
/// Predator growth is capped per pass so a single huge prey glut can't cause
/// an unbounded single-turn jump.
pub const PREDATOR_GROWTH_CAP: f32 = 4.0;

/// Direct predation: per growth pass, each predator token with at least one
/// adjacent eligible prey token has this probability of consuming exactly
/// one of them (token removed from the board). Independent of
/// `PREY_PREDATOR_SUPPRESSION` above, which only dampens prey's own growth
/// score — this is the actual kill. Kept probabilistic (rather than
/// guaranteed) so a single predator token doesn't deterministically strip
/// its whole neighborhood every pass.
pub const PREDATION_CONSUME_CHANCE: f32 = 0.5;

/// Minimum viable population threshold (content doc: "confirmed in shape,
/// not yet numerically specced"). Social species need this many on-map
/// tokens before their relationships count toward scoring at all; every
/// other species only needs 1 (i.e. no real threshold).
pub const SOCIAL_SPECIES_MIN_POPULATION: u32 = 3;
pub const DEFAULT_MIN_POPULATION: u32 = 1;

/// Secret objective numeric target for the "N+ of species X" population-goal
/// card variant (brief item 5, "mixed per-card").
pub const SECRET_OBJECTIVE_POPULATION_TARGET: u32 = 4;
/// Secret objective distinct-terrain count for the adjacency-goal variant
/// ("species X adjacent to 2+ distinct terrain types").
pub const SECRET_OBJECTIVE_ADJACENT_TERRAIN_COUNT: usize = 2;

/// Shared group threshold (brief: "likely a dual condition ... explicitly
/// left untuned"). Both conditions must hold for anyone's secret objective
/// to count.
pub const GROUP_THRESHOLD_MIN_TOTAL_POPULATION: u32 = 15;
pub const GROUP_THRESHOLD_MIN_CONTIGUOUS_TERRAIN_CHAIN: usize = 6;

/// Number of turns before the game ends and objectives are revealed — not
/// called out explicitly in the brief; picked so a session with the
/// generous board radius above has room to fill in meaningfully without
/// dragging (retune once played).
pub const TOTAL_TURNS_PER_PLAYER: u32 = 10;
