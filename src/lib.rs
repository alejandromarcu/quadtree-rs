extern crate num;

// TODO they shouldn't be pub, but doing so avoids the unused warnings for now.
pub mod point;
pub mod rectangle;
pub mod node;

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

pub struct QuadTree {
    config : QuadTreeConfig,
}

impl QuadTree {
    pub fn new() -> Self {
        QuadTree {
            config : QuadTreeConfig::default(),
        }
    }

    pub fn with_config(self, config : QuadTreeConfig) -> Self {
        QuadTree {
            config,
            ..self
        }
    }
}



