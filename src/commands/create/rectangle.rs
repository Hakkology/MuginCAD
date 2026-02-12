use crate::commands::preview;
use crate::commands::{Command, CommandContext, PointResult};
use crate::model::{Entity, Rectangle, Vector2};

define_command!(RectangleCommand);

impl Command for RectangleCommand {
    fn name(&self) -> &'static str {
        "RECTANGLE"
    }

    fn initial_prompt(&self) -> String {
        "RECTANGLE Specify first corner:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 2 {
            let p1 = self.points[0];
            let p2 = self.points[1];
            let min = Vector2::new(p1.x.min(p2.x), p1.y.min(p2.y));
            let max = Vector2::new(p1.x.max(p2.x), p1.y.max(p2.y));
            ctx.model
                .add_entity(Entity::Rectangle(Rectangle::new(min, max, ctx.filled_mode)));
            PointResult::Complete
        } else {
            PointResult::NeedMore {
                prompt: "Specify other corner:".to_string(),
            }
        }
    }

    fn draw_preview(
        &self,
        ctx: &crate::view::rendering::context::DrawContext,
        points: &[Vector2],
        current_cad: Vector2,
    ) {
        use eframe::egui;

        if let Some(&start) = points.first() {
            let min = Vector2::new(start.x.min(current_cad.x), start.y.min(current_cad.y));
            let max = Vector2::new(start.x.max(current_cad.x), start.y.max(current_cad.y));
            let rect_screen = egui::Rect::from_min_max(
                ctx.to_screen(Vector2::new(min.x, max.y)),
                ctx.to_screen(Vector2::new(max.x, min.y)),
            );
            ctx.painter
                .rect_stroke(rect_screen, 0.0, preview::preview_stroke());

            let width = (max.x - min.x).abs();
            let height = (max.y - min.y).abs();

            let bottom_mid = ctx.to_screen(Vector2::new((min.x + max.x) / 2.0, min.y));
            preview::draw_dimension_text(
                ctx,
                egui::pos2(bottom_mid.x, bottom_mid.y + 14.0),
                format!("W: {:.2}", width),
            );

            let right_mid = ctx.to_screen(Vector2::new(max.x, (min.y + max.y) / 2.0));
            preview::draw_dimension_text(
                ctx,
                egui::pos2(right_mid.x + 30.0, right_mid.y),
                format!("H: {:.2}", height),
            );
        }
    }

    impl_command_common!(RectangleCommand);
}
