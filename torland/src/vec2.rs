use botc::code::Dir;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

pub type Vec2u = Vec2<usize>;
pub type Vec2i = Vec2<isize>;

#[derive(Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Copy> Copy for Vec2<T> {}

impl Vec2u {
    pub fn mod_add(&self, rhs: Vec2i, rem: Vec2u) -> Vec2u {
        Self {
            x: (self.x as isize + rhs.x).rem_euclid(rem.x as isize) as usize,
            y: (self.y as isize + rhs.y).rem_euclid(rem.y as isize) as usize,
        }
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}

impl<T, U: Into<T>> From<(U, U)> for Vec2<T> {
    fn from(value: (U, U)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl<T: From<isize>> From<Dir> for Vec2<T> {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Front => (0, -1),
            Dir::FrontRight => (1, -1),
            Dir::Right => (1, 0),
            Dir::BackRight => (1, 1),
            Dir::Back => (0, 1),
            Dir::BackLeft => (-1, 1),
            Dir::Left => (-1, 0),
            Dir::FrontLeft => (-1, -1),
        }
        .into()
    }
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
