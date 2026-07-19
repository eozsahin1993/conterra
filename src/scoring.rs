use crate::balance::{
    DEFAULT_MIN_POPULATION, GROUP_THRESHOLD_MIN_CONTIGUOUS_TERRAIN_CHAIN,
    GROUP_THRESHOLD_MIN_TOTAL_POPULATION, SOCIAL_SPECIES_MIN_POPULATION,
};
use crate::board::Board;
use crate::species::{self, Species};

/// Minimum viable population threshold (content doc: "a species' relationships
/// only count toward scoring once its current on-map count meets a
/// threshold"). Scoring-side only — placement itself stays unrestricted.
pub fn meets_minimum_population(board: &Board, species: Species) -> bool {
    let threshold = if species::is_social_species(species) {
        SOCIAL_SPECIES_MIN_POPULATION
    } else {
        DEFAULT_MIN_POPULATION
    };
    board.animal_count(species) as u32 >= threshold
}

pub struct GroupThresholdStatus {
    pub met: bool,
    pub total_population: u32,
    pub longest_corridor: usize,
}

/// The one shared group threshold gating whether anyone's secret objective
/// counts at all (brief: "likely a dual condition"). Both a total-population
/// floor and a contiguous-terrain-corridor floor must hold.
pub fn group_threshold_status(board: &Board) -> GroupThresholdStatus {
    let total_population: u32 = Species::ALL.iter().map(|&s| board.animal_count(s) as u32).sum();
    let (_, longest_corridor) = board.longest_terrain_corridor();
    GroupThresholdStatus {
        met: total_population >= GROUP_THRESHOLD_MIN_TOTAL_POPULATION
            && longest_corridor >= GROUP_THRESHOLD_MIN_CONTIGUOUS_TERRAIN_CHAIN,
        total_population,
        longest_corridor,
    }
}
