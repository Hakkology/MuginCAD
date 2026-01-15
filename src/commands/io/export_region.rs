use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Vector2;
use eframe::egui;

#[derive(Debug, Clone)]
pub struct SelectExportRegionCommand {
    p1: Option<Vector2>,
    p2: Option<Vector2>,
}

impl SelectExportRegionCommand {
    pub fn new() -> Self {
        Self { p1: None, p2: None }
    }
}

impl Command for SelectExportRegionCommand {
    fn name(&self) -> &'static str {
        "Select Export Region"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Utility
    }

    fn initial_prompt(&self) -> String {
        "Click first corner of export region:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        if self.p1.is_none() {
            self.p1 = Some(pos);
            PointResult::NeedMore {
                prompt: "Click second corner:".to_string(),
            }
        } else {
            self.p2 = Some(pos);
            if let Some(p1) = self.p1 {
                let min = Vector2::new(p1.x.min(pos.x), p1.y.min(pos.y));
                let max = Vector2::new(p1.x.max(pos.x), p1.y.max(pos.y));
                ctx.model.export_region = Some((min, max));
            }
            PointResult::Complete
        }
    }

    fn get_points(&self) -> &[Vector2] {
        // Return points for preview
        // If we have p1, return p1. If p2, p1/p2?
        // Actually get_points returns a slice.
        // We can't return slice of optional?
        // We'll return empty if none.
        &[]
    }

    // Using draw_preview to draw the rectangle
    fn draw_preview(
        &self,
        ctx: &crate::view::rendering::context::DrawContext,
        _points: &[Vector2],
        current_cad: Vector2,
    ) {
        if let Some(p1) = self.p1 {
            let p2 = current_cad;
            // Draw rectangle logic
            // We can use ctx.painter
            let min = Vector2::new(p1.x.min(p2.x), p1.y.min(p2.y));
            let max = Vector2::new(p1.x.max(p2.x), p1.y.max(p2.y));
            let rect = crate::model::shapes::rectangle::Rectangle::new(min, max, false);

            // Draw lines for rect
            let p1 = rect.min;
            let p2 = Vector2::new(rect.max.x, rect.min.y);
            let p3 = rect.max;
            let p4 = Vector2::new(rect.min.x, rect.max.y);

            let screen_lines = vec![
                (ctx.to_screen(p1), ctx.to_screen(p2)),
                (ctx.to_screen(p2), ctx.to_screen(p3)),
                (ctx.to_screen(p3), ctx.to_screen(p4)),
                (ctx.to_screen(p4), ctx.to_screen(p1)),
            ];

            for (s, e) in screen_lines {
                ctx.painter.line_segment(
                    [s, e],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255)),
                );
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
