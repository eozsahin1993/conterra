# Conterra: Commune Concept

Status: brainstorm, not committed. Nothing described here has been built yet.
A 2026-07-23 design session locked in several mechanics as
decided-pending-playtest — turn structure, Walk, the Shrine, personal
money, suited market piles. They're written below as decided.

## The vision

Every player is a civilian of one shared commune, not a separate faction
racing anyone else. There's no rival settlement to out-build — the only
thing that can defeat the group is the world itself failing to stay healthy
enough to live in. Individual players still matter, but through what they
personally choose to specialize in, not through owning separate land or
beating each other.

The game starts on a **pre-built, deliberately fragmented map** — no terrain
type naturally forms a large connected region, just small disconnected
pockets. The commune can't actually win until somewhere on the map has a
large, healthy, connected stretch of matching terrain — which can't happen
by accident on a map this broken up. Repairing the land is the whole
opening game, not a late-game bonus.

The whole map is one Pangea-style landmass, bordered by sea. That border has
to be real, pre-generated board space, not just narrative flavor — a
Volcano's island needs actual room to appear in (roughly 4x4 tiles as a
starting estimate), so the map needs a genuine buffer ring of sea tiles
surrounding the Pangea, sized generously in every direction. Until a Volcano
activates a stretch of it, that buffer is inert — not reachable or
developable, just reserved space (see Events below). Exact buffer size
depends on the Pangea's own radius and isn't pinned down yet.

The one rule everything else has to obey: **nothing may hand the commune
free growth.** Every system here — trading, praying, buying cards — has to
cost something out of the commune's actual living population, the same
population that's already fighting to survive and grow on the board. If a
mechanic lets the group skip that fight, the game gets more boring, not
less, because the easy path will always beat the interesting one.

## The world staying coherent as it changes

Nothing that reshapes the map should feel like tiles teleporting in at
random — that reads as noise, not a living world. The fix: any change can
only ever extend terrain that's already touching the affected area. A flood
only turns land into wetland where it's adjacent to existing water; a
wildfire only spreads into forest that's actually connected to where it
started. Every change should look like a biome encroaching or receding at
its edges, the way a real landscape would — not a patch appearing out of
nowhere. This applies equally to the negative and the positive side below.
This still needs to be stress-tested against how fragmented the starting
map actually is, but it's the current best answer.

## Events — always negative, nobody's fault but the world's

Fire, flood, drought, blight — ambient natural pressure tied to the world
staying broken or neglected. Nobody summons these, nobody controls them,
they're simply what an unhealed world does on its own. They're consistently
bad; there's no positive spin to find in one landing on a colony. This is
what "you vs. nature" actually means here — the danger is systemic, not an
enemy, and it's always working against the commune unless the commune does
something about it.

**Blight is the persistent one.** It resolves like any event — intensity
sets the initial bite of cubes removed — but it also leaves a **disease
marker** on the region, and every world phase that marker chips one more
cube until a Cleanse blessing removes it. The fuse is readable in
full-tile language: the first chip off a load-bearing prey tile breaks
it, and everything above starts starving the next world phase — unless
the region holds a spare full tile, which is what makes redundancy
worth growing. This gives Favor a *standing*
demand between crises — the shrine becomes a maintenance budget, not just
event insurance — and it gives blight's Divert a decision no other event
has: since crowding is real under the 3-cube cap, a plague pointed at an
overgrown monoculture region is the god's own pruning. Same
Block/Divert machinery, genuinely different question. Coherence rule:
like a flood needs adjacent water, blight needs a populated region —
disease needs hosts.

**Mountain and Ocean exist from the start** — small, isolated patches
scattered on the map alongside Forest/River/Savanna, visible from turn one
and permanently unprocessable (see Processing below), but not entirely
useless while isolated: a single small patch has enough room for Tier 1
species, so the commune can start cultivating something there early rather
than just staring at it. What an isolated patch *can't* support is a full
food web — there's no room for Tier 2 until multiple patches become one
larger, contiguous habitat. Since there's more than one patch of each,
some will naturally be bigger or better-positioned than others — a real
reason to scout, not just wait.

**The Cataclysm** (working name — exact theming, a vengeful spirit, a
falling star, something else, is still undecided) is the singular,
existential Act 2 threat: not an opportunity, a danger, threatening to wipe
out the island itself. This is what keeps Favor equally weighted against
Trade instead of risking neglect — ignoring it doesn't just mean missing a
bonus, it risks losing everything. Diverting it uses the same Divert
machinery that already exists, but the job here is specific: **join
connective tiles**, merging several of the small, isolated Mountain/Ocean
patches into one larger, contiguous habitat — big enough to finally support
a full food web up to Tier 2, not just bridging to the mainland. Which
patches end up joined has to include the *correct* one — not every patch
has a **Temple** inside it, and finding one is how the commune confirms a
given patch is part of the site that matters, same delight-of-discovery
idea as before, just now load-bearing instead of pure bonus.

Averting the Cataclysm and unlocking that second ecosystem tier are **the
same act, not two separate things.** The commune builds a **Shrine** at the
correct patch and performs the **Sacrifice** there — see Goals below for
what that actually requires now. Succeeding saves the island and connects
the patch to the mainland in the same stroke; there's no version where one
happens without the other.

The Cataclysm also rolls from a bigger intensity range than an ordinary
event, same as everything "big" here — a real threat should actually
behave like one, which raises the stakes on diverting it well rather than
letting it happen or trying to block it outright.

An apex predator still can't just be seeded the way an ordinary species
can — reusing the same predator-needs-prey logic the food web already runs
on, it would simply starve without a sustained prey population already
thriving in that terrain first. Reaching one alive enough to matter for the
Sacrifice still needs the right tools, bought with Trade ahead of time
(physical gear, rifle-tier, not a card that conjures the outcome) — a basic
tool that works fine on ordinary prey isn't enough for something this
exclusive. Same tiered-difficulty pattern as habitat thresholds and
restoring land, just required again now instead of optional.

**Real scope question this raises:** whichever patch turns out to be
correct still needs genuine room for a supporting ecosystem (the earlier
estimate was 9+ tiles, closer to what an ordinary apex-tier species already
costs elsewhere) — worth generating more than one patch of reasonable size
so a too-small correct patch isn't a dead end. And since the event track's
cadence (see Turns and table talk) is what eventually triggers the
Cataclysm, the exact round count before it becomes likely still needs to
scale sensibly with player count, same open question as ordinary events.

## Buildings

No per-civilian home plots — just one shared town hall for the whole
commune to start. There's no starting region tied to any individual; the
map is fully communal from the outset, matching how harvesting already
works: any colony anywhere can be tended by anyone, so there's no need for
a personal foothold to justify that.

The town hall does have a real mechanical job, though: **actions only work
within a fixed radius of a building.** Planting and harvesting can't happen
past that range from the start. To extend the commune's reach outward, Trade
buys a **House**, which extends the usable radius from wherever it's built —
a genuinely spatial use for Trade money, not just card-buying.

**Praying doesn't automatically follow Houses.** The main altar lives at the
town hall, and prayer only works near an altar, not near any building — so
a commune that expands its farmland fast via Houses can end up with
far-flung colonies no one can Shield or Divert for. A second **Altar** can be
built to extend prayer's reach too, but it's a separate investment from
Houses, not a byproduct of one. That asymmetry is deliberate: Trade sprawls
easily, Favor stays centralized unless the commune deliberately decides to
decentralize it too.

No time or movement is involved in any of this — it's a static "is this tile
in range" check, resolved instantly, consistent with nothing else here
unfolding over turns.

**Harvesting and selling have different range rules.** Harvesting works
anywhere within a House's extended reach, out at the frontier. Selling —
actually converting goods into Trade — only works back at the civilization
core, the same way praying is tied to an altar rather than any building. A
civilian can harvest a distant colony, but has to be home to cash it in.

**Harvesting an animal colony is hunting**, and the tool gates the tier:
Tier 1 prey falls to the baseline kit every civilian carries (the default
arrow); secondary predators need a bought mid-tier tool from the display;
apex needs rifle-tier, as the Sacrifice already demands — and Ocean
species want the boat on top. High-tier animals are double-gated from two
different economies: growing one takes the whole ecosystem underneath it,
and harvesting it takes the tool investment plus the logistics of getting
the armed meeple there.

**Carrying and batch selling** are what make the loop's cost amortizable
rather than flat. Every meeple has a basic satchel — two slots, and goods
or extra Tools both occupy slots; the purchasable Backpack upgrades slot
count, one unified carrying system. **Sell unloads everything carried in
a single play**, so the walk-out-and-back commute amortizes over a full
load — and the haul still wants to be diversified, not because of
prices (fixed) but because of the map: filling a satchel from one
species means stripping its tiles, and load-bearing tiles are exactly
full. The loop is meant to start brutal and be
bought down: satchel to Backpack, lone trips to a stationed hunter
handing goods to a mule via free Exchange, unpaved routes to Roads. That
progression is the engine-building payoff, and it lives in logistics,
not the deck.

Buildings place by **chaining**: a new building must sit within the
commune's current action range, so the network grows from its own edge —
no teleported outposts.

Each civilian is represented by a meeple occupying a specific building at
any given moment — a physical piece, not just tracked state, so where
everyone actually is stays visible at a glance. **Walk** — moving the
meeple to a different building — is paid by **discarding any card from
hand**, resolving instantly the moment it's taken (no multi-turn transit,
consistent with everything else here). That one rule does triple duty: a
card that's dead in the current situation (a Sell stuck at the frontier)
becomes fuel instead of a wasted draw, the commute becomes a real hand
decision — which card do I burn to get home? — rather than a chore, and
the walk-tax lands on deck tempo: nothing free, but nothing wasted either.
Reaching a frontier House to harvest, then getting back to the
civilization center to actually sell, still costs real cards beyond the
harvest and sell themselves — a recurring tax on sprawling outward, not
just a one-time range check. That's what keeps building Houses further out
a genuine tradeoff instead of strictly better.

That tradeoff deliberately fights with the Cataclysm elsewhere in this doc:
staying compact is the efficient day-to-day move, but since nobody can
predict which Mountain/Ocean patch will turn out correct, a fully compact
commune risks having no way to reach it once the Cataclysm reveals it.
There's a constant, low-grade pressure to keep some reach extended even
when it's locally inefficient — insurance against a payoff whose location
nobody controls.

A second **Town Center** is the release valve for that tension, not a
loophole in it — deliberately priced very high, so it's the specific
answer to "we now have a real reason to decentralize" (the correct patch
turning out to be far from home) rather than a default expansion. It
collapses the walk-tax for whichever region it's built in, the same way the
original one does, but affording it should mean giving up real ground
elsewhere. Reaching and developing the correct patch is required to win —
a second Town Center isn't. The site stays reachable the cheaper way,
through ordinary Houses and the walk-tax; the second Town Center only ever
buys convenience on top of that, never access itself.

## Processing — the mundane way terrain changes

Alongside Events and Blessings, Trade can fund **Processing**: precise,
deliberate, human-labor terrain conversion, limited to vegetative/cultivable
terrain — Forest, Wetland, Savanna. Clearing forest into savanna, or
draining wetland into savanna, is cheap — that's mundane labor doing
something it plausibly could. Going the other way costs more: turning
savanna back into forest needs irrigation and planting, not just labor,
because restoring land is harder than degrading it. That asymmetry isn't
arbitrary — it reinforces the same "the world decays if neglected" idea
Events are built on; Processing can make that decay worse cheaply, or fight
it, expensively.

**Mountain and Ocean are permanently outside mundane reach** — they already
exist somewhere on the map, but no amount of Trade or labor connects or
develops them; only the Cataclysm, diverted correctly, can (see Events
below). That's what keeps Processing from quietly replacing the need to
pray: the biggest, most consequential terrain in the game is exclusively
divine to actually reach, even though it's been sitting there in plain
sight the whole game.

## Growth on the table

The digital game's Fibonacci growth sim is computer-shaped; the tabletop
version replaces it with two ideas that together stay auditable at a
glance.

**The support rule — the whole ecology in one sentence: in each
region, a tier may occupy strictly fewer tiles than the tier below has
full tiles.** Two full prey tiles carry one mid-tier tile (which may
fill to 3); two full mid tiles carry one apex tile. There is no cube
arithmetic anywhere: every support question is "count full tiles
below, count occupied tiles above, above must be fewer" — tiny
numbers, and full tiles are the most visible object on the board. The
rule does existence and shape at once: a species violating it is
starving; a species satisfying it, with room, grows — and it may only
claim a *new* tile (by spread or by Seed) if the rule still holds with
that extra tile counted, so neither nature nor players can grow into
violation. Placement above the base is also **local**: any new tile of
a tier above Tier 1 — seeded or spread — must sit adjacent to a full
tile of the tier below. Predators live where the prey is thick, so
pyramids cluster physically: apex beside its full mid tiles, mid
beside its full prey tiles, concentric layers you can photograph.
(Tier 1 is exempt — terrain is its food, so it spreads onto any
adjacent matching tile.) A side effect that defuses growth-order
contention: upper tiers can only claim the few empties beside their
support, while the base can claim anything matching — so when a
predator takes the one contested hex, the ever-growing base just fills
elsewhere next phase. Support is a
*standing requirement, not consumption* — predators never eat down the
prey count, they simply require the full tiles beneath them; the
moment eating is simulated, bookkeeping returns and glanceability
dies. One growth rule, both directions, checked in the world phase:
supported and roomy → add one cube; violating → remove one, emptiest
tile first. And to keep cascades free of calculation: **all checks
read the board as it stood at the start of the world phase** — the
top-down sweep implements this by sequence, so a broken prey tile
starves the mid-tier *next* phase, the apex the phase after; the
dominoes fall one cube per round, and the collapse's slow pace is
itself the comeback window for a Cleanse or re-seed to arrive.
Everything that adds population routes through this single check — the
no-free-growth north star in its simplest form.

**Population is cubes on hexes, Pandemic-style.** Colored cubes, color
= tier (base / mid / apex); the terrain says which species. **Each
tile holds one species only, up to 3 cubes** — a tile reads at a
glance, and frees up only when its last cube leaves. Support never sums
across tiles: 2+1 is not a full tile, and the whole game asks only "how
many *full* tiles" — no cube arithmetic anywhere.

**Placement above the base has two gates, both checked by looking.**
A new upper-tier tile — whether played as a Seed or grown by spread —
is legal only if, after it, (a) the count still holds (the tier still
occupies strictly fewer tiles than full tiles below), and (b) it sits
**adjacent to a full tile of the tier below**. So a mid-tier can't be
born until the region holds 2 full prey tiles, and it must be seeded
onto a specific tile touching one of them — never dropped anywhere in
range. Predators live where the prey is thick, so pyramids cluster
physically into concentric layers you can photograph. (Tier 1 is
exempt from both gates — terrain is its food, so it spreads onto any
empty adjacent matching tile and can never starve.)

**The world phase is one top-down sweep, plus two bookkeeping ticks.**
Apex first, then mid, then base; **each species in each region is
touched exactly once**, and does exactly one thing:
- violating the support rule → **remove one cube**, from its emptiest tile
- satisfying it, with a legal placement available → **add one cube**,
  filling its fullest partial tile before claiming a new adjacent tile
- otherwise (supported but no room) → nothing

One touch, one direction — a species can never lose a cube and win it
back in the same phase. **Top-down order is the entire trick**: because
each tier is resolved before the tier below it changes, every tier
reads its support as it stood at phase start, with zero memory and no
"apply simultaneously" instruction to track. A broken prey tile
therefore starves the mid *next* phase, the apex the phase after —
dominoes falling one cube per round, slow enough that a Cleanse,
re-seed, or Expedite has a real window to arrive. Growth is never
player-chosen (wildlife steers itself; Blessings are the only human
hand on the spread). Then: **Chip** — each disease marker removes one
cube, after the sweep so its broken support bites next phase — and
**Event track** — advance by player count, fire on crossing 4.

That single sweep is the answer to the churn worry: because a species
acts once and the base is resolved last, the case you flagged — a tier
losing a cube to starvation and regaining it the same phase — cannot
occur. The only genuine contention is one empty tile an upper tier and
the base both want, and it resolves in the upper tier's favor by sweep
order. That's harmless, and the reason is the load-bearing asymmetry of
the whole ecology: **everything constrains the predator; nothing
constrains the base.** An upper tier's growth passes two gates — the
count *and* adjacency to full support; the base passes neither — always
fed, spreading onto any empty matching hex in the region. So the base
is the free variable and every predator population is a determined
function of it, which is why "the base wins the long run" is a
structural guarantee, not a hope. Even in the worst case — a boxed-in
base whose only empty adjacent tile is the contested one — the predator
taking it merely *delays* one cube of base growth; the base can never
starve, so nothing cascades and no full tile is lost. Contention costs
the ecology nothing real, ever. Removal is likewise never contested: a
starving species always takes from its *own* emptiest tile, so there's
no choosing between species anywhere in the sweep.

**Consequences worth stating.** The shape is a literal pyramid in
tiles — the minimal apex chain is **3 full prey + 2 full mid + 1 apex
≈ 16 cubes across 6 tiles**, so viable patches want ~7–8 tiles and the
map lands medium, not small; that heft is deliberate, since the
Sacrifice's apex chains (a game-chosen subset of the five ecosystems)
are how the commune proves it transformed the map. Because Tier 1 always grows, the base constantly raises the
ceiling above it — prey sprawl funds predator height, hunting a
load-bearing tile lowers it. There is **no eat-prey logic and no
positive feedback loop**: predators never consume prey (the commune is
the only thing that eats, via hunting), and an apex you can't yet hunt
just occupies tiles — "predators overrun and collapse the ecosystem"
is structurally impossible. Any collapse rolls down only as far as its
cause pushed and stops at the largest self-sustaining sub-pyramid; one
surviving base cube can always rebuild. The only thing that takes a
region to zero is external — hunting, an event, an uncleansed disease
— so the real spiral risk is over-hunting your own base, as visible as
a full tile with an arrow pointed at it.

**Overgrowth has four brakes, at four prices:** (1) room — a saturated
region caps everything in it; (2) culling, tool-gated — mid needs the
mid tool, apex the rifle, so *you must be able to manage what you
cultivate*; (3) starvation, free but brutal — break a load-bearing
prey tile with the basic arrow and everything above chips down
indiscriminately; (4) containment — Processing a connecting tile
severs the terrain a species spreads through, a fence made of land.
(Diverted blight is the god's own pruning.) The table never lacks an
answer, only chooses which price.

**The sweep cost is the number of distinct colonies** (species-in-a-
region), one glance each — not tiles, not regions. Peak is a full
base+mid+apex pyramid × the number of developed ecosystems; the design
ceiling to hold is roughly **10–12 simultaneous colonies**, which the
win condition already implies (the Sacrifice's four apex chains mean
~4–6 real ecosystems, so bound the map to consolidate scattered pockets
into that handful rather than two dozen dots). Two properties keep it
light: young pockets are one trivial touch and saturated pyramids are
skipped, so the realistic mid-game peak is ~8–10, and it *decreases*
late as regions saturate — the opposite of most engine games.

**Regions are independent, so the world phase parallelizes.** Nothing
crosses a region border, so there's no cross-region ordering: at a full
table, each player grabs a different ecosystem and resolves it
simultaneously — upkeep divides by player count instead of one person
sweeping everything while others watch. The board with the most regions
is the board with the most hands. Watch item: if a single region's
sweep still drags, the levers are a slower Tier-1 cadence or only
resolving regions that changed — levers, not new rules.

## Two economies, spent differently

They split along one line, and the whole design leans on it: **the profane
is personal, the sacred is communal.** What a civilian earns with their own
hands and their own walking is theirs; what's offered at the altar belongs
to everyone and never comes back out. The commune shares its land, its
fate, and its god — not your wallet.

**Trade** comes from harvesting: converting part of the commune's current
population into goods, then selling them. **Prices are fixed constants**,
printed once on a reference card by tier × terrain — no tracks, no
adjustment, no market upkeep of any kind. (An earlier draft had price
elasticity — dumping a good dropped its price, recovering each round —
as the tax on leaning on the easiest source. Cut: two moving tracks of
upkeep doing a job the growth model now does better, because
over-extraction's real cost lives on the map — stripping cubes from one
species breaks its full tiles and starves everything above them. The
punishment for monoculture selling moved from the market to the
ecology, where it's visible.) Income can afford occasional gaps: no
upkeep exists anywhere — nothing drains money on a timer — so a bad
round stalls the shopping, never cascades.

Money is **personal**, not pooled. Each civilian keeps their own wallet;
two meeples at the same building can hand money off freely, the same rule
as Tool Exchange — the real cost is the Walks it took to meet, so the
transfer itself stays free. Basic actions (Walk, basic harvesting, praying,
replaying a Seed card already in your deck) never cost money — a broke
civilian is poor, not paralyzed. Personal wallets also make the big
communal buildings a scene: a second Town Center is deliberately priced
past any one wallet, so affording one is a literal fundraiser — the
commune converging on one building, purses out.

Personal money exists for a structural reason, not just flavor: under
simultaneous planning (see Turns and table talk below), a shared pool
creates collisions — two civilians secretly committing to spend the same
money — and any tiebreak reintroduces exactly the turn order simultaneity
removed. Personal purses make secret commitments collision-free by
construction.

Cards are bought from **suited piles**, not one shared visible row: choose
a pile publicly (the suit taxonomy — Harvest, Devotion, Logistics, Wilds,
or similar — still needs pinning down), then privately draw three from it
and keep one. The table plans at the level of what the commune needs
("someone should go deep on Devotion before the Cataclysm window"), and
everyone can see which suits a civilian keeps buying — specialization
stays legible — but nobody can reach into another player's draw and pick
for them. Decks start similar and diverge fast.

Base sell price should track how hard-won a species' habitat actually is,
not be an arbitrary list — Savanna-tied animals sell cheapest, since
Processing already makes Savanna the cheapest terrain to create; anything
tied to Forest carries a premium, since restoring land there costs more
than degrading toward Savanna does. Trade's prices and Processing's costs
reinforce the same idea instead of being two disconnected numbers.

**Favor** comes from praying, and it's sacred rather than transactional —
the god is unambiguously on the commune's side, never the source of what's
hurting it. All prayer feeds one single communal pool — no private
stockpile, no separate pools for separate uses.

The pool is physically **the Shrine**: a small box (an actual cardboard
shrine standing on the table) with a slot in the top. Offering values live
on the tokens, not the cards: the token supply is a shuffled, face-down
pool of mixed values (1s, 2s, 3s), and a Pray card only ever says how
*many* tokens to slide in, unseen — a basic Pray inserts one, Devotion
upgrades insert more or add riders. Nobody, including the one praying,
looks at what went in. **Nobody ever knows how much Favor the commune has
— not hidden by rule, uncountable by design, from the very first prayer
of the game.** How can you count a god's favor? Even the one who prays
doesn't know what their prayer weighed. The token supply is finite and
sits visibly by the board (around ten tokens as a starting guess), so the
table always knows how *many* offerings are inside — never what they're
worth. That's the coarse "should we keep praying" signal, and when the
supply runs dry the Shrine is visibly full.

(A rejected earlier version printed values 1–3 on the Pray cards
themselves. It made the pool countable early — identical starting decks
mean all-known values — and its uncertainty depended on how many *other*
players' hidden contributions surrounded you, which collapses solo.
Values-on-shuffled-tokens works identically at every player count. One
optional spice, untested: a couple of zero-value tokens in the pool —
unheard prayers. Thematically delicious, potentially feels-bad; start
without them.)

Spending is **blind-draw payment**: draw tokens from the Shrine one at a
time, flipping each, until the cost is met. Overshoot is burned — **the
god does not make change.** A consumed offering never comes back out,
which is the sacred-is-communal principle enforced physically. Event
responses degrade gracefully: drawing toward a Divert that comes up short
still buys a Block if that cheaper cost was met — the god heard, just not
enough for the bigger ask. (An earlier version let whoever stood at the
altar peek at an exact maintained total — a hidden dial, an emergent
priest role. Cut: blind-draw makes the count not exist at all, which is
less rules text and more mystery, and the counting chore it solved
disappeared with it.)

One honest bend this puts on the Blessings principle below: a blessing's
*effect* is still guaranteed whenever the pool suffices, but its
*efficiency* isn't — a cheap blessing can eat a fat token. The gamble is a
tip jar, not a coin flip; accepted.

Favor funds two categories:

**Godly events** — how the commune responds when an event threatens a
colony. Not an ownership question — since no colony belongs to any one
civilian, the response is decided by whoever holds the rotating **Elder**
token when the event fires (see Turns and table talk below):
- **Block** it outright — cheap, safe, but the opportunity's wasted, nothing
  changes either way.
- **Divert** it toward a different location instead — costs more than
  Block, since it's the bigger ask: choosing where the world's tension
  releases rather than just refusing it. The location has to be somewhere
  the event could plausibly happen (same coherence rule as everything
  else — a diverted flood still needs to land near existing water), but
  within that, it's a real choice.
- **Let it happen** — free, where it already is, full gamble.

There's no separate "heal the land" ability — Divert and Let it happen are
the *only* ways the commune positively reshapes the map, both by pointing
the world's own volatility somewhere useful rather than somewhere harmful,
gradually connecting fragmented land into stretches big enough to support
new habitat. Divert gives control over location at a cost; Let it happen
gives up that control for free, but since width is unknown either way, an
uncontrolled event can sometimes sprawl out and connect far more matching
terrain than anyone would've dared aim for with a costlier, deliberate
Divert — sometimes the better move is to let nature do its own thing and
hope.

No matter which is chosen, **intensity always stays unknown** — one single
number that determines both how strong an event is and how wide it spreads
(width is derived directly from intensity, not rolled separately). A
diverted or unblocked event could turn out small and contained, or big and
spreading well past where anyone meant it to. That uncertainty never goes
away, which is what keeps this "you vs. nature" instead of "you command
nature": Favor buys influence over whether and where, never over how much.

An event resolves **immediately and once** — the moment it triggers, its
intensity and full affected area are determined in a single step. Nothing
spreads or develops over following turns; every other system here (Trade,
Favor) resolves instantly too, and events stay consistent with that. The
one deliberate bend: blight's *event* still resolves in a single step,
but it leaves a disease marker — a lingering **condition** the world
phase processes each round (see Events). The principle bans incoherent
multi-turn sprawl, not persistent board state.

**Blessings** — the other thing Favor funds. Broadly, anything that
accelerates or protects effort the commune's already making, never
population or terrain handed out directly:
- **Expedite** — favor a colony's next growth pass.
- Other tactical blessings — clearing a disease/affliction from a colony,
  boosting the price goods sell for, and similar small edges. Not a fixed
  list, just the shape: help something already in motion go better.
- **Buy** — spend Favor on cards exclusive to devotion, not purchasable
  with Trade.

Blessings stay small, local, and reliable — spend Favor, get the stated
edge; the only gamble is blind-draw efficiency (see the Shrine above),
never whether the effect lands. That's deliberately different from events and
Volcanoes, where intensity, timing, and location stay permanently outside
anyone's control no matter how developed the commune's Blessing options
get. Reliable tactical help and genuine divine unpredictability are two
different things sharing the same economy, not one softening into the
other.

## The deck

No card is just "money" — currency only ever comes from playing the board,
never from a card that produces it out of thin air. That keeps every deck
full of things a civilian *does*.

One unified hand, not separate systems. Harvest, Sell, Pray, Process, Buy,
and playing a Seed/Blessing/Tactical card all draw from the same hand
and the same per-turn play budget — a civilian draws a hand (4-5 cards) and
can only play a few of them (2-3, exact number still tuning) each turn.
Walk sits outside the play budget but inside the same hand: it's paid by
discarding any card (see Buildings), so hand size is also the walk budget.
That makes a full harvest-and-cash-in cycle — Walk out, Harvest, Walk back,
Sell — a real budget decision on its own, not a free sequence squeezed in
around "real" plays. Exchange stays the one exception, deliberately free
(see Buildings and Tools below) since there's no decision left to price once
two meeples are already at the same place.

Every card works like a normal deck-builder card, not a one-shot purchase:
pay once to add it to your deck, then it cycles through draw, play, discard,
and reshuffle like everything else, coming back around to be played again
rather than disappearing after first use. A replayed Seed card just plants
a separate new colony elsewhere — still the same small seed value each
time, still has to survive growth on its own, so this doesn't open a
back door around the no-free-growth rule.

- **Seed cards** — plant a small starting population of a species, gated by
  whether the right habitat exists yet, priced higher for stronger species.
- **Blessing cards** — Expedite and Buy, the prayer-funded effects described
  above. Terrain no longer heals through a card at all — that's Divert and
  Let it happen's job, triggered in response to an actual event, not bought
  ahead of time.
- **Tactical cards** — small, temporary edges that never add population or
  territory directly: an extra draw, easing a placement restriction once,
  locking in a good sale price. Praying and Blocking are both these; Walk
  is not a card at all — it's the base action paid by discarding any card
  (see Buildings).

**What's deliberately not in the deck: Buildings and Tools.** Cards
represent repeatable actions, drawn fresh and chosen each time. Buildings
(House, Altar, Town Center) and Tools (rifle vs. the default arrow, boat,
outpost, backpack) represent one-time investments that permanently upgrade
what an action can already do — nobody redraws a House to keep its range,
and nobody redraws a rifle to keep hunting with it. Both are bought with
Trade or Favor same as anything else, they just never enter the deck at
all. Not every Tool upgrades an action either — a Backpack upgrades
carrying capacity instead, letting one meeple hold multiple Tools at once
rather than needing an Exchange relay to move them individually.

Buildings and Tools aren't the same kind of thing, though. A Building is
fixed infrastructure — it extends range from wherever it's placed, full
stop. A Tool is a physical object that can only be in one place at a time,
carried by whichever civilian's meeple bought or is currently holding it —
not communally available everywhere at once the way a Building's range is.
**Exchange** moves a Tool between civilians: two meeples at the same
building can hand it off, and it should probably be free rather than
costing its own action — the real cost already lives in the Walk actions
it takes to get two meeples to the same place, so taxing the hand-off too
would just be friction on top of a decision that's already been priced,
without adding a new one. That makes getting the right tool to the right
place at the right time — the rifle actually reaching whoever's standing
at the frontier when the apex predator's ready — a real logistics problem
that lives entirely in positioning, not in the exchange itself.

This only applies to purchased upgrade tools, though — basic tools (the
default arrow) are baseline capability every civilian has everywhere, no
carrying or Exchange needed. The logistics puzzle only shows up for the
scarce, bought tier worth making a fuss over, not for routine actions.

## What the supply looks like on the table

Three purchase structures, and they form a deliberate agency gradient —
the more sacred the economy, the less you get to choose:

- **The infrastructure display** — Buildings (House, Altar, Town Center)
  and Tools (rifle, boat, backpack, outpost), face-up, always visible,
  bought with Trade, never entering any deck. Full information, full
  agency: infrastructure is engineering, not fortune. (A Road has been
  floated as an addition here — presumably cheapening or waiving Walk's
  discard along a route — but it has no rule yet.)
- **Trade card piles, suited** — publicly pick a pile, privately draw
  three, keep one. Partial agency: you choose the kind of card, fortune
  picks the option. Seed/animal cards are suited by **food-web tier**
  (primary / secondary / tertiary), so a buyer knows which rung of the
  ecosystem they're buying into but not which species or terrain they'll
  get — you might want a Savanna grazer and draw a Forest one.
  Walk-as-discard keeps the miss from being a dead card, and adapting
  plans to what the draw actually gave you is the point, not a bug.
- **The Blessing deck, fully closed** — Favor-bought cards are drawn
  blind, sight unseen. No agency at all, and that's the theology again:
  grace is received, not chosen. Trade lets you choose the pile; Favor
  doesn't even show you the card. Contents are buffs to effort already
  underway — growth boosts, disease cleansing, market-rate edges,
  prayer-efficiency improvements — never raw Favor, population, or
  terrain directly, per the no-free-growth rule.

## Turns and table talk

Civilians don't take turns in sequence — each round, everyone plans
**simultaneously and secretly**, committing their plays face-down, then
commitments resolve in seat order. That kills downtime at four or five
players, and it's the strongest anti-alpha structure there is: nobody can
direct a turn that's already locked in.

Hands are **hidden**, and table talk is legal only at the level of intent:
"I'm heading to the east patch," "I've got the frogs covered" — never
naming or counting the cards in a hand. Without that one restriction,
reading hands aloud is always optimal and the hiddenness evaporates. The
same line runs through every system here — **public at the level of
intent, private at the level of execution**: hidden hands with open
intentions, a Shrine whose offering count shows but whose worth doesn't,
market piles chosen publicly and drawn privately, personal wallets in a
shared market. Coordination stays real precisely because the map
punishes failing at it — two civilians who secretly converge on the
same colony strip its load-bearing tile together, which is the game
teaching the table to talk.

Simultaneous rounds need a different event trigger than "every 4 player
turns": an **event track** advances by the number of players at the end of
each round and fires an event each time it crosses 4 — the same
self-scaling math, batched to round boundaries. Large tables can bank two
events in one break; busy world, busy god. A rotating **Elder** token
decides each event's response (Block, Divert, or let it happen) — rotation
keeps the spotlight fair, and with an uncountable Shrine the Elder's call
is genuinely theirs to make.

## A round, start to finish

1. **Draw** — everyone refills to hand size from their own deck.
2. **Plan** — simultaneous and secret; each player commits 2–3 plays
   face-down. Intent-level talk only.
3. **Resolve** — in seat order: Walks (burning discards), hunts, hauls,
   sales, prayers, purchases, seeds. All player-side changes happen
   here and only here.
4. **World phase** — the upkeep, once per round: one top-down sweep
   of the wild (each species grows or shrinks by one cube; Tier 1
   always grows if roomy), then Chip, then Event track (Elder
   responds if one fires).
5. **Cleanup** — played cards to discards, Elder token passes
   clockwise.

Players act only in step 3; the world acts only in step 4 — no
interleaving. The upkeep contains no economy: nothing feeds, rots, or
pays rent. The only things maintained are on the map, which is where a
stewardship game wants its maintenance.

## Player count and scaling

Decks scale with players; meeples and the map scale with the world. Every
player runs exactly one deck, always — at low counts a player controls
extra **meeples** with that single deck, drawing one hand and assigning
each play to whichever of their bodies is positioned for it. Solo is one
deck driving two or three meeples: one mind orchestrating a small
commune, logistics-heavy and quiet, not a degraded copy of the table
game. Two players run three or four meeples between them. The base design
point is four meeples.

The rule that keeps solo and full-table play from drifting apart: **key
every rule to meeples and decks, never to players.** The word "player"
appears in setup and nowhere else — "another meeple at your building,"
never "another player." The engine is already count-invariant (growth,
events, price elasticity, the Shrine are all system-side); player count
only turns quantity knobs — map size, meeple count, Sacrifice mix. What
solo honestly loses is the social layer (hidden hands, intent-only talk,
market collisions, the Elder's judgment call) — that layer exists to
manage information between humans and shouldn't be simulated for one.

## Specialization without assigned roles

Every civilian has access to the same actions, the same market, and the
same Shrine — nobody is handed a job. **No assigned roles of any kind,
including starting tilts**: starting decks are fully identical (drafted
backgrounds were floated and rejected), so every path is earned at the
market. Paths are publicly legible — pile purchases are visible, so the
table knows who's been buying Devotion — but they stay social, never
mechanical: no badge, title, or path bonus ever appears in the rules,
because the moment the game confirms a job, expectations harden and the
role-policing that emergent specialization avoids comes back. "You've
become our priest" is something the table says, never the rulebook. What makes people diverge is which cards
they personally keep choosing to buy. Keep buying harvesting upgrades and
you become the commune's de facto harvester; keep buying prayer-boosting
cards and you become the one everyone leans on before a big terraforming
push. The group ends up covering different needs naturally, without anyone
being told to.

## Goals

Act 1's shared floor — population and a connected corridor, together — is
still a real prerequisite: it needs to hold before the commune can support
the diversity the actual win condition asks for.

The actual win condition is **Sacrifice** — offering apex predators at the
Shrine (see Events above), all at once, to earn the ultimate favor of the
gods and avert the Cataclysm. Not sold for Trade, not a resource — a
ritual, funded by Favor same as everything else sacred here. The map generates **five ecosystems: three homeland (Forest, River,
Savanna) and two frontier (Mountain, Ocean).** ("Homeland vs frontier"
is the terrain class, kept distinct from the trophic prey/mid/apex
tiers *inside* each ecosystem; earlier drafts called both "Tier" and
they must be disambiguated — see the naming pass.) But the Sacrifice
demands only a **game-decided subset — 2 of the 3 homeland apexes plus
1 of the 2 frontier apexes (either Mountain or Ocean), three predators
offered at once** — which is what keeps the endgame a genuine
unpredictable mix rather than a fixed checklist.

The twist lives in the terrain you can't touch. The homeland is
controllable — Process it, steer events across it, engineer those
pyramids — so "2 of 3 homeland" is mild variability among things you
can plan. The frontier is uncontrollable by rule: Mountain and Ocean
can't be Processed, only the Cataclysm reaches them, and which one
turns out correct isn't something the commune engineers. So the single
frontier apex in the mix is the wildcard the whole endgame bends
around — you can prepare five ecosystems' worth of ambition, but the
god decides whether it's the mountain or the ocean predator that must
lie on the altar, and you learn it too late to have prepared only for
that one. That forces a real hedge: build lean toward a guess and risk
the reveal, or over-develop for safety and spend the extra effort.
Apex is the hardest rung on any food web, so clearing even three at
once is a test of whether the commune mastered the map, not a lucky
snowball.

Performing the Sacrifice requires the **Shrine**, built at the correct
Mountain/Ocean patch — not a separate building from what averts the
Cataclysm, the same one. Building it is part of the commune's real endgame
prep, not something that happens for free once the apex predators are
ready.

No individual ranking either way — personal identity comes through
specialization (what a civilian keeps choosing to buy), not a formal goal
layered on top. A private-goal layer was floated and cut: open titles
weren't actually secret since the actions behind them are visible on a
shared board; hidden numeric counts weren't a real mission and didn't scale
with player count, since whether you personally get to act on a given
event is partly turn-order luck, not skill.

**Win/lose stays binary, but a score on top captures how well, not just
whether.** Speed (rounds used), diversity beyond the required minimum,
Temples found along the way, how large or healthy the correct patch's
ecosystem got — none of it affects whether the commune actually won, it
just gives a completed game something to compare against other
playthroughs. Not hidden information anyone tracks live, just a tally at
the end.

## Component: mutable terrain

The whole game reshapes the map, so terrain representation is
load-bearing. The problem looks hard because cubes sit on the tiles —
but **one rule dissolves it: any terrain change kills whatever lives on
that tile.** A species is tied to its terrain, so the moment a hex
changes biome, its cubes return to the supply. That means the physical
operation is never "juggle live cubes across a swap" — it's *clear the
tile (the animals die), then place the new terrain.* The tile is always
empty at the instant it changes, so both representations are trivial:

- **Overlay chits on a printed base map** — the cheap prototype. On
  change, remove the old chit, drop the new one; cubes were already
  swept off. Dirt cheap, right for the paper playtest.
- **Modular hex tiles in a recessed tray** — the real-edition version.
  Same clear-then-swap operation, but the new tile seats flush so the
  *next* pyramid grows on a stable surface. Leaning here for the final
  form.

The encroachment rule makes every change a bounded, Pandemic-outbreak
operation — a flood is "take Wetland tiles, place on land touching
existing water, count = intensity" — never free-form editing. Two
payoffs of clear-on-change beyond easy components: it makes destruction
*dramatic* (a flood is you sweeping a whole pyramid off the board by
hand — the tragedy is physical, matching "events are always
negative"), and it gives Processing real teeth (converting a tile clears
it, so you can't terraform land your own ecosystem stands on without
killing part of it — which pushes Processing to the frontier, exactly
where encroachment already wants it). Mountain/Ocean should be a
physically chunkier, heavier component that can't be swapped, teaching
permanence by feel; the inert Sea buffer is the fixed underlying layer.
(The digital edition trivializes all of this — a tile type is just a
variable.)

## Floated, not yet decided

Pitched in the 2026-07-23 session, neither adopted nor rejected:

- **Cataclysm variants** — several scenario personalities (drowning world,
  burning world, blighted world…), each skewing the event mix and the
  Sacrifice's demands. The proven replayability lever, and the Cataclysm's
  theming is undecided anyway.
- **An Omen queue** — a face-down queue of upcoming events Favor can pay to
  peek at; scouting the future as a Favor sink. Intensity stays unknown.
- **Blessing Aspects** — blessings split into aspects of the god, only 2–3
  in play per game, chosen at setup.
- **Public vows** — a civilian declares a self-imposed constraint aloud and
  earns Favor for keeping it; personal identity without hidden scoring.
- **Renewal** — regrowth priced cheaper on recently-disturbed land (fire
  ecology, floodplain fertility) as comeback texture. Flagged: bends the
  "events have no positive spin" rule; unresolved.

## Open questions

- How many Mountain/Ocean patches actually get generated on the map, and
  how is "the correct one" actually determined — random at the moment the
  Cataclysm is diverted, or something the commune can influence/scout for
  in advance via Temples?
- The **event track** (see Turns and table talk) replaces the old
  every-4-player-turns event die, but the same playtest question stands:
  events land more frequently relative to *personal* progress at a small
  table than at a large one even though the absolute rate is identical.
  Also still open: does how neglected the map is affect the odds, or is it
  a flat roll?
- Total round count for a game isn't set yet, and the Cataclysm's
  likelihood needs it as an anchor: probability should climb over time but
  be *guaranteed* by some checkpoint (roughly two-thirds through, as a
  starting guess) so a real tail of rounds is always reserved for Act 2.
  Actual numbers need playtesting, not paper math.
- Does the Cataclysm's intensity come from a strictly higher floor than an
  ordinary event, or just a wider random band that happens to skew bigger?
  Undecided.
- Exact win check for the apex predator: does reaching population 2+ alone
  finish the game, or does it specifically require killing/harvesting at
  least one with the right tools? Still not decided.
- The Trade market's exact suit taxonomy, how many piles, and how big the
  face-up infrastructure display is.
- Offering token count and value mix, plus Block/Divert costs tuned
  against blind-draw overshoot — expected waste should stay around a
  single point.
- Does Walk-by-discard need a per-round cap, or is hand size a natural
  enough limit?
- What a Road actually does. Two candidate models: roads as tempo (a Road
  connects two buildings; Walking between them costs no discard — the
  reward for a route mattering) versus roads as permission (distant
  Houses require a road connection to build — a fourth tax on expansion,
  which risks overpricing the emergency reach the Cataclysm demands).
  Current lean: tempo.
- A naming pass on the sacred objects: the offering box, the Altar
  building, Temple discoveries, and the endgame Shrine are four different
  holy nouns that currently blur together.
- How fragmented can the starting map be before even a basic foothold
  becomes unreachable and the economy can't bootstrap at all? Bounded
  now from the other side too: the world-phase sweep wants a design
  ceiling of ~4–6 developed ecosystems (~10–12 colonies), so the map
  should let scattered starting pockets *consolidate* into that handful
  via Divert rather than persist as many independent dots each needing
  their own sweep. Fragmented at the start, consolidated by mid-game.
- What's the actual radius a building covers, and does House/Altar range
  stack or overlap when built close together?
