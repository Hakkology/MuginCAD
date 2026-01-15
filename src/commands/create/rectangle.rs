use crate::commands::{Command, CommandContext, PointResult};
use crate::model::{Entity, Rectangle, Vector2};

#[derive(Debug, Clone)]
pub struct RectangleCommand {
    points: Vec<Vector2>,
}

impl RectangleCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

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

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn draw_preview(
        &self,
        ctx: &crate::view::rendering::context::DrawContext,
        points: &[Vector2],
        current_cad: Vector2,
    ) {
        use eframe::egui;
        let preview_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
        );

        if let Some(&start) = points.first() {
            let min = Vector2::new(start.x.min(current_cad.x), start.y.min(current_cad.y));
            let max = Vector2::new(start.x.max(current_cad.x), start.y.max(current_cad.y));
            let rect_screen = egui::Rect::from_min_max(
                ctx.to_screen(Vector2::new(min.x, max.y)),
                ctx.to_screen(Vector2::new(max.x, min.y)),
            );
            ctx.painter.rect_stroke(rect_screen, 0.0, preview_stroke);

            let width = (max.x - min.x).abs();
            let height = (max.y - min.y).abs();
            let dim_color = egui::Color32::from_rgb(255, 200, 100);
            let dim_font = egui::FontId::proportional(11.0);

            let bottom_mid = ctx.to_screen(Vector2::new((min.x + max.x) / 2.0, min.y));
            ctx.painter.text(
                egui::pos2(bottom_mid.x, bottom_mid.y + 14.0),
                egui::Align2::CENTER_CENTER,
                format!("W: {:.2}", width),
                dim_font.clone(),
                dim_color,
            );

            let right_mid = ctx.to_screen(Vector2::new(max.x, (min.y + max.y) / 2.0));
            ctx.painter.text(
                egui::pos2(right_mid.x + 30.0, right_mid.y),
                egui::Align2::CENTER_CENTER,
                format!("H: {:.2}", height),
                dim_font.clone(),
                dim_color,
            );
        }
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
