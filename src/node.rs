use crate::point::{Coordinate, Point};
use crate::rectangle::Rectangle;
use crate::{Id, QuadTreeConfig};

use std::collections::HashMap;
use std::fmt;

pub trait Node<T: Coordinate>: fmt::Debug {
    fn add(&mut self, id: Id, p: Point<T>);
    fn get_cells_info(&self) -> Vec<CellInfo<T>>;
    fn find_in_area(&self, area: &Rectangle<T>) -> Vec<Id>;
}

struct Cell<T: Coordinate> {
    config: QuadTreeConfig,
    boundary: Rectangle<T>,
    parent: *mut Quad<T>,
    points: HashMap<Id, Point<T>>,
}

impl<T: Coordinate + 'static> Node<T> for Cell<T> {
    fn add(&mut self, id: Id, p: Point<T>) {
        self.points.insert(id, p);

        if self.points.len() as i32 > self.config.max_per_cell {
            let (_, first_point) = self.points.iter().nth(0).as_ref().unwrap().clone();
            let all_in_the_same_location = !self.points.iter().any(|(_, p) | p != first_point);
            if all_in_the_same_location {
                return;
            }

            let mut quad: Box<Quad<T>> = Quad::new(self.config.clone(), self.boundary.clone());

            for (id, point) in self.points.iter() {
                quad.add(id.clone(), point.clone());
            }

            unsafe {
                (*self.parent).replace_child(self, quad);
            }
        }
    }

    fn get_cells_info(&self) -> Vec<CellInfo<T>> {
        vec![CellInfo::new(&self.boundary, self.points.len())]
    }

    fn find_in_area(&self, area: &Rectangle<T>) -> Vec<Id> {
        // Just an optimization, not sure if it's worth it.
        let all = self.boundary.is_inside_of(area);
        self.points
            .iter()
            .filter(|(_, point)| all || area.is_point_inside(point))
            .map(|(id, _)| *id)
            .collect::<Vec<Id>>()
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

impl<T: Coordinate> Cell<T> {
    fn new(config: QuadTreeConfig, boundary: Rectangle<T>, parent: *mut Quad<T>) -> Self {
        Cell {
            config,
            boundary,
            parent,
            points: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct CellInfo<T: Coordinate> {
    pub boundary: Rectangle<T>,
    pub count: usize,
}

impl<T: Coordinate> CellInfo<T> {
    pub fn new(boundary: &Rectangle<T>, count: usize) -> Self {
        CellInfo {
            boundary: boundary.clone(),
            count,
        }
    }
}

pub(crate) struct Quad<T: Coordinate> {
    pub boundary: Rectangle<T>,
    children: [Option<Box<dyn Node<T>>>; 4],
}

impl<T: Coordinate> Node<T> for Quad<T> {
    fn add(&mut self, id: Id, p: Point<T>) {
        let (xh, yh) = self.boundary.center().as_tuple();
        let child_n = match (p.x >= xh, p.y >= yh) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        };
        self.children[child_n].as_mut().unwrap().add(id, p);
    }

    fn get_cells_info(&self) -> Vec<CellInfo<T>> {
        let mut info = vec![];
        for ch in self.children.iter() {
            let mut ch_info = ch.as_ref().unwrap().get_cells_info();
            info.append(&mut ch_info);
        }

        info
    }

    fn find_in_area(&self, area: &Rectangle<T>) -> Vec<Id> {
        if !self.boundary.overlaps(area) {
            return vec![];
        }

        let mut ids = vec![];

        for ch in self.children.iter() {
            let mut child_ids = ch.as_ref().unwrap().find_in_area(area);
            ids.append(&mut child_ids);
        }

        ids
    }
}

impl<T: Coordinate + 'static> Quad<T> {
    pub fn new(config: QuadTreeConfig, boundary: Rectangle<T>) -> Box<Self> {
        let quad = Quad {
            boundary: boundary.clone(),
            children: [None, None, None, None],
        };
        let mut quad = Box::new(quad);

        let (x0, y0, x1, y1) = boundary.as_tuple();
        let (xh, yh) = boundary.center().as_tuple();
        let boundaries = [
            Rectangle::new(x0, y0, xh, yh),
            Rectangle::new(xh, y0, x1, yh),
            Rectangle::new(x0, yh, xh, y1),
            Rectangle::new(xh, yh, x1, y1),
        ];
        for i in 0..4 {
            // TODO improve the clone
            let cell = Cell::new(config.clone(), boundaries[i].clone(), &mut *quad);
            quad.children[i] = Some(Box::new(cell));
        }

        quad
    }

    fn replace_child(&mut self, curr_child_p: *const dyn Node<T>, new_child: Box<dyn Node<T>>) {
        let idx = self
            .children
            .iter()
            .position(|ch| &(**ch.as_ref().unwrap()) as *const dyn Node<T> == curr_child_p)
            .expect("Child not found when trying to replace it.");

        self.children[idx] = Some(new_child);
    }
}

impl<T: Coordinate> fmt::Debug for Quad<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Quad (boundary: {:?};\n    q1:{:?},\n    q2:{:?},\n    q3:{:?},\n    q4:{:?})",
            self.boundary,
            self.children[0].as_ref().unwrap(),
            self.children[1].as_ref().unwrap(),
            self.children[2].as_ref().unwrap(),
            self.children[3].as_ref().unwrap()
        )
    }
}

#[cfg(test)]
mod quad_test {
    use super::Quad;
    use crate::node::Node;
    use crate::point::Point;
    use crate::rectangle::Rectangle;
    use crate::QuadTreeConfig;

    #[test]
    fn quad_new_right_cell_boundaries() {
        let quad = Quad::new(QuadTreeConfig::default(), Rectangle::new(0, 10, 10, 40));
        let cells_info = quad
            .get_cells_info()
            .iter()
            .map(|ci| format!("{:?}", ci.boundary))
            .collect::<Vec<String>>();

        assert_eq!("((0, 10) - (5, 25))", cells_info[0]);
        assert_eq!("((5, 10) - (10, 25))", cells_info[1]);
        assert_eq!("((0, 25) - (5, 40))", cells_info[2]);
        assert_eq!("((5, 25) - (10, 40))", cells_info[3]);
    }

    #[test]
    fn quad_replace_child() {
        let mut quad = Quad::new(QuadTreeConfig::default(), Rectangle::new(0, 10, 10, 40));

        // Each quadrant is a cell, so 4 cells total
        assert_eq!(4, quad.get_cells_info().len());

        // replace the first quadrant with another quad
        let quad2 = Quad::new(QuadTreeConfig::default(), Rectangle::new(0, 5, 10, 25));

        let child_pointer = &**(quad.children[0].as_ref().unwrap()) as *const dyn Node<i32>;
        quad.replace_child(child_pointer, quad2);

        // Now the first quadrant has 4 cells plus the other 3 quadrants
        assert_eq!(7, quad.get_cells_info().len());
    }

    #[test]
    fn quad_split() {
        let mut quad = Quad::new(QuadTreeConfig::new(0, 5), Rectangle::new(0, 0, 256, 256));

        // Adding 5 points in the top left quadrant, shouldn't cause a split yet
        for i in 0..5 {
            quad.add(i, Point::new(i, 0));
        }

        assert_eq!(4, quad.get_cells_info().len());

        // But when adding the 6th point in the bottom right quadrant of the top left,
        // it should split now, and the top left is now a quad.
        quad.add(5, Point::new(127, 127));
        assert_eq!(7, quad.get_cells_info().len());

        // Adding this new point will cause a few more splits since it needs to get to a
        // cell small enough to have less than 5 points.
        quad.add(6, Point::new(6, 0));
        assert_eq!(19, quad.get_cells_info().len());

        // let's add a point somewhere else and make sure that it got in the right place
        quad.add(7, Point::new(7, 7));
        let cell_boundary = Rectangle::new(4, 4, 8, 8);

        let cell_count = quad
            .get_cells_info()
            .iter()
            .find(|&cell| cell.boundary == cell_boundary)
            .expect("The cell was not found, something is wrong")
            .count;

        assert_eq!(1, cell_count);
    }

    // TODO test find_in_area
}

#[cfg(test)]
mod cell_test {
    use super::Quad;
    use crate::node::Node;
    use crate::point::Point;
    use crate::rectangle::Rectangle;
    use crate::QuadTreeConfig;

    #[test]
    fn find_in_area_all() {
        // The child boundary will be (0, 0, 10, 10)
        let mut quad = Quad::new(QuadTreeConfig::default(), Rectangle::new(0, 0, 20, 20));
        let ch = &mut quad.children[0].as_mut().unwrap();

        ch.add(1, Point::new(0, 0));
        ch.add(2, Point::new(0, 9));
        ch.add(3, Point::new(9, 0));
        ch.add(4, Point::new(5, 5));

        let mut points = ch.find_in_area(&Rectangle::new(-1, -1, 15, 15));
        points.sort();

        assert_eq!(vec![1, 2, 3, 4], points);
    }

    #[test]
    fn find_in_area() {
        // The child boundary will be (0, 0, 10, 10)
        let mut quad = Quad::new(QuadTreeConfig::default(), Rectangle::new(0, 0, 20, 20));
        let ch = &mut quad.children[0].as_mut().unwrap();

        ch.add(1, Point::new(0, 0));
        ch.add(2, Point::new(0, 8));
        ch.add(3, Point::new(8, 0));
        ch.add(4, Point::new(5, 5));

        let mut points = ch.find_in_area(&Rectangle::new(0, 0, 9, 9));
        points.sort();
        assert_eq!(vec![1, 2, 3, 4], points);

        let points = ch.find_in_area(&Rectangle::new(5, 5, 6, 6));
        assert_eq!(vec![4], points);

        let points = ch.find_in_area(&Rectangle::new(4, 4, 5, 5));
        assert_eq!(true, points.is_empty());

        let mut points = ch.find_in_area(&Rectangle::new(0, 0, 6, 6));
        points.sort();
        assert_eq!(vec![1, 4], points);
    }

    #[test]
    fn same_location() {
        // The child boundary will be (0, 0, 10, 10)
        let mut quad = Quad::new(QuadTreeConfig::new(50, 100), Rectangle::new(0, 0, 20, 20));

        // All those points are in the same location, so this shouldn't trigger a split of the quad,
        // or it would cause infinite recursion.
        for i in 0..1000 {
            quad.add(i, Point::new(1,1));
        }
        

        assert_eq!(4, quad.get_cells_info().len());

        // now, if we add just an extra point, it should split the quad
        quad.add(1000, Point::new(8, 8));
        assert_eq!(7, quad.get_cells_info().len());
    }
}
