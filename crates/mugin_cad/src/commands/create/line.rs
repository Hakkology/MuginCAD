use crate::commands::preview;
use crate::commands::{Command, CommandContext, PointResult};
use crate::model::{Entity, Line, Vector2};

define_command!(LineCommand);

impl Command for LineCommand {
    fn name(&self) -> &'static str {
        "LINE"
    }

    fn initial_prompt(&self) -> String {
        "LINE Specify first point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        // Apply ortho constraint if shift is pressed
        let constrained_pos = if let Some(&last) = self.points.last() {
            self.constrain_point(pos, Some(last), ctx.modifiers)
        } else {
            pos
        };

        self.points.push(constrained_pos);

        if self.points.len() >= 2 {
            let start = self.points[self.points.len() - 2];
            let end = self.points[self.points.len() - 1];
            ctx.model.add_entity(Entity::Line(Line::new(start, end)));
        }

        PointResult::NeedMore {
            prompt: "Specify next point (Shift for ortho):".to_string(),
        }
    }

    fn draw_preview(
        &self,
        ctx: &crate::view::rendering::context::DrawContext,
        points: &[Vector2],
        current_cad: Vector2,
    ) {
        if let Some(&last_point) = points.last() {
            preview::draw_line_to_cursor(ctx, last_point, current_cad);
        }
    }

    impl_command_common!(LineCommand);
}
