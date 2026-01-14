pub mod vector;
use serde::{Deserialize, Serialize};
pub use vector::Vector2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entity {
    Line {
        start: Vector2,
        end: Vector2,
    },
    Circle {
        center: Vector2,
        radius: f32,
        filled: bool,
    },
    Rectangle {
        min: Vector2,
        max: Vector2,
        filled: bool,
    },
}

pub struct CadModel {
    pub entities: Vec<Entity>,
}

impl CadModel {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
