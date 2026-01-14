pub mod axis;
pub mod circle;
pub mod config;
pub mod line;
pub mod rectangle;
pub mod snap;
pub mod undo;
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

    /// Get the center point of the entity
    pub fn center(&self) -> Vector2 {
        match self {
            Entity::Line(line) => Vector2::new(
                (line.start.x + line.end.x) / 2.0,
                (line.start.y + line.end.y) / 2.0,
            ),
            Entity::Circle(circle) => circle.center,
            Entity::Rectangle(rect) => Vector2::new(
                (rect.min.x + rect.max.x) / 2.0,
                (rect.min.y + rect.max.y) / 2.0,
            ),
        }
    }

    /// Translate (move) the entity by a delta
    pub fn translate(&mut self, delta: Vector2) {
        match self {
            Entity::Line(line) => {
                line.start = line.start + delta;
                line.end = line.end + delta;
            }
            Entity::Circle(circle) => {
                circle.center = circle.center + delta;
            }
            Entity::Rectangle(rect) => {
                rect.min = rect.min + delta;
                rect.max = rect.max + delta;
            }
        }
    }

    /// Rotate the entity around a pivot point by angle (radians)
    pub fn rotate(&mut self, pivot: Vector2, angle: f32) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let rotate_point = |p: Vector2| -> Vector2 {
            let dx = p.x - pivot.x;
            let dy = p.y - pivot.y;
            Vector2::new(
                pivot.x + dx * cos_a - dy * sin_a,
                pivot.y + dx * sin_a + dy * cos_a,
            )
        };

        match self {
            Entity::Line(line) => {
                line.start = rotate_point(line.start);
                line.end = rotate_point(line.end);
            }
            Entity::Circle(circle) => {
                circle.center = rotate_point(circle.center);
            }
            Entity::Rectangle(rect) => {
                // For rectangle, rotate the corners and recalculate bounds
                let p1 = rotate_point(rect.min);
                let p2 = rotate_point(Vector2::new(rect.max.x, rect.min.y));
                let p3 = rotate_point(rect.max);
                let p4 = rotate_point(Vector2::new(rect.min.x, rect.max.y));

                rect.min = Vector2::new(
                    p1.x.min(p2.x).min(p3.x).min(p4.x),
                    p1.y.min(p2.y).min(p3.y).min(p4.y),
                );
                rect.max = Vector2::new(
                    p1.x.max(p2.x).max(p3.x).max(p4.x),
                    p1.y.max(p2.y).max(p3.y).max(p4.y),
                );
            }
        }
    }

    /// Scale the entity from a base point by a factor
    pub fn scale(&mut self, base: Vector2, factor: f32) {
        let scale_point = |p: Vector2| -> Vector2 {
            Vector2::new(
                base.x + (p.x - base.x) * factor,
                base.y + (p.y - base.y) * factor,
            )
        };

        match self {
            Entity::Line(line) => {
                line.start = scale_point(line.start);
                line.end = scale_point(line.end);
            }
            Entity::Circle(circle) => {
                circle.center = scale_point(circle.center);
                circle.radius *= factor;
            }
            Entity::Rectangle(rect) => {
                rect.min = scale_point(rect.min);
                rect.max = scale_point(rect.max);
            }
        }
    }
}

pub struct CadModel {
    pub entities: Vec<Entity>,
    pub axis_manager: axis::AxisManager,
}

impl CadModel {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            axis_manager: axis::AxisManager::new(),
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
