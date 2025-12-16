use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Copy + From<u8> + Ord> Vec2<T> {
    pub fn zero() -> Vec2<T> {
        Vec2::new(T::from(0), T::from(0))
    }

    pub fn one() -> Vec2<T> {
        Vec2::new(T::from(1), T::from(1))
    }

    pub fn max(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn min(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn in_bounds(self, min: Self, max: Self) -> bool {
        self.x >= min.x && self.y >= min.y && self.x <= max.x && self.y <= max.y
    }
}

impl Vec2<i64> {
    pub fn abs(&self) -> Self {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    pub fn manhattan_distance(&self) -> i64 {
        let abs = self.abs();
        abs.x + abs.y
    }

    pub fn all_dirs() -> Vec<Vec2<i32>> {
        vec![
            Vec2::new(1, 0),
            Vec2::new(-1, 0),
            Vec2::new(0, 1),
            Vec2::new(0, -1),
        ]
    }

    pub fn dot(&self, other: &Self) -> i64 {
        self.x * other.x + self.y * other.y
    }
}

// impl<T: Copy + Add + Mul> Vec2<T> {
//     pub fn len(self) -> T
//     where
//         <T as Mul>::Output: Add,
//     {
//         self.x * self.x + self.y * self.y
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sub() {
        let v0 = Vec2::new(10, 5);
        let v1 = Vec2::new(3, 2);
        assert_eq!(Vec2::new(7, 3), v0 - v1);
    }
}
