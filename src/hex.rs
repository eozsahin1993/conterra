//! Axial hex-coordinate math. See https://www.redblobgames.com/grids/hexagons/
//! for the reference derivations used here.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn new(q: i32, r: i32) -> Self {
        Hex { q, r }
    }

    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    pub const DIRECTIONS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

    pub fn neighbor(&self, dir: usize) -> Hex {
        let (dq, dr) = Self::DIRECTIONS[dir % 6];
        Hex::new(self.q + dq, self.r + dr)
    }

    pub fn neighbors(&self) -> [Hex; 6] {
        let mut out = [Hex::new(0, 0); 6];
        for i in 0..6 {
            out[i] = self.neighbor(i);
        }
        out
    }

    pub fn distance(&self, other: &Hex) -> i32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s() - other.s()).abs()) / 2
    }

    /// All hexes within `radius` of the origin — used to build the generously-sized
    /// shared board (brief: "generous grid size as the primary lever against
    /// placement difficulty").
    pub fn spiral_from_origin(radius: i32) -> Vec<Hex> {
        let mut out = Vec::new();
        for q in -radius..=radius {
            let r1 = (-radius).max(-q - radius);
            let r2 = radius.min(-q + radius);
            for r in r1..=r2 {
                out.push(Hex::new(q, r));
            }
        }
        out
    }
}

/// A random connected cluster of `size` hexes, chain-grown from a seed hex.
/// Brief: "procedurally-generated shapes ... random connected hex clusters,
/// chain-grown from a seed hex, for compactness."
pub fn grow_shape(size: usize, rng: &mut impl rand::Rng) -> Vec<Hex> {
    let mut shape = vec![Hex::new(0, 0)];
    let mut frontier: Vec<Hex> = Hex::new(0, 0).neighbors().to_vec();
    let mut in_shape: HashSet<Hex> = shape.iter().copied().collect();

    while shape.len() < size && !frontier.is_empty() {
        let idx = rng.gen_range(0..frontier.len());
        let candidate = frontier.swap_remove(idx);
        if in_shape.contains(&candidate) {
            continue;
        }
        in_shape.insert(candidate);
        shape.push(candidate);
        for n in candidate.neighbors() {
            if !in_shape.contains(&n) {
                frontier.push(n);
            }
        }
    }
    shape
}

/// Rotate a hex offset by `steps` * 60 degrees around the origin (axial rotation).
pub fn rotate(hex: Hex, steps: i32) -> Hex {
    let mut q = hex.q;
    let mut r = hex.r;
    let mut s = hex.s();
    for _ in 0..(steps.rem_euclid(6)) {
        let (nq, nr, ns) = (-r, -s, -q);
        q = nq;
        r = nr;
        s = ns;
    }
    debug_assert_eq!(q + r + s, 0);
    Hex::new(q, r)
}
