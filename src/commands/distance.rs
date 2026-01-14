use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::annotation::TextAnnotation;
use crate::model::{Entity, Vector2};

/// Distance command - measures distance between two points
#[derive(Debug, Clone)]
pub struct DistanceCommand {
    points: Vec<Vector2>,
}

impl DistanceCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

impl Command for DistanceCommand {
    fn name(&self) -> &'static str {
        "Distance"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "Specify first point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        match self.points.len() {
            1 => PointResult::NeedMore {
                prompt: "Specify second point:".to_string(),
            },
            2 => {
                // Create distance annotation
                let start = self.points[0];
                let end = self.points[1];
                let annotation = TextAnnotation::new_distance(start, end);

                // Calculate distance for status message
                let _distance = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt();

                ctx.model.add_entity(Entity::Text(annotation));

                PointResult::Complete
            }
            _ => PointResult::Complete,
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
