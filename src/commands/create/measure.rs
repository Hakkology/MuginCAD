use crate::commands::{Command, CommandCategory, CommandContext, InputResult, PointResult};
use crate::model::shapes::annotation::TextAnnotation;
use crate::model::{Entity, Vector2};

define_command!(MeasureCommand);

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
            return PointResult::NeedMore {
                prompt: "Specify second point:".to_string(),
            };
        }

        // Two points collected
        let start = self.points[0];
        let end = self.points[1];

        let annotation = TextAnnotation::new_distance(start, end);
        ctx.model.add_entity(Entity::Text(annotation));

        PointResult::Complete
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        if let Some(pos) = crate::commands::parse_point(input) {
            InputResult::Point(self.push_point(pos, ctx))
        } else {
            InputResult::Invalid {
                message: "Please specify a point or click.".to_string(),
            }
        }
    }

    impl_command_common!(MeasureCommand);
}
