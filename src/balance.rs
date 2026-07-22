//! Tunable constants. Values are sane defaults, not final — retune from
//! playtests rather than from first principles.

/// Board radius (see `Hex::spiral_from_origin`) — generous size is the
/// primary lever against placement difficulty. Was 10, but the frontend hex
/// render size grew (26px -> 34px) and this was shrunk back down to keep
/// the on-screen map footprint roughly the same; with the seam-matching
/// placement rule, watch for the board running out of legal spots too
/// quickly and retune from playtests if so.
pub const BOARD_RADIUS: i32 = 7;

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
/// Lowered from 2 to 1 after that was still too restrictive in practice.
pub const PLACEMENT_MIN_MATCHING_SEAMS: usize = 1;

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
/// tiles share one counter (`board::animal_colonies`).
///
/// Growth is Fibonacci-shaped: each pass a colony can gain its own
/// *previous* population as new growth (`next = current + previous`),
/// dampened by crowding/predation. A freshly-placed token starts here, with
/// this as its seed "previous" — under ideal conditions the first few
/// passes run the real Fibonacci sequence (2, 3, 5, 8, 13, ...).
pub const INITIAL_POPULATION: f32 = 2.0;
pub const INITIAL_PREVIOUS_POPULATION: f32 = 1.0;

/// Penalty per bordering contending prey (a different, non-apex species
/// competing for the same space) — subtracted directly from population,
/// and also counts against the "room" available for growth.
pub const PREY_CONTENTION_PENALTY: f32 = 0.5;
/// Penalty per bordering predator — subtracted directly from population
/// (being eaten), and also counts against available room. Boom-bust, not
/// flat suppression: scales with predator count.
pub const PREY_PREDATOR_SUPPRESSION: f32 = 3.0;

/// Predator-role: bordering prey count for *full* growth potential (prey
/// factor caps at 1.0 here); scales linearly below this.
pub const PREDATOR_FULL_GROWTH_PREY_COUNT: u32 = 3;
/// Flat penalty applied when there is zero bordering prey at all — a
/// predator colony that outruns its prey supply crashes noticeably fast
/// rather than lingering.
pub const PREDATOR_FALL_RATE_AT_ZERO_PREY: f32 = -4.0;

/// Hard cap on the magnitude of a single pass's population change — real
/// Fibonacci growth is unbounded, so this keeps numbers sane over a long
/// game instead of letting a thriving colony's population explode.
pub const GROWTH_RATE_CAP: f32 = 8.0;

/// A colony spills over — one new tile per pass, on an adjacent open hex
/// matching one of the colony's own terrains — once its population is at
/// or above this amount *per tile it already has* (so a bigger colony
/// needs proportionally more population to justify spreading further, not
/// a fixed number regardless of size).
pub const COLONY_SPILLOVER_THRESHOLD_PER_TILE: f32 = 8.0;
/// A colony at/below this starves: one tile removed per pass until gone.
/// Flat, not scaled by size — a colony's total population genuinely
/// hitting zero means it's gone regardless of how many tiles it spans.
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
