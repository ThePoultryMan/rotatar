use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(from = "(i32, i32)", into = "(i32, i32)")]
pub struct TwoInts {
    x: i32,
    y: i32,
}

impl TwoInts {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl From<(i32, i32)> for TwoInts {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add for TwoInts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<TwoInts> for (i32, i32) {
    fn from(value: TwoInts) -> Self {
        (value.x, value.y)
    }
}
