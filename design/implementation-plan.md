# Conterra: Implementation Plan (Backend + Frontend)

Status: implementation-layer spec derived from
[deckbuilder-economy-concept.md](deckbuilder-economy-concept.md). This
translates the settled design into concrete code changes against the
current codebase. It is a **rules replacement on a shared substrate**,
not a refactor — the design is effectively a new game that keeps
Conterra's hex board, species roster, food web, and networking.

Read the concept doc first for *why*; this doc is *what to change, where*.

---

## 0. Framing: what survives, what dies, what's new

The current game (place terrain shapes → seed animals → a continuous
Fibonacci sim grows them → hit a population/corridor threshold) shares
only a substrate with the new design.

**KEEP (the substrate):**
- `hex.rs` / `frontend/src/hex.ts` — axial hex geometry, neighbors,
  rotation, spiral bounds. Untouched.
- `species.rs` — the 39-species roster, `FoodWebEdge`, `food_web()`,
  `prey_of`/`predators_of`, and **`Tier { Base, Mid, Apex }`**. This is
  the single most valuable keep: the design's prey/mid/apex trophic
  tiers *are* this enum, already derived from the food web. No change
  except adding a homeland/frontier terrain-class lookup (§1.2).
- `board.rs` spatial core — `radius`, `valid_hexes`, `terrain` map,
  `animals` map, the flood-fill machinery (`animal_colonies`,
  `longest_terrain_corridor`). The connected-component pattern is reused
  wholesale for **regions**.
- `server.rs` — WebSocket/HTTP scaffolding, `ManagedGame`, `AppState`,
  per-connection mpsc, broadcast, dev persistence. Message *dispatch*
  changes; the transport does not.
- `store.ts` — the single networking/state chokepoint. Protocol payloads
  change; the architecture (one authoritative snapshot + local UI state)
  is exactly right and stays.
- `Board.tsx` SVG rendering, `HexTile.tsx`, hex math. Extended, not
  replaced.

**REPLACE:**
- `growth.rs` — the entire Fibonacci model (`colony_factors_and_rate`,
  `f32` counters, previous-counters, directions, spillover thresholds)
  → the cube-pyramid support rule + top-down world-phase sweep (§2).
- `game.rs` turn engine — sequential `advance_turn` with round-wrap
  growth → simultaneous plan/resolve rounds + world phase + Elder (§4).
- `market.rs` — two option rows → the supply system: suited piles,
  infrastructure display, closed Blessing deck (§5).
- `scoring.rs` + `objectives.rs` — population/corridor gate + secret
  objectives → the Sacrifice win condition (§6).
- Board population state — `animal_counters: f32`,
  `animal_previous_counters`, `animal_directions` → a single
  `animal_cubes: u8` (0–3) per tile (§1.1). Delete the other two.

**NEW (greenfield modules):**
- `cards.rs` / `deck.rs` — card definitions, per-player deck/hand/discard.
- `meeple.rs` — meeples, position, carried tools/goods.
- `economy.rs` — personal money wallets, fixed price table, goods.
- `shrine.rs` — the hidden Favor token pool (blind-draw).
- `events.rs` — event track, event resolution, disease markers,
  Block/Divert/Let-happen.
- `cataclysm.rs` — the Act-2 threat, patch-joining, Sacrifice trigger.
- `actions.rs` — the player verb set (Walk/Hunt/Sell/Pray/Process/Buy/
  Seed/Exchange) and the simultaneous-plan commit/resolve engine.
- `regions.rs` — connected matching-terrain regions + support-rule checks.
- Frontend: `Hand`, `Shrine`, `PlayerPanel`, `EventTrack`, `SupplyPiles`,
  `PlanningTray`, `Meeple` components.

---

## 1. Core state model changes (`board.rs`, new types)

### 1.1 Population: cubes, not counters

Replace the three population HashMaps with one:

```rust
// board.rs — REMOVE:
//   pub animal_counters: HashMap<Hex, f32>,
//   pub animal_previous_counters: HashMap<Hex, f32>,
//   pub animal_directions: HashMap<Hex, Direction>,
// ADD:
pub animal_cubes: HashMap<Hex, u8>,   // 1..=3; absent == empty tile
```

Invariant: a tile in `animals` has `animal_cubes[hex] ∈ 1..=3`. One
species per tile (already true — `animals` is `HashMap<Hex, Species>`).
`Direction` enum and all Fibonacci seed constants
(`INITIAL_POPULATION`, `PREY_*`, `PREDATOR_*`, `GROWTH_RATE_CAP`,
spillover/starvation thresholds) are deleted from `balance.rs`.

`Board::place_animal` keeps its eligibility spine (in-bounds, terrain
present, species-can-live-here, tile empty) but now also enforces the
**support rule** (§2.2) and seeds `animal_cubes = 1`.

### 1.2 Terrain: add Wetland and Sea; add terrain class

```rust
// terrain.rs
pub enum Terrain { Forest, River, Ocean, Savanna, Mountain, Wetland, Sea }

pub enum TerrainClass { Homeland, Frontier, Inert }
// Forest/River/Savanna/Wetland => Homeland (Processable, base ecosystems)
// Mountain/Ocean              => Frontier (only Cataclysm reaches them)
// Sea                         => Inert (buffer ring, undevelopable until Cataclysm)

impl Terrain {
    pub fn class(&self) -> TerrainClass { /* match */ }
    pub fn is_processable(&self) -> bool { matches!(self, Forest|Savanna|Wetland) }
}
```

- **Wetland** is required for Processing (Forest↔Wetland↔Savanna) and for
  flood events. The species roster does not yet have Wetland species —
  either map Wetland to River's food web for now or add wetland species
  in `species.rs` (open question, §9).
- **Sea** is the inert buffer ring generated around the Pangea at setup;
  undevelopable until a Cataclysm activates a stretch.

### 1.3 Regions (`regions.rs`, new)

A **region** is a connected component of same-terrain-class-compatible
tiles used by the support rule. Reuse the `animal_colonies` flood-fill
pattern:

```rust
pub struct Region { pub terrain: Terrain, pub tiles: Vec<Hex> }
pub fn regions_of_terrain(board: &Board, terrain: Terrain) -> Vec<Region>;
pub fn region_containing(board: &Board, hex: Hex) -> Option<Region>;
```

"Nothing crosses a region border" (support, growth, starvation) means
every support check is scoped to one `Region`. Regions are recomputed
per world-phase sweep, never stored (same stateless approach as colonies).

### 1.4 Full-tile helpers (the whole ecology's vocabulary)

```rust
// regions.rs
/// Tiles in this region fully occupied (cubes == 3) by `species`' tier.
pub fn full_tiles(board: &Board, region: &Region, tier: Tier) -> usize;
/// Tiles in this region occupied (>=1 cube) by `species`' tier.
pub fn occupied_tiles(board: &Board, region: &Region, tier: Tier) -> usize;
```

---

## 2. The growth model (`growth.rs` — full replacement)

Delete `colony_factors_and_rate`, `ColonyFactors`, `Outcome`,
`colony_spillover_threshold`, `colony_preview`, `run_growth_pass`.
Replace with the support rule + a single top-down sweep.

### 2.1 The support rule (one function, three duties)

```rust
/// In a region, a tier may occupy strictly fewer tiles than the tier
/// below has FULL tiles. Tier 1 (Base) is always supported.
pub fn is_supported(board: &Board, edges: &[FoodWebEdge],
                    region: &Region, species: Species) -> bool {
    match tier(edges, species) {
        Tier::Base => true,                       // terrain feeds it
        Tier::Mid  => occupied_tiles(region, Mid)  < full_tiles(region, Base),
        Tier::Apex => occupied_tiles(region, Apex) < full_tiles(region, Mid),
    }
}
```

### 2.2 Placement gates (Seed + spread share these)

```rust
/// A new upper-tier tile is legal only if BOTH hold after placement:
///  (a) count: still strictly fewer tiles than full tiles below
///  (b) adjacency: the new tile touches a FULL tile of the tier below
pub fn can_place_new_tile(board, edges, region, species, hex) -> bool;
```

Base tier is exempt from both gates (spreads onto any empty adjacent
matching hex). This replaces `is_species_unlocked` /
`HABITAT_THRESHOLD_*` gating entirely.

### 2.3 The world phase — one top-down sweep + two ticks

```rust
pub struct WorldPhaseReport {
    pub grown: Vec<(Hex, Species)>,
    pub starved: Vec<(Hex, Species)>,
    pub disease_chips: Vec<(Hex, Species)>,
    pub event: Option<EventResolution>,
}

pub fn run_world_phase(board, edges, diseases, event_track, rng) -> WorldPhaseReport {
    // 1. THE WILD RESOLVES — top-down: Apex, then Mid, then Base.
    //    Each (species, region) touched exactly once:
    //      - !is_supported            => remove 1 cube from EMPTIEST tile
    //      - supported & legal target => add 1 cube to FULLEST partial tile,
    //                                    else claim a new adjacent legal tile
    //      - else (no room)           => nothing
    //    Top-down order = cascade delay for free (each tier read before
    //    the tier below changes). One touch, one direction — no churn.
    // 2. CHIP — each disease marker removes 1 cube (after the sweep).
    // 3. EVENT TRACK — advance by player count; on crossing, fire an event.
}
```

Key implementation notes:
- Iterate species grouped by tier in the order Apex → Mid → Base. Within
  a tier, process fewer-cubes-first (deterministic tiebreak).
- "Emptiest/fullest tile first" — sort a region's tiles for that species
  by cube count.
- Contested empty tile between an upper tier and the base resolves to the
  upper tier automatically because it's swept first (no special rule);
  base fills elsewhere next phase. Harmless per the design's asymmetry.
- This is deterministic except where a genuine tie needs `rng` (e.g. two
  equally-empty tiles) — mirror the current code's `shuffle(rng)` habit.

### 2.4 Preview (for hover tooltips)

`AnimalTileInfo` loses all the Fibonacci fields. New per-tile preview:

```rust
pub struct TilePreview {
    pub cubes: u8,
    pub supported: bool,          // will it grow/hold, or starve next phase?
    pub full: bool,               // cubes == 3 (load-bearing?)
    pub next_phase: NextPhase,    // Grow | Hold | Shrink
}
```

---

## 3. Terrain mutation (`board.rs`, `events.rs`, `actions.rs`)

The design's key simplifier: **any terrain change kills every cube on
that tile.** So mutation is always clear-then-set — never preserve
population across a change.

```rust
impl Board {
    /// Change a tile's terrain. Clears any animal + cubes there first
    /// (habitat destroyed). Returns the removed species for reporting.
    pub fn change_terrain(&mut self, hex: Hex, to: Terrain) -> Option<Species>;
}
```

Callers:
- **Processing** (`actions.rs`) — deliberate Forest↔Wetland↔Savanna. Cost
  asymmetric (degrade cheap, restore expensive) via `economy.rs` prices.
  Because it clears cubes, a player cannot Process a tile their own
  ecosystem occupies without killing it — no extra rule needed.
- **Events** (`events.rs`) — flood/fire/etc. place terrain from a "bank"
  onto tiles adjacent to matching existing terrain (encroachment), width
  = intensity. Each changed tile clears its cubes = the destruction.
- **Cataclysm** (`cataclysm.rs`) — the only thing that converts
  Sea/joins Frontier patches.

Encroachment helper (mirrors event coherence rule):

```rust
/// Tiles eligible to become `terrain` this event: empty/land tiles
/// adjacent to an existing tile of `terrain`. Pick `intensity` of them.
pub fn encroachment_targets(board, terrain, intensity, rng) -> Vec<Hex>;
```

Component note (physical edition, not code): overlay chits (prototype)
vs recessed swap-tiles (final). Irrelevant to the digital build — terrain
is just the `Terrain` value in the map.

---

## 4. Turn engine: simultaneous rounds (`game.rs`, `actions.rs`)

The biggest structural change. Current: strictly sequential
`advance_turn`, growth on round-wrap. New: simultaneous secret planning →
ordered resolution → world phase.

### 4.1 Round structure

```rust
pub enum RoundPhase {
    Planning,     // all players commit plays face-down
    Resolving,    // committed plays resolve in seat order
    World,        // run_world_phase, Elder handles any fired event
}
```

Per-player per-round commit:

```rust
pub struct PlannedTurn {
    pub player: PlayerId,
    pub plays: Vec<CardPlay>,       // 2-3 from hand (budget in balance.rs)
    pub walks: Vec<WalkCommit>,     // each discards a card from hand
    pub committed: bool,
}
```

Flow (replaces `advance_turn`):
1. **Planning:** each player sends `Commit { plays, walks }`. Hidden from
   others (server stores, does not broadcast contents). When all
   `committed`, advance to Resolving.
2. **Resolving:** apply each player's plays in seat order (deterministic).
   Actions mutate board/economy/shrine (§7).
3. **World:** `run_world_phase`. Advance the event track by player count;
   if an event fires, the **Elder** (rotating token) resolves it
   (Block/Divert/Let-happen) via a follow-up message.
4. Rotate Elder, deal each player back up to hand size, check Sacrifice
   win / Cataclysm loss, start next round's Planning.

### 4.2 The Elder

```rust
// game.rs / GameSession
pub elder_idx: usize,   // rotates each round
```

Only the Elder may answer a fired event. Because the Shrine is
uncountable (§8), their Block/Divert is an *attempt* resolved by
blind-draw.

### 4.3 Player-count scaling

- `total_rounds` and event cadence scale with player count (design open
  question — stub with a constant, §9).
- **Meeples = players, min 2** (solo drives 2). One deck per player
  always; solo's larger hand/play budget is a `balance.rs` constant.
- Everything keyed to meeples/decks, never "player" — enforce in naming.

---

## 5. The supply system (`market.rs` → `supply.rs`)

Replace the two `MarketOption` rows with three structures.

```rust
pub enum Pile { AnimalTier(Tier), Labor, Market, Devotion }   // suited draw piles

pub struct Supply {
    pub piles: HashMap<Pile, Vec<Card>>,        // face-down; draw 3 keep 1
    pub infrastructure: Vec<InfraItem>,         // face-up, fixed price
    pub blessings: Vec<BlessingCard>,           // closed deck, blind draw
    pub pile_prices: HashMap<Pile, u32>,        // flat, printed
}
```

- **Buy from a pile** (`actions.rs::buy`): pay `pile_prices[pile]` from the
  buyer's wallet, draw 3, return 2, keep 1 to discard pile. Server reveals
  the 3 only to the buyer (per-recipient snapshot already supports this).
- **Animal piles** are keyed by `Tier` — buyer knows the trophic tier,
  not the species/terrain (draw is blind within the pile).
- **Infrastructure** (Buildings + Tools) is face-up, bought with money,
  never enters a deck.
- **Blessing deck** is fully closed — blind draw on Favor spend.

Keep and reuse: `species_edges()`, `rotated_translated` (terrain-shape
geometry may still be used by the Cataclysm/patch generator). Delete the
terrain-shape *market* and animal-row unlock gating.

---

## 6. Win condition (`scoring.rs` + `objectives.rs` → `sacrifice.rs`)

Delete `group_threshold_status`, `SecretObjective`, `deal_objective`,
`evaluate_objective`. Replace with the Sacrifice.

```rust
pub struct SacrificeRequirement {
    pub homeland: Vec<Terrain>,   // 2 of {Forest, River, Savanna}, game-chosen
    pub frontier: Terrain,        // 1 of {Mountain, Ocean}, game-chosen (wildcard)
}
// Win = an apex predator from each required ecosystem, alive simultaneously,
// hunted and offered at the Shrine. Three apex predators total.

pub fn sacrifice_ready(board, edges, req) -> bool;   // all required apexes present+huntable
pub fn check_win(session) -> Option<GameOutcome>;
```

- The required subset is decided at setup but its **reveal timing** is an
  open question (§9) — the frontier pick should reveal late for the twist.
- Act-1 floor (population + connected corridor) may survive as a
  *prerequisite* gate before Act 2 — reuse `longest_terrain_corridor`.
- End-of-game **score** (speed, diversity, Temples, ecosystem health) is a
  tally, not a gate — a lightweight `Scorecard` struct.

---

## 7. Player actions (`actions.rs`, new)

The verb set, all routed through the simultaneous-resolve engine (§4).
Each is a `CardPlay` variant or a base action.

```rust
pub enum CardPlay {
    Seed { species: Species, hex: Hex },       // gated by support+adjacency (§2.2)
    Hunt { hex: Hex },                          // remove cubes, tool-gated by tier
    Sell,                                       // unload ALL carried goods, at core
    Pray,                                       // insert N tokens into Shrine (§8)
    Process { hex: Hex, to: Terrain },          // terrain change (§3)
    Buy { pile: Pile },                         // draw 3 keep 1 (§5)
    PlayBlessing { card: BlessingCardId, target: Option<Hex> },
    PlayTactical { card: TacticalCardId, .. },
}
// Base actions (not cards):
//   Walk  — discard any card to move a meeple one building
//   Exchange — free hand-off (money/tool/goods) between co-located meeples
```

- **Hunt** tool-gating: `Tier::Base` → default arrow (free); `Mid` → mid
  tool; `Apex` → rifle. Tools live on meeples (`meeple.rs`), carried,
  Exchange-able. Hunting removes cubes to the meeple's satchel as goods.
- **Sell** requires meeple at the civilization core; unloads the whole
  satchel; pays fixed prices (§ economy).
- **Range checks** (building radius, altar radius) are static "is this hex
  in range" — no movement simulation. Store building positions, compute
  reachable set.

---

## 8. Economy + Shrine (`economy.rs`, `shrine.rs`, new)

### 8.1 Personal money + fixed prices

```rust
// economy.rs
pub struct Wallet { pub player: PlayerId, pub money: u32 }   // per player, not pooled
pub fn sell_price(tier: Tier, terrain: Terrain) -> u32;      // FIXED constant table
```

No price tracks, no elasticity, no recovery — a `tier × terrain` constant
table (Savanna cheapest, Forest premium; higher tier = higher price).
Exchange between co-located meeples is free. Basic actions cost no money.

### 8.2 The Shrine (hidden Favor)

```rust
// shrine.rs
pub struct Shrine {
    pub tokens: Vec<u8>,          // face-down offerings; values 1..=3, SERVER-ONLY
    pub supply_remaining: usize,  // public: how MANY tokens left to offer
}
impl Shrine {
    pub fn pray(&mut self, count: usize, rng: &mut impl Rng);  // insert `count` shuffled tokens
    pub fn blind_pay(&mut self, cost: u32, rng) -> PayResult;  // draw until met; overshoot burned
}
```

- **Uncountable by design:** the token *values* are never in any client
  snapshot. Only `supply_remaining` (public count, not worth) is wired.
- `blind_pay` draws tokens one at a time until `cost` is met; leftover of
  the last token is discarded ("the god does not make change"). A failed
  Divert that meets the cheaper Block cost degrades to Block.
- This is the one piece of genuinely hidden server state beyond secret
  hands — it must never leak through the snapshot.

---

## 9. Events, disease, Cataclysm (`events.rs`, `cataclysm.rs`, new)

```rust
// events.rs
pub struct EventTrack { pub position: u32, pub threshold: u32 }  // advance by player count
pub enum EventKind { Fire, Flood, Drought, Blight }
pub enum EventResponse { Block, Divert { to: Hex }, LetHappen }

pub struct DiseaseMarker { pub region_seed: Hex }   // chips 1 cube/world-phase until Cleansed

pub fn resolve_event(board, kind, response, shrine, rng) -> EventResolution;
// intensity rolled once; width derived from intensity; encroachment-coherent.
```

- **Blight** leaves a `DiseaseMarker` (persistent board state — the one
  sanctioned exception to "events resolve once"). Chipped in world-phase
  step 2. Cleanse blessing removes it.
- **Cataclysm** (`cataclysm.rs`): Act-2 existential threat. Joins Frontier
  (Mountain/Ocean) patches into contiguous habitat via the Divert
  machinery; reveals which patch is "correct" (Temple-bearing). Triggers
  the Shrine build + Sacrifice availability. Timing/guarantee are open
  (§ balance stubs).

---

## 10. Protocol changes (`protocol.rs` + `frontend/src/types.ts`)

### New `ClientMessage` variants
```rust
Join { name }                      // keep
Start                              // keep
Commit { plays: Vec<CardPlay>, walks: Vec<WalkCommit> }   // NEW: secret plan
ElderRespond { response: EventResponse }                  // NEW
Buy { pile: Pile }                 // (or fold into Commit)
// DELETE: Select { option_id, placement }, Shuffle
```

### New `StateSnapshot` (per-recipient, secrecy-critical)
Remove: `terrain_row`, `animal_row`, `my_objective`, all Fibonacci
`AnimalTileInfo` fields, `last_spillover`/`last_starvation`,
`colony_starvation_threshold`.

Add:
```rust
pub struct StateSnapshot {
    pub phase: GamePhase,
    pub round_phase: RoundPhase,
    pub players: Vec<PlayerPublic>,        // name, money, meeple positions, elder?
    pub terrain: Vec<(Hex, Terrain)>,
    pub animals: Vec<(Hex, Species, u8)>,  // species + cube count
    pub diseases: Vec<Hex>,
    pub buildings: Vec<Building>,
    pub event_track: EventTrackView,       // position/threshold only
    pub shrine_supply_remaining: usize,    // COUNT ONLY — never token values
    pub supply: SupplyView,                // pile prices, infra face-up; blessing deck count only
    pub my_hand: Vec<Card>,                // recipient's own hand ONLY
    pub my_deck_count: usize,
    pub my_draw: Option<Vec<Card>>,        // the 3 from a Buy, recipient only
    pub sacrifice: SacrificeView,          // revealed requirement so far
    pub result: Option<GameOutcome>,
}
```

The existing `for_player` per-recipient construction (protocol.rs:88) is
exactly the mechanism needed for hidden hands + hidden Favor — extend it,
don't replace it.

---

## 11. Frontend changes (SolidJS — `frontend/src/`)

> The frontend is **SolidJS**, not React: `createSignal`/`createStore`/
> `createMemo`, `<For>`/`<Show>`, `class=`. Plan accordingly.

### Keep / extend
- `store.ts` — the chokepoint. Add new message senders (`commit`,
  `elderRespond`, `buy`), extend `handleMessage`. Architecture unchanged.
- `Board.tsx` / `HexTile.tsx` — extend to render cubes (0–3 pips per tile,
  colored by tier), disease markers, buildings, and meeples. The
  `preview` memo's job shifts from seam-check to support/adjacency-check
  (mirror §2.2 client-side for placement feedback).
- `hex.ts` — unchanged.
- `format.ts` — extend with card/price/tier formatting.

### Replace
- `types.ts` — mirror the new protocol (§10). Delete `MarketOption`,
  `SecretObjective`, Fibonacci `AnimalTileInfo` fields.
- `MarketRow.tsx` → `SupplyPiles.tsx` — suited piles (buy → draw-3-keep-1
  modal), infrastructure display, blessing deck.
- `Colony.tsx` → cube rendering inside `HexTile` or a small `Cubes.tsx`;
  tooltip shows cubes/supported/full/next-phase (§2.4).
- `Sidebar.tsx` → `PlayerPanel.tsx` — money, hand size, meeple locations,
  Elder indicator, sacrifice progress.
- `ResultModal.tsx` — rewire to `GameOutcome` (win/loss + scorecard).

### New components
- `Hand.tsx` — the recipient's own cards; select 2–3 to commit + cards to
  burn for Walks. Drives the `Commit` message.
- `PlanningTray.tsx` — the secret commit UI; shows "committed / waiting on
  N players" without revealing others' plays.
- `Shrine.tsx` — shows `shrine_supply_remaining` (the public count) as
  offering tokens remaining; never a total value. Blind-pay animation on
  Block/Divert.
- `EventTrack.tsx` — track position, next-event marker, Elder badge, and
  the Block/Divert/Let-happen chooser (Elder only).
- `Meeple.tsx` — meeple tokens on buildings, carried tools/goods,
  Exchange affordance.

### `selection.ts`
Local UI state expands: from `{selectedOption, rotation}` to a
plan-builder (selected cards, targets, walk-discards) held locally until
`Commit` fires. Same "local bridge" role, bigger payload.

---

## 12. Balance constants to (re)introduce (`balance.rs`)

Delete all Fibonacci/growth-rate constants. Add:
- `TILE_CUBE_CAP = 3`
- `HAND_SIZE`, `PLAY_BUDGET` (2–3), `SOLO_HAND_SIZE`, `SOLO_PLAY_BUDGET`
- `STARTING_DECK` composition (2 Harvest, 1 Sell, 1 Pray, 1 Process,
  1 Seed-T1, 2 Chores)
- `EVENT_TRACK_THRESHOLD` (=4 player-turns worth), Elder rotation
- `SELL_PRICES[tier][terrain]` fixed table
- `PILE_PRICES[pile]`, infrastructure prices
- `OFFERING_TOKEN_SUPPLY` (~10), token value distribution (1/2/3)
- `SACRIFICE_HOMELAND_COUNT = 2`, `SACRIFICE_FRONTIER_COUNT = 1`
- `TOTAL_ROUNDS` (player-count-scaled), Cataclysm timing floor/guarantee
- `MAP_RADIUS`, sea-buffer width, frontier-patch count/size

---

## 13. Suggested build phases (each yields something runnable)

A big-bang rewrite is the wrong move; the substrate lets us stage it.

1. **Cube substrate.** Swap `animal_counters` → `animal_cubes`, delete
   Fibonacci, implement the support rule + world-phase sweep (§1–2). Keep
   the *old* sequential turn loop temporarily driving the new growth. Get
   the ecology correct and glanceable in isolation. Port the current
   growth tests to support-rule assertions.
2. **Regions + terrain mutation.** `regions.rs`, `change_terrain`
   (clear-on-change), Wetland/Sea, Processing action (§3). Still
   single-verb turns.
3. **Economy + supply + decks.** Wallets, fixed prices, `supply.rs`,
   `cards.rs`/`deck.rs`, Seed/Hunt/Sell/Buy verbs, meeples + satchels +
   tools (§5,7,8.1). Solo/hotseat playable as a resource loop.
4. **Simultaneous rounds.** Replace `advance_turn` with the plan/resolve/
   world engine + Elder (§4). Protocol `Commit` (§10). Frontend Hand +
   PlanningTray.
5. **Favor + Shrine.** `shrine.rs`, Pray, blind-pay, hidden token values
   (§8.2). Frontend Shrine.
6. **Events + disease.** `events.rs`, event track, Block/Divert/Let-happen,
   blight markers (§9). Frontend EventTrack.
7. **Cataclysm + Sacrifice.** `cataclysm.rs`, frontier patch-joining,
   `sacrifice.rs` win condition, scorecard (§6). Reveal-timing decision.
8. **Scaling + polish.** Player-count map/round/hand scaling, world-phase
   parallelization affordance, naming pass.

Phase 1 alone de-risks the single most important claim of the redesign
(that the growth model is simple and correct); everything after is
additive.

---

## 14. Open questions that block or shape implementation

Design-doc open items that code needs decided (or stubbed with a
constant) before the relevant phase:

- **Sacrifice reveal timing** (§6) — setup vs Cataclysm reveal. Blocks
  `sacrifice.rs` + snapshot. Lean: frontier revealed late.
- **Road** — tempo (free Walk between connected buildings) vs permission
  (required for distant Houses). Lean: tempo. Blocks building/movement.
- **Correct-patch determination** — random at Divert vs Temple-scouted.
  Blocks `cataclysm.rs`.
- **Wetland species** — reuse River's web, or author a Wetland roster in
  `species.rs`. Blocks Processing-into-Wetland being meaningful.
- **Cataclysm timing** — floor vs guaranteed checkpoint; `TOTAL_ROUNDS`
  anchor. Balance stub for now.
- **Suit taxonomy** — exact piles (Labor/Market/Devotion assumed here) and
  card lists. Only the *shape* is needed for phase 3; contents tune later.
- **Naming pass** — two collisions to resolve before they ossify in code:
  sacred nouns (offering box / Altar / Temple / Shrine) and the
  terrain-class-vs-trophic "Tier" overload. Use `TerrainClass` +
  `Tier` (trophic) distinctly from day one (§1.2).

---

## 15. Tests to preserve or add

- **Preserve as behavioral specs:** `game.rs:312` persistence round-trip
  (extend to new state), the hex/geometry tests, food-web/tier lookups.
- **Delete:** `market.rs:203-309` seam/shape-placement tests (mechanic
  removed), Fibonacci growth assertions.
- **Add (the load-bearing new invariants):**
  - Support rule: a Mid tile can't exist without 2 full Base tiles;
    strictly-fewer-tiles holds after every world phase.
  - Top-down cascade delay: breaking a full prey tile starves the mid
    *next* phase, apex the phase after — never same-phase.
  - No churn: no species loses and regains a cube in one phase.
  - Base is unstarvable; one surviving base cube regrows.
  - Terrain change clears cubes.
  - Shrine values never appear in any `StateSnapshot` (secrecy test).
  - Blind-pay: overshoot burned; failed Divert degrades to Block.
