use crate::commands::{Command, CommandCategory, CommandContext, InputResult, PointResult};
use crate::model::shapes::annotation::TextAnnotation;
use crate::model::{Entity, Vector2};

define_command!(TextCommand);

impl Command for TextCommand {
    fn name(&self) -> &'static str {
        "Text"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "Specify text position:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, _ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        // After first point, wait for text input
        PointResult::NeedMore {
            prompt: "Enter text content:".to_string(),
        }
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        if self.points.is_empty() {
            // Try to parse as point first
            if let Some(pos) = crate::commands::parse_point(input) {
                self.points.push(pos);
                return InputResult::Point(PointResult::NeedMore {
                    prompt: "Enter text content:".to_string(),
                });
            }
            return InputResult::Invalid {
                message: "Please specify a position first.".to_string(),
            };
        }

        // We have a position, treat input as text content
        let text = input.trim().to_string();
        if text.is_empty() {
            return InputResult::Invalid {
                message: "Text cannot be empty.".to_string(),
            };
        }

        let annotation = TextAnnotation::new_custom(self.points[0], text);
        ctx.model.add_entity(Entity::text(annotation));

        InputResult::Point(PointResult::Complete)
    }

    impl_command_common!(TextCommand);
}
