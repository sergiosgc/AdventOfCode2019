use std::{ops::{Add, Neg, Sub}};

pub enum CardinalPoint {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64) -> Coord { Coord{ x, y } }
    pub fn neighbours(self) -> Vec::<Coord> {
        vec![
            self + Coord { x: 0, y: -1},
            self + Coord { x: -1, y: 0},
            self + Coord { x: 1, y: 0},
            self + Coord { x: 0, y: 1},
            self + Coord { x: -1, y: -1},
            self + Coord { x: 1, y: -1},
            self + Coord { x: -1, y: 1},
            self + Coord { x: 1, y: 1},
        ].to_vec()
    }
    pub const fn from_cardinal(cardinal: CardinalPoint) -> Coord {
        match cardinal {
            CardinalPoint::N => Coord { x: 0, y: -1},
            CardinalPoint::NE => Coord { x: 1, y: -1 },
            CardinalPoint::E => Coord { x: 1, y: 0 },
            CardinalPoint::SE => Coord { x: 1, y: 1 },
            CardinalPoint::S => Coord { x: 0, y: 1 },
            CardinalPoint::SW => Coord { x: -1, y: 1 },
            CardinalPoint::W => Coord { x: -1, y: 0 },
            CardinalPoint::NW => Coord { x: -1, y: -1 },
        }
    }
}
impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Neg for Coord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x, 
            y: -self.y,
        }
    }
}