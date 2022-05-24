use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};

#[derive(
    Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Inspectable,
)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

// We want to be able to sum coordinates
impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u16;
        let y = ((self.y as i16) + y as i16) as u16;
        Self { x, y }
    }
}
