use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Entity;
use crate::model::Vector2;
use crate::model::math::geometry;

#[derive(Debug, Clone)]
pub struct MeasurePerimeterCommand {
    points: Vec<Vector2>,
}

impl MeasurePerimeterCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

impl Command for MeasurePerimeterCommand {
    fn name(&self) -> &'static str {
        "Measure Perimeter"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "Click inside a closed region to measure Perimeter:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        if let Some((_, vertices)) = geometry::find_closed_region(&ctx.model, pos) {
            let perim = geometry::calculate_path_perimeter(&vertices);

            // Calculate centroid
            let mut cx = 0.0;
            let mut cy = 0.0;
            for v in &vertices {
                cx += v.x;
                cy += v.y;
            }
            if !vertices.is_empty() {
                let inv_len = 1.0 / vertices.len() as f32;
                cx *= inv_len;
                cy *= inv_len;
            }
            let centroid = Vector2::new(cx, cy);

            let annotation = crate::model::TextAnnotation::new_perimeter(centroid, perim, vertices);
            ctx.model.add_entity(Entity::Text(annotation));

            PointResult::Complete
        } else {
            PointResult::NeedMore {
                prompt: "Region not closed. Try another point.".to_string(),
            }
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
