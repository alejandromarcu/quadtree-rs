use crate::point::{Coordinate, Point};
use crate::rectangle::Rectangle;
use crate::QuadTreeConfig;

use std::collections::HashMap;
use std::fmt;

// TODO use generics
type Id = i32;

trait Node<T: Coordinate>: fmt::Debug {
    fn add(&mut self, id: Id, p: Point<T>);
    fn get_cells_info(&self) -> Vec<CellInfo<T>>;
}

struct Cell<T: Coordinate> {
    config: QuadTreeConfig,
    boundary: Rectangle<T>,
    parent: *mut Quad<T>,
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

    fn get_cells_info(&self) -> Vec<CellInfo<T>> {
        vec![CellInfo::new(&self.boundary, self.points.len())]
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
pub struct CellInfo<T : Coordinate> {
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

struct Quad<T: Coordinate> {
    config: QuadTreeConfig,
    boundary: Rectangle<T>,
    children: [Option<Box<Node<T>>>; 4],
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
}

impl<T: Coordinate + 'static> Quad<T> {
    fn new(config: QuadTreeConfig, boundary: Rectangle<T>) -> Box<Self> {
        let quad = Quad {
            config: config.clone(),
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
mod test {
    use super::Quad;
    use crate::QuadTreeConfig;
    use crate::rectangle::Rectangle;
    use crate::node::Node;

    #[test]
    fn quad_new() {
        let quad = Quad::new(QuadTreeConfig::default(), Rectangle::new(0, 10, 10, 40));
        let cells_info = quad.get_cells_info();
        assert_eq!("((0, 10) - (5, 25))", format!("{:?}", cells_info[0].boundary));
        assert_eq!("((5, 10) - (10, 25))", format!("{:?}", cells_info[1].boundary));
        assert_eq!("((0, 25) - (5, 40))", format!("{:?}", cells_info[2].boundary));
        assert_eq!("((5, 25) - (10, 40))", format!("{:?}", cells_info[3].boundary));
    }
}