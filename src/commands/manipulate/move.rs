use crate::commands::preview;
use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Vector2;

define_manipulation_command!(MoveCommand);

impl Command for MoveCommand {
    fn name(&self) -> &'static str {
        "MOVE"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    fn cannot_execute_message(&self) -> String {
        "No entities selected. Select entities first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        "MOVE Specify base point:".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_indices = ctx.selected_indices.iter().cloned().collect();
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 1 {
            PointResult::NeedMore {
                prompt: "Specify destination point (Shift for ortho):".to_string(),
            }
        } else {
            // Calculate delta and move entity
            let from = self.points[0];
            let to = pos;
            let mut delta = to - from;

            // Apply ortho constraint if Shift is pressed
            if ctx.modifiers.shift {
                if delta.x.abs() > delta.y.abs() {
                    delta = Vector2::new(delta.x, 0.0);
                } else {
                    delta = Vector2::new(0.0, delta.y);
                }
            }

            for &idx in &self.entity_indices {
                if let Some(entity) = ctx.model.entities.get_mut(idx) {
                    entity.translate(delta);
                }
            }

            PointResult::Complete
        }
    }

    fn draw_preview(
        &self,
        ctx: &crate::view::rendering::context::DrawContext,
        points: &[Vector2],
        current_cad: Vector2,
    ) {
        use eframe::egui;
        if let Some(&base) = points.first() {
            preview::draw_line_to_cursor(ctx, base, current_cad);
            ctx.painter
                .circle_stroke(ctx.to_screen(base), 4.0, preview::preview_stroke());
            preview::draw_point_marker(ctx, current_cad, egui::Color32::WHITE);
        }
    }

    impl_command_common!(MoveCommand);
}
