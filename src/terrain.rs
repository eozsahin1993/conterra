use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Terrain {
    Forest,
    River,
    Ocean,
    Savanna,
    Mountain,
}

impl Terrain {
    pub const ALL: [Terrain; 5] = [
        Terrain::Forest,
        Terrain::River,
        Terrain::Ocean,
        Terrain::Savanna,
        Terrain::Mountain,
    ];
}
