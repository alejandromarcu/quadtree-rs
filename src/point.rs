use num::Num;
use std::fmt;

pub trait Coordinate: Num + PartialOrd + Copy {}

impl<T> Coordinate for T where T: Num + PartialOrd + Copy {}

#[derive(Clone)]
pub struct Point<T: Coordinate> {
    x: T,
    y: T,
}

impl<T: Coordinate> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn as_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Coordinate + fmt::Display> fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn int_point() {
        let p = Point::new(5, 6);
        assert_eq!("(5, 6)", format!("{:?}", p));
    }

    #[test]
    fn float_point() {
        let p = Point::new(0.75, 1.4);
        assert_eq!("(0.75, 1.4)", format!("{:?}", p));
    }

    #[test]
    fn as_tuple() {
        let p = Point::new(3, 26);
        let (x, y) = p.as_tuple();
        assert_eq!(3, x);
        assert_eq!(26, y);
    }
}
