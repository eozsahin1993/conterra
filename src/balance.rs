//! Every numeric constant here corresponds to an item in the brief's "Open
//! questions still to resolve" or the content doc's "Open / to design later"
//! list — magnitudes the design explicitly deferred until there was something
//! playable to balance against. Picked to be sane defaults, not final values;
//! retune from actual playtests rather than from first principles.

/// Board radius (see `Hex::spiral_from_origin`) — "generous grid size as the
/// primary lever against placement difficulty." Radius 6 is ~127 tiles,
/// generous for a 1-6 player game.
pub const BOARD_RADIUS: i32 = 6;

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

/// Unified per-tile-colony growth/starvation counter (brief: "Role-based
/// growth, starvation, and spillover — unified into one bidirectional
/// per-tile counter"). Adjacent same-species tiles are treated as one
/// colony sharing a single counter (see `board::animal_colonies`), so these
/// magnitudes apply to a colony's aggregate border, not one tile in
/// isolation. Exact magnitudes explicitly untuned per the brief's open
/// questions — picked as sane defaults pending real playtests.
///
/// Prey-role pressure: rises per open/uncontested bordering hex.
pub const PREY_GROWTH_PER_OPEN_ADJACENT: f32 = 1.0;
/// Prey-role pressure penalty per bordering contending prey (a different,
/// non-apex species competing for the same space).
pub const PREY_CONTENTION_PENALTY: f32 = 0.5;
/// Prey-role pressure penalty per bordering predator (boom-bust, not flat
/// suppression — scales with predator count, can drive pressure negative).
pub const PREY_PREDATOR_SUPPRESSION: f32 = 2.0;

/// Predator-role: minimum bordering prey count for the colony to be
/// considered thriving (rising) rather than merely surviving (flat).
pub const PREDATOR_MIN_ADJACENT_PREY_THRESHOLD: u32 = 2;
/// Predator-role pressure per unit of bordering prey above the minimum
/// threshold (only applies once at/above the threshold).
pub const PREDATOR_RISE_RATE_PER_EXCESS_PREY: f32 = 1.0;
/// Predator-role pressure when there is zero bordering prey at all (falls).
pub const PREDATOR_FALL_RATE_AT_ZERO_PREY: f32 = -2.0;

/// Non-linear acceleration factor applied to raw pressure: `rate = pressure
/// * (1 + FACTOR * |pressure|)`. Favorable/unfavorable conditions don't
/// just add linearly, they compound (brief: "the rate ... is non-linear").
pub const GROWTH_NONLINEAR_ACCEL_FACTOR: f32 = 0.15;
/// Hard cap on the magnitude of a single pass's rate, so one extreme
/// neighborhood can't cause an unbounded single-pass jump.
pub const GROWTH_RATE_CAP: f32 = 6.0;

/// Top threshold: a colony whose counter is at or above this spills over —
/// one new tile added to the same colony, on an adjacent open hex matching
/// one of the colony's own terrains — once per pass, every pass, for as
/// long as it stays at/above threshold (brief: "~16, placeholder").
pub const COLONY_SPILLOVER_THRESHOLD: f32 = 16.0;
/// Bottom threshold: a colony at or below this starves — one tile removed
/// per pass (symmetric with spillover) until the colony is gone.
pub const COLONY_STARVATION_THRESHOLD: f32 = 0.0;

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
pub const TOTAL_TURNS_PER_PLAYER: u32 = 50;
