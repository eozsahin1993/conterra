//! The species roster and food web, transcribed directly from
//! `conterra-content.md`. Every predator->prey edge here is one of the
//! individually-sourced real relationships from that document; see its
//! change log for citations. This module is pure data plus small derived
//! lookups (terrain membership, tier) — no game logic lives here.

use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Species {
    // Forest
    Wolf,
    Bear,
    Boar,
    Coyote,
    Hawk,
    Adder,
    Squirrel,
    // River
    Otter,
    Alligator,
    Osprey,
    Bullfrog,
    Salmon,
    Crayfish,
    Shrimp,
    // Ocean
    Orca,
    GreatWhiteShark,
    Seal,
    Fish,
    SeaTurtle,
    Squid,
    Krill,
    Jellyfish,
    // Savanna
    Lion,
    Cheetah,
    Jackal,
    SecretaryBird,
    PuffAdder,
    Zebra,
    Gazelle,
    Hare,
    Gerbil,
    // Mountain
    GoldenEagle,
    SnowLeopard,
    RedFox,
    Raven,
    Goat,
    Marmot,
    Pika,
    SnowVole,
    Ibex,
}

impl Species {
    pub const ALL: &'static [Species] = &[
        Species::Wolf,
        Species::Bear,
        Species::Boar,
        Species::Coyote,
        Species::Hawk,
        Species::Adder,
        Species::Squirrel,
        Species::Otter,
        Species::Alligator,
        Species::Osprey,
        Species::Bullfrog,
        Species::Salmon,
        Species::Crayfish,
        Species::Shrimp,
        Species::Orca,
        Species::GreatWhiteShark,
        Species::Seal,
        Species::Fish,
        Species::SeaTurtle,
        Species::Squid,
        Species::Krill,
        Species::Jellyfish,
        Species::Lion,
        Species::Cheetah,
        Species::Jackal,
        Species::SecretaryBird,
        Species::PuffAdder,
        Species::Zebra,
        Species::Gazelle,
        Species::Hare,
        Species::Gerbil,
        Species::GoldenEagle,
        Species::SnowLeopard,
        Species::RedFox,
        Species::Raven,
        Species::Goat,
        Species::Marmot,
        Species::Pika,
        Species::SnowVole,
        Species::Ibex,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Species::Wolf => "Wolf",
            Species::Bear => "Bear",
            Species::Boar => "Boar",
            Species::Coyote => "Coyote",
            Species::Hawk => "Hawk",
            Species::Adder => "Adder",
            Species::Squirrel => "Squirrel",
            Species::Otter => "Otter",
            Species::Alligator => "Alligator",
            Species::Osprey => "Osprey",
            Species::Bullfrog => "Bullfrog",
            Species::Salmon => "Salmon",
            Species::Crayfish => "Crayfish",
            Species::Shrimp => "Shrimp",
            Species::Orca => "Orca",
            Species::GreatWhiteShark => "Great White Shark",
            Species::Seal => "Seal",
            Species::Fish => "Fish",
            Species::SeaTurtle => "Sea Turtle",
            Species::Squid => "Squid",
            Species::Krill => "Krill",
            Species::Jellyfish => "Jellyfish",
            Species::Lion => "Lion",
            Species::Cheetah => "Cheetah",
            Species::Jackal => "Jackal",
            Species::SecretaryBird => "Secretary Bird",
            Species::PuffAdder => "Puff Adder",
            Species::Zebra => "Zebra",
            Species::Gazelle => "Gazelle",
            Species::Hare => "Hare",
            Species::Gerbil => "Gerbil",
            Species::GoldenEagle => "Golden Eagle",
            Species::SnowLeopard => "Snow Leopard",
            Species::RedFox => "Red Fox",
            Species::Raven => "Raven",
            Species::Goat => "Goat",
            Species::Marmot => "Marmot",
            Species::Pika => "Pika",
            Species::SnowVole => "Snow Vole",
            Species::Ibex => "Ibex",
        }
    }
}

/// One `predator -> prey` edge, scoped to the terrain it's real in (a species
/// active in two terrains, like Hawk or Adder, can have different diets in
/// each — this is scoped per-edge rather than derived from a global diet list).
pub struct FoodWebEdge {
    pub terrain: Terrain,
    pub predator: Species,
    pub prey: Species,
}

macro_rules! edges {
    ($($terrain:expr => $pred:ident -> [$($prey:ident),+ $(,)?]),+ $(,)?) => {
        vec![
            $(
                $(
                    FoodWebEdge { terrain: $terrain, predator: Species::$pred, prey: Species::$prey },
                )+
            )+
        ]
    };
}

pub fn food_web() -> Vec<FoodWebEdge> {
    use Terrain::*;
    edges![
        // Forest
        Forest => Wolf -> [Boar, Coyote],
        Forest => Bear -> [Boar],
        Forest => Boar -> [Adder],
        Forest => Coyote -> [Squirrel],
        Forest => Hawk -> [Adder, Squirrel],
        // River
        River => Otter -> [Salmon, Crayfish],
        River => Alligator -> [Crayfish, Bullfrog],
        River => Osprey -> [Salmon],
        River => Bullfrog -> [Crayfish],
        River => Salmon -> [Shrimp],
        // Ocean
        Ocean => Orca -> [Seal, SeaTurtle],
        Ocean => GreatWhiteShark -> [Seal, SeaTurtle, Fish],
        Ocean => Seal -> [Fish, Squid],
        Ocean => Fish -> [Krill],
        Ocean => SeaTurtle -> [Jellyfish],
        // Savanna
        Savanna => Lion -> [Zebra, Jackal],
        Savanna => Cheetah -> [Gazelle, Hare],
        Savanna => Jackal -> [Hare],
        Savanna => SecretaryBird -> [PuffAdder, Gerbil],
        Savanna => PuffAdder -> [Gerbil],
        // Mountain
        Mountain => GoldenEagle -> [Goat, Adder, Hawk, Marmot, Pika, RedFox],
        Mountain => SnowLeopard -> [Ibex, Goat, Marmot],
        Mountain => RedFox -> [Pika],
        Mountain => Raven -> [SnowVole],
        Mountain => Adder -> [SnowVole],
    ]
}

/// Every terrain a species has at least one real edge (predator or prey) in.
/// Most species resolve to exactly one; Hawk and Adder resolve to two
/// (Forest + Mountain), per the roster's deliberate multi-terrain design.
pub fn species_terrains(edges: &[FoodWebEdge], species: Species) -> Vec<Terrain> {
    let mut set = HashSet::new();
    for e in edges {
        if e.predator == species || e.prey == species {
            set.insert(e.terrain);
        }
    }
    let mut out: Vec<Terrain> = set.into_iter().collect();
    out.sort_by_key(|t| format!("{:?}", t));
    out
}

pub fn prey_of(edges: &[FoodWebEdge], terrain: Terrain, predator: Species) -> Vec<Species> {
    edges
        .iter()
        .filter(|e| e.terrain == terrain && e.predator == predator)
        .map(|e| e.prey)
        .collect()
}

pub fn predators_of(edges: &[FoodWebEdge], terrain: Terrain, prey: Species) -> Vec<Species> {
    edges
        .iter()
        .filter(|e| e.terrain == terrain && e.prey == prey)
        .map(|e| e.predator)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// Never appears as prey anywhere on the roster.
    Apex,
    /// Both predator and prey somewhere on the roster.
    Mid,
    /// Never appears as a predator anywhere on the roster.
    Base,
}

pub fn tier(edges: &[FoodWebEdge], species: Species) -> Tier {
    let is_predator = edges.iter().any(|e| e.predator == species);
    let is_prey = edges.iter().any(|e| e.prey == species);
    match (is_predator, is_prey) {
        (true, false) => Tier::Apex,
        (true, true) => Tier::Mid,
        (false, true) => Tier::Base,
        (false, false) => unreachable!("every roster species has at least one edge"),
    }
}

/// Social/group-living species called out in the brief ("a single token of a
/// social/group-living species ... doesn't represent a viable population") —
/// placeholder list, not yet finalized (brief's Open Questions #6). The
/// brief's own examples (wolves, elephants, lions) minus Elephant, which was
/// cut from the roster entirely; Orca (pod-living) added as a clear real fit.
pub fn is_social_species(species: Species) -> bool {
    matches!(species, Species::Wolf | Species::Lion | Species::Orca)
}
