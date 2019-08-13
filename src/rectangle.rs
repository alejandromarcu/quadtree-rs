use crate::point::Coordinate;
use crate::point::Point;
use std::fmt;

#[derive(Clone, PartialEq)]
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

    pub fn is_inside_of(&self, other: &Rectangle<T>) -> bool {
        self.x0 >= other.x0 && self.y0 >= other.y0 && self.x1 <= other.x1 && self.y1 <= other.y1
    }

    pub fn is_point_inside(&self, point: &Point<T>) -> bool {
        point.x >= self.x0 && point.x < self.x1 && point.y >= self.y0 && point.y < self.y1
    }

    pub fn overlaps(&self, other: &Rectangle<T>) -> bool {
        !(other.x1 <= self.x0 || other.x0 >= self.x1 || other.y1 <= self.y0 || other.y0 >= self.y1)
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
    use crate::point::Point;

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

    #[test]
    fn is_inside_of() {
        let r = Rectangle::new(0, 0, 10, 10);
        assert_eq!(true, r.is_inside_of(&Rectangle::new(0, 0, 10, 10)));
        assert_eq!(true, r.is_inside_of(&Rectangle::new(-5, -5, 15, 15)));

        assert_eq!(false, r.is_inside_of(&Rectangle::new(1, -5, 15, 15)));
        assert_eq!(false, r.is_inside_of(&Rectangle::new(-5, 1, 15, 15)));
        assert_eq!(false, r.is_inside_of(&Rectangle::new(-5, -5, 9, 15)));
        assert_eq!(false, r.is_inside_of(&Rectangle::new(-5, -5, 15, 1)));
    }

    #[test]
    fn overlaps() {
        let r = Rectangle::new(0, 0, 10, 10);

        // each one is on one side of the rectangle
        assert_eq!(false, r.overlaps(&Rectangle::new(-5, -5, 15, 0)));
        assert_eq!(false, r.overlaps(&Rectangle::new(-5, -5, 0, 15)));
        assert_eq!(false, r.overlaps(&Rectangle::new(10, -5, 15, 15)));
        assert_eq!(false, r.overlaps(&Rectangle::new(-5, 10, 15, 15)));

        // similar to the above but it overlaps a bit
        assert_eq!(true, r.overlaps(&Rectangle::new(-5, -5, 15, 1)));
        assert_eq!(true, r.overlaps(&Rectangle::new(-5, -5, 1, 15)));
        assert_eq!(true, r.overlaps(&Rectangle::new(9, -5, 15, 15)));
        assert_eq!(true, r.overlaps(&Rectangle::new(-5, 9, 15, 15)));

        // r is inside this one
        assert_eq!(true, r.overlaps(&Rectangle::new(-5, -5, 15, 15)));

        // inside r
        assert_eq!(true, r.overlaps(&Rectangle::new(5, 5, 6, 6)));
    }

    #[test]
    fn is_point_inside() {
        let r = Rectangle::new(0, 0, 10, 10);
       
        assert_eq!(true, r.is_point_inside(&Point::new(0, 5)));
        assert_eq!(true, r.is_point_inside(&Point::new(5, 0)));
        assert_eq!(true, r.is_point_inside(&Point::new(9, 9)));

        assert_eq!(false, r.is_point_inside(&Point::new(10, 10)));
        assert_eq!(false, r.is_point_inside(&Point::new(5, -1)));
        assert_eq!(false, r.is_point_inside(&Point::new(5, 11)));
        assert_eq!(false, r.is_point_inside(&Point::new(-1, 5)));
        assert_eq!(false, r.is_point_inside(&Point::new(11, 5)));

    }   
}
