use crate::commands::{Command, CommandCategory, CommandContext, InputResult, PointResult};
use crate::model::shapes::annotation::TextAnnotation;
use crate::model::{Entity, Vector2};

/// Measure command - creates a distance dimension between two points
#[derive(Debug, Clone)]
pub struct MeasureCommand {
    points: Vec<Vector2>,
}

impl MeasureCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

impl Command for MeasureCommand {
    fn name(&self) -> &'static str {
        "Measure"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "Specify first point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 1 {
            // Need second point
            return PointResult::NeedMore {
                prompt: "Specify second point:".to_string(),
            };
        }

        // Two points collected
        let start = self.points[0];
        let end = self.points[1];

        let annotation = TextAnnotation::new_distance(start, end);
        ctx.model.add_entity(Entity::Text(annotation));

        // Reset points to allow continuous measuring?
        // Or finish? Usually finish.
        // Let's finish for now.
        PointResult::Complete
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        // Try parsing input as point
        if let Some(pos) = crate::commands::parse_point(input) {
            InputResult::Point(self.push_point(pos, ctx))
        } else {
            InputResult::Invalid {
                message: "Please specify a point or click.".to_string(),
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
