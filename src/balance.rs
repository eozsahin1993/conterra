//! Tunable constants. Values are sane defaults, not final — retune from
//! playtests rather than from first principles.

/// Board radius (see `Hex::spiral_from_origin`) — generous size is the
/// primary lever against placement difficulty. Bumped up from 6: with the
/// seam-matching placement rule added, a smaller board runs out of legal
/// spots too quickly.
pub const BOARD_RADIUS: i32 = 10;

/// Market row size — fixed at 4, not tunable.
pub const MARKET_ROW_SIZE: usize = 4;

/// Every terrain-shape piece — market-row options and the starting tile —
/// is a fixed 4-hex cluster.
pub const TERRAIN_SHAPE_SIZE: usize = 4;

/// A new piece's hex counts as "seam-safe" if it touches no existing board
/// terrain, or matches whatever existing terrain it touches. At least this
/// many of the piece's hexes must be seam-safe for the placement to be
/// legal — not all 4: since a piece's own internal terrain layout isn't
/// chosen with the board in mind, requiring every touching hex to match
/// made most placements impossible once the board had any real shape.
pub const PLACEMENT_MIN_MATCHING_SEAMS: usize = 2;

/// Each terrain-shape piece mixes 2-3 distinct terrain types across its 4
/// hexes — never one uniform terrain, never all 4 different.
pub const TERRAIN_SHAPE_MIN_DISTINCT: usize = 2;
pub const TERRAIN_SHAPE_MAX_DISTINCT: usize = 3;

/// Shuffling the market row costs the player's turn.
pub const SHUFFLE_COSTS_TURN: bool = true;

/// Habitat maturity thresholds: minimum home-terrain tile count before a
/// species becomes eligible to place. Scaled by tier — apex predators need
/// a more established habitat than base prey.
pub const HABITAT_THRESHOLD_BASE: usize = 4;
pub const HABITAT_THRESHOLD_MID: usize = 6;
pub const HABITAT_THRESHOLD_APEX: usize = 9;

/// Unified per-tile-colony growth/starvation counter. Adjacent same-species
/// tiles share one counter (`board::animal_colonies`), so these magnitudes
/// apply to a colony's aggregate border, not one tile in isolation.
///
/// Prey-role pressure: rises per open/uncontested bordering hex.
pub const PREY_GROWTH_PER_OPEN_ADJACENT: f32 = 1.0;
/// Penalty per bordering contending prey (a different, non-apex species
/// competing for the same space).
pub const PREY_CONTENTION_PENALTY: f32 = 0.5;
/// Penalty per bordering predator (boom-bust — scales with predator count,
/// can drive pressure negative). Raised from 2.0: prey needs to decline
/// faster under real predation pressure so a predator's own food supply
/// isn't effectively infinite.
pub const PREY_PREDATOR_SUPPRESSION: f32 = 3.0;

/// Predator-role: minimum bordering prey count to be thriving (rising)
/// rather than merely surviving (flat). Raised from 2: a predator colony
/// needs more surrounding prey to justify further growth as it expands
/// into thinner territory.
pub const PREDATOR_MIN_ADJACENT_PREY_THRESHOLD: u32 = 3;
/// Pressure per unit of bordering prey above the minimum threshold.
/// Lowered from 1.0 — a big predator colony's border naturally touches
/// more prey tiles just by being bigger, so this needed to be weaker to
/// avoid unbounded runaway growth (e.g. Lions spreading indefinitely).
pub const PREDATOR_RISE_RATE_PER_EXCESS_PREY: f32 = 0.6;
/// Pressure when there is zero bordering prey at all. Made more negative
/// (was -2.0) so a predator colony that outruns its prey supply crashes
/// noticeably faster instead of lingering.
pub const PREDATOR_FALL_RATE_AT_ZERO_PREY: f32 = -4.0;

/// Non-linear acceleration: `rate = pressure * (1 + FACTOR * |pressure|)`.
pub const GROWTH_NONLINEAR_ACCEL_FACTOR: f32 = 0.15;
/// Hard cap on a single pass's rate magnitude.
pub const GROWTH_RATE_CAP: f32 = 6.0;

/// A colony at/above this spills over: one new tile per pass, on an
/// adjacent open hex matching one of the colony's own terrains. Raised
/// from 16 alongside the predator-growth slowdown above, so spillover
/// isn't reached as trivially by a snowballing predator colony.
pub const COLONY_SPILLOVER_THRESHOLD: f32 = 20.0;
/// A colony at/below this starves: one tile removed per pass until gone.
pub const COLONY_STARVATION_THRESHOLD: f32 = 0.0;

/// Minimum viable population: social species need this many on-map tokens
/// before their relationships count toward scoring; everything else needs 1.
pub const SOCIAL_SPECIES_MIN_POPULATION: u32 = 3;
pub const DEFAULT_MIN_POPULATION: u32 = 1;

/// Secret objective numeric target for the population-goal card variant.
pub const SECRET_OBJECTIVE_POPULATION_TARGET: u32 = 4;
/// Distinct-terrain count for the adjacency-goal variant.
pub const SECRET_OBJECTIVE_ADJACENT_TERRAIN_COUNT: usize = 2;

/// Shared group threshold — both conditions must hold for anyone's secret
/// objective to count.
pub const GROUP_THRESHOLD_MIN_TOTAL_POPULATION: u32 = 15;
pub const GROUP_THRESHOLD_MIN_CONTIGUOUS_TERRAIN_CHAIN: usize = 6;

/// Turns before the game ends and objectives are revealed.
pub const TOTAL_TURNS_PER_PLAYER: u32 = 50;
