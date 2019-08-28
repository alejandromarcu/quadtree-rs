extern crate quadtree_rs;

use quadtree_rs::point::Point;
use quadtree_rs::rectangle::Rectangle;
use quadtree_rs::{QuadTree, QuadTreeConfig};

use std::f64::consts::PI;
use std::time::Instant;

use svg_fmt::*;

fn main() {
    let config = QuadTreeConfig::new(10, 20);
    let side = 1000.0;
    let mut qt = QuadTree::new(Rectangle::new(0.0, 0.0, side, side), config);

    let n_radial = 100;
    let n_along = 100;

    let mut id = 0;

    let start = Instant::now();

    for i in 0..n_radial {
        for j in 0..n_along {
            let angle = 2.0 * PI / n_radial as f64 * i as f64;
            let dist = side / n_along as f64 * j as f64 / 2.0;
            let x = angle.sin() * dist;
            let y = angle.cos() * dist;
            let _ = qt.add(id, Point::new(x + side / 2.0, y + side / 2.0));
            id += 1;
        }
    }
    // println!(
    //     "Added {} points.  Total cells: {}.  Took {}ms",
    //     id,
    //     qt.get_cells_info().len(),
    //     start.elapsed().as_millis()
    // );

    println!("{}", BeginSvg { w: 400.0, h: 400.0 });
    for cell_info in qt.get_cells_info().iter() {
        let r = &cell_info.boundary;
        println!(
            "    {}",
           rectangle(trf(r.x0), trf(r.y0), trf(r.x1 - r.x0), trf(r.y1 - r.y0))
            // rectangle(trf(r.x0), trf(r.y0), 10.0, 10.0)
              //  .fill(Fill::Color(white()))
              .fill(Fill::None)
                .stroke(Stroke::Color(black(), 1.0))
        );
    }

    for (_, point) in qt.points.iter() {
        println!("    {}", Circle { x: trf(point.x), y:trf(point.y), radius : 1.0, style : Style::default() });
    }
    println!("{}", EndSvg);

}

fn trf(x : f64) -> f32 {
    x as f32
   // ((x as f32) + 0.0) * 10.0
}
