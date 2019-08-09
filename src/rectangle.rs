use crate::point::Coordinate;
use crate::point::Point;
use std::fmt;

#[derive(Clone)]
pub struct Rectangle<T: Coordinate> {
    pub x0: T,
    pub y0: T,
    pub x1: T,
    pub y1: T,
}

impl<T: Coordinate> Rectangle<T> {
    pub fn new(x0: T, y0: T, x1: T, y1: T) -> Self {
        Rectangle { x0, y0, x1, y1 }
    }

    pub fn as_tuple(&self) -> (T, T, T, T) {
        (self.x0, self.y0, self.x1, self.y1)
    }

    pub fn center(&self) -> Point<T> {
        Point {
            x: (self.x0 + self.x1) / 2.into(),
            y: (self.y0 + self.y1) / 2.into(),
        }
    }
}

impl<T: Coordinate> fmt::Debug for Rectangle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(({}, {}) - ({}, {}))",
            self.x0, self.y0, self.x1, self.y1
        )
    }
}

#[cfg(test)]
mod test {
    use super::Rectangle;

    #[test]
    fn center_int() {
        let r = Rectangle::new(0, 10, 10, 15);
        let (xh, yh) = r.center().as_tuple();
        assert_eq!(xh, 5);
        assert_eq!(yh, 12);
    }

    #[test]
    fn center_float() {
        let r = Rectangle::new(0.0, 10.0, 10.0, 15.0);
        let (xh, yh) = r.center().as_tuple();
        assert_eq!(xh, 5.0);
        assert_eq!(yh, 12.5);
    }

    #[test]
    fn print_int() {
        let r = Rectangle::new(0, 10, 10, 15);
        assert_eq!("((0, 10) - (10, 15))", format!("{:?}", r));
    }
}
