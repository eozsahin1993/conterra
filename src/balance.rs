//! Tunable constants. Values are sane defaults, not final — retune from
//! playtests rather than from first principles.

/// Board radius (see `Hex::spiral_from_origin`) — generous size is the
/// primary lever against placement difficulty.
pub const BOARD_RADIUS: i32 = 6;

/// Market row size — fixed at 4, not tunable.
pub const MARKET_ROW_SIZE: usize = 4;

/// Every terrain-shape piece — market-row options and the starting tile —
/// is a fixed 4-hex cluster.
pub const TERRAIN_SHAPE_SIZE: usize = 4;

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
/// can drive pressure negative).
pub const PREY_PREDATOR_SUPPRESSION: f32 = 2.0;

/// Predator-role: minimum bordering prey count to be thriving (rising)
/// rather than merely surviving (flat).
pub const PREDATOR_MIN_ADJACENT_PREY_THRESHOLD: u32 = 2;
/// Pressure per unit of bordering prey above the minimum threshold.
pub const PREDATOR_RISE_RATE_PER_EXCESS_PREY: f32 = 1.0;
/// Pressure when there is zero bordering prey at all.
pub const PREDATOR_FALL_RATE_AT_ZERO_PREY: f32 = -2.0;

/// Non-linear acceleration: `rate = pressure * (1 + FACTOR * |pressure|)`.
pub const GROWTH_NONLINEAR_ACCEL_FACTOR: f32 = 0.15;
/// Hard cap on a single pass's rate magnitude.
pub const GROWTH_RATE_CAP: f32 = 6.0;

/// A colony at/above this spills over: one new tile per pass, on an
/// adjacent open hex matching one of the colony's own terrains.
pub const COLONY_SPILLOVER_THRESHOLD: f32 = 16.0;
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
