pub mod circle;
pub mod line;
pub mod rectangle;
pub mod vector;

use serde::{Deserialize, Serialize};

pub use circle::Circle;
pub use line::Line;
pub use rectangle::Rectangle;
pub use vector::Vector2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entity {
    Line(Line),
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Entity {
    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        match self {
            Entity::Line(line) => line.hit_test(pos, tolerance),
            Entity::Circle(circle) => circle.hit_test(pos, tolerance),
            Entity::Rectangle(rect) => rect.hit_test(pos, tolerance),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Entity::Line(_) => "Line",
            Entity::Circle(_) => "Circle",
            Entity::Rectangle(_) => "Rectangle",
        }
    }
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

    pub fn pick_entity(&self, pos: Vector2, tolerance: f32) -> Option<usize> {
        self.entities
            .iter()
            .enumerate()
            .rev()
            .find(|(_, e)| e.hit_test(pos, tolerance))
            .map(|(i, _)| i)
    }
}
