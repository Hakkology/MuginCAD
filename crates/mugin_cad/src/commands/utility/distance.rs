use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::shapes::annotation::TextAnnotation;
use crate::model::{Entity, Vector2};

define_command!(DistanceCommand);

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
                let start = self.points[0];
                let end = self.points[1];
                let annotation = TextAnnotation::new_distance(start, end);
                ctx.model.add_entity(Entity::Text(annotation));
                PointResult::Complete
            }
            _ => PointResult::Complete,
        }
    }

    impl_command_common!(DistanceCommand);
}
