use std::ops::Add;

use crate::decl_tokens_enum;

decl_tokens_enum! {
    Dir,
    // Order is important here
    ("front",      Front      ),
    ("frontright", FrontRight ),
    ("right",      Right      ),
    ("backright",  BackRight  ),
    ("back",       Back       ),
    ("backleft",   BackLeft   ),
    ("left",       Left       ),
    ("frontleft",  FrontLeft  )
}

impl Add for Dir {
    type Output = Dir;
    fn add(self, rhs: Self) -> Self::Output {
        match (self as usize + rhs as usize) % 8 {
            x if x == Dir::Front as usize => Dir::Front,
            x if x == Dir::FrontRight as usize => Dir::FrontRight,
            x if x == Dir::Right as usize => Dir::Right,
            x if x == Dir::BackRight as usize => Dir::BackRight,
            x if x == Dir::Back as usize => Dir::Back,
            x if x == Dir::BackLeft as usize => Dir::BackLeft,
            x if x == Dir::Left as usize => Dir::Left,
            x if x == Dir::FrontLeft as usize => Dir::FrontLeft,
            _ => unreachable!(),
        }
    }
}

impl Add<Dir> for &Dir {
    type Output = Dir;
    fn add(self, rhs: Dir) -> Self::Output {
        Dir::add(*self, rhs)
    }
}

impl Add<&Dir> for Dir {
    type Output = Dir;
    fn add(self, rhs: &Dir) -> Self::Output {
        Dir::add(self, *rhs)
    }
}