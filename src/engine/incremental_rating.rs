use std::ops::{Add, Sub};

use crate::{moving::Move, piece::Side};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IncrementalRating {
    black_material: i64,
    white_material: i64,
    black_positional: i64,
    white_positional: i64,
}

impl IncrementalRating {
    pub fn set_material(&mut self, side: Side, val: i64) {
        match side {
            Side::White => self.white_material = val,
            Side::Black => self.black_material = val,
        }
    }
    pub fn set_positional(&mut self, side: Side, val: i64) {
        match side {
            Side::White => self.white_positional = val,
            Side::Black => self.black_positional = val,
        }
    }
    pub fn material(&self, side: Side) -> i64 {
        match side {
            Side::White => self.white_material,
            Side::Black => self.black_material,
        }
    }
    pub fn positional(&self, side: Side) -> i64 {
        match side {
            Side::White => self.white_positional,
            Side::Black => self.black_positional,
        }
    }

    pub fn sum(&self) -> i64 {
        self.black_material + self.white_material + self.black_positional + self.white_positional
    }
    // pub fn perspective_sum(&self, perspective: Side) -> i64 {
    //     self.sum() * if perspective == Side::White { 1 } else { -1 }
    // }

    pub fn sub_mut(&mut self, rhs: &Self) {
        self.black_material -= rhs.black_material;
        self.white_material -= rhs.white_material;
        self.white_positional -= rhs.white_positional;
        self.black_positional -= rhs.black_positional;
    }
    pub fn add_mut(&mut self, rhs: &Self) {
        self.black_material += rhs.black_material;
        self.white_material += rhs.white_material;
        self.white_positional += rhs.white_positional;
        self.black_positional += rhs.black_positional;
    }
}

impl Add for IncrementalRating {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            black_material: self.black_material + rhs.black_material,
            white_material: self.white_material + rhs.white_material,
            black_positional: self.black_positional + rhs.black_positional,
            white_positional: self.white_positional + rhs.white_positional,
        }
    }
}
impl Sub for IncrementalRating {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            black_material: self.black_material - rhs.black_material,
            white_material: self.white_material - rhs.white_material,
            black_positional: self.black_positional - rhs.black_positional,
            white_positional: self.white_positional - rhs.white_positional,
        }
    }
}
impl Default for IncrementalRating {
    fn default() -> Self {
        Self {
            black_material: 0,
            white_material: 0,
            black_positional: 0,
            white_positional: 0,
        }
    }
}
