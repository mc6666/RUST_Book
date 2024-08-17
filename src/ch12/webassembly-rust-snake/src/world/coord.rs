use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn tuple_into_coord() {
        let a: Coord = (1, 10).into();
        assert_eq!(a, Coord { x: 1, y: 10 });
    }

    #[test]
    fn add() {
        let a = Coord { x: 1, y: 10 };
        let b = Coord { x: 2, y: 11 };
        assert_eq!(a + b, Coord { x: 3, y: 21 });
    }
}
