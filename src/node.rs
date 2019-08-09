use crate::point::{Coordinate, Point};
use crate::rectangle::Rectangle;
use crate::QuadTreeConfig;

use std::collections::HashMap;
use std::fmt;

// TODO use generics
type Id = i32;

trait Node<T: Coordinate>: fmt::Debug {
    fn add(&mut self, id: Id, p: Point<T>);
}

struct Cell<T: Coordinate> {
    config: QuadTreeConfig,
    boundary: Rectangle<T>,
    parent: *mut Quad,
    points: HashMap<Id, Point<T>>,
}

impl<T: Coordinate> Node<T> for Cell<T> {
    fn add(&mut self, id: Id, p: Point<T>) {
        self.points.insert(id, p);

        if self.points.len() as i32 > self.config.max_per_cell {
            println!("Split in leaf {:?}", self);
            // let mut quad = Quad::new(self.config.clone(), self.boundary.clone());

            // for (id, point) in self.points.iter() {
            //     quad.add(id.clone(), point.clone());
            // }

            // unsafe {
            // // TODO fix 0

            //     (*self.parent).children[0] = Some(quad);
            // }
        }
    }
}

impl<T: Coordinate> fmt::Debug for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cell(boundary: {:?}, points: {})",
            self.boundary,
            self.points.len()
        )
    }
}

impl<T : Coordinate> Cell<T> {
    fn new(config: QuadTreeConfig, boundary: Rectangle<T>, parent: *mut Quad) -> Self {
        Cell {
            config,
            boundary,
            parent,
            points: HashMap::new(),
        }
    }
}

struct Quad;