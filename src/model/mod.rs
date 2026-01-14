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

impl Entity {
    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        match self {
            Entity::Line { start, end } => pos.dist_to_line(*start, *end) < tolerance,
            Entity::Circle {
                center,
                radius,
                filled,
            } => {
                let d = pos.dist(*center);
                if *filled {
                    d <= *radius + tolerance
                } else {
                    (d - radius).abs() < tolerance
                }
            }
            Entity::Rectangle { min, max, filled } => {
                let inside = pos.x >= min.x - tolerance
                    && pos.x <= max.x + tolerance
                    && pos.y >= min.y - tolerance
                    && pos.y <= max.y + tolerance;

                if *filled {
                    inside
                } else {
                    // Check if near edges
                    let near_x =
                        (pos.x - min.x).abs() < tolerance || (pos.x - max.x).abs() < tolerance;
                    let near_y =
                        (pos.y - min.y).abs() < tolerance || (pos.y - max.y).abs() < tolerance;
                    inside && (near_x || near_y)
                }
            }
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Entity::Line { .. } => "Line",
            Entity::Circle { .. } => "Circle",
            Entity::Rectangle { .. } => "Rectangle",
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
        // Reverse order to pick the top-most (last drawn) entity
        self.entities
            .iter()
            .enumerate()
            .rev()
            .find(|(_, e)| e.hit_test(pos, tolerance))
            .map(|(i, _)| i)
    }
}
