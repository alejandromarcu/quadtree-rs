extern crate num;

// TODO they shouldn't be pub, but doing so avoids the unused warnings for now.
pub mod node;
pub mod point;
pub mod rectangle;

use node::{Node, Quad};
use point::{Coordinate, Point};
use rectangle::Rectangle;

use std::collections::HashMap;

// TODO use generics
type Id = i32;

#[derive(Clone, Debug)]
pub struct QuadTreeConfig {
    min_per_quad: i32,
    max_per_cell: i32,
}

impl QuadTreeConfig {
    pub fn default() -> Self {
        QuadTreeConfig {
            min_per_quad: 50,
            max_per_cell: 100,
        }
    }

    pub fn new(min_per_quad: i32, max_per_cell: i32) -> Self {
        assert!(min_per_quad < max_per_cell);
        QuadTreeConfig {
            min_per_quad,
            max_per_cell,
        }
    }
}

pub struct QuadTree<T: Coordinate> {
    root: Box<Quad<T>>,
    points: HashMap<Id, Point<T>>,
}

impl<T: Coordinate> QuadTree<T> {
    pub fn new(boundary: Rectangle<T>, config: QuadTreeConfig) -> Self {
        QuadTree {
            root: Quad::new(config, boundary),
            points: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: Id, point: Point<T>) -> Result<(), String> {
        if self.points.contains_key(&id) {
            return Err(format!("Id already exists: {:?}", id));
        }
        (*self.root).add(id, point.clone());
        self.points.insert(id, point);
        Ok(())
    }

    pub fn get_point(&self, id: Id) -> Option<&Point<T>> {
        self.points.get(&id)
    }
}

#[cfg(test)]
mod quadtree_tests {
    use super::{QuadTree, QuadTreeConfig};
    use crate::point::Point;
    use crate::rectangle::Rectangle;

    #[test]
    fn add_duplicate() {
        let mut qt = QuadTree::new(
            Rectangle::new(0.0, 0.0, 20.0, 20.0),
            QuadTreeConfig::default(),
        );
        qt.add(1, Point::new(10.0, 10.0)).unwrap();
        qt.add(2, Point::new(12.0, 15.0)).unwrap();
        qt.add(3, Point::new(10.0, 10.0)).unwrap();

        assert_eq!(true, qt.add(1, Point::new(11.0, 10.0)).is_err());
    }

    #[test]
    fn get_point() {
        let mut qt = QuadTree::new(
            Rectangle::new(0.0, 0.0, 20.0, 20.0),
            QuadTreeConfig::default(),
        );
        qt.add(1, Point::new(10.0, 10.0)).unwrap();
        qt.add(2, Point::new(12.0, 15.0)).unwrap();

        assert_eq!((10.0, 10.0), qt.get_point(1).unwrap().as_tuple());
        assert_eq!(true, qt.get_point(100).is_none());
    }

}
