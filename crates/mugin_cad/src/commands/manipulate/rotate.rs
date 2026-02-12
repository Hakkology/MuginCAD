use crate::commands::preview;
use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Vector2;
use std::f32::consts::PI;

define_manipulation_command!(RotateCommand);

impl Command for RotateCommand {
    fn name(&self) -> &'static str {
        "ROTATE"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    fn cannot_execute_message(&self) -> String {
        "No entities selected. Select entities first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        "ROTATE Specify base point (pivot):".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_ids = ctx.model.get_top_level_selected_ids(&ctx.selected_ids);
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 1 {
            PointResult::NeedMore {
                prompt: "Specify rotation angle point (Shift for 45° snap):".to_string(),
            }
        } else {
            // Calculate angle from base point
            let pivot = self.points[0];
            let to = pos;

            let dx = to.x - pivot.x;
            let dy = to.y - pivot.y;
            let mut angle = dy.atan2(dx);

            // Snap to 45 degree increments if Shift is pressed
            if ctx.modifiers.shift {
                let snap_angle = PI / 4.0; // 45 degrees
                angle = (angle / snap_angle).round() * snap_angle;
            }

            for &id in &self.entity_ids {
                if let Some(entity) = ctx.model.find_by_id_mut(id) {
                    entity.rotate(pivot, angle);
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

        if let Some(&pivot) = points.first() {
            let pivot_screen = ctx.to_screen(pivot);

            // Calculate angle and radius
            let dx = current_cad.x - pivot.x;
            let dy = current_cad.y - pivot.y;
            let angle = dy.atan2(dx);
            let radius = (dx * dx + dy * dy).sqrt() * ctx.zoom;

            // Draw arc from 0 to current angle
            let arc_radius = radius.min(80.0);
            let num_segments = 32;
            let start_angle = 0.0_f32;
            let end_angle = angle;

            let mut arc_points = Vec::new();
            for i in 0..=num_segments {
                let t = i as f32 / num_segments as f32;
                let a = start_angle + (end_angle - start_angle) * t;
                let px = pivot_screen.x + arc_radius * a.cos();
                let py = pivot_screen.y - arc_radius * a.sin();
                arc_points.push(egui::pos2(px, py));
            }

            let angle_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 100));
            for i in 0..arc_points.len().saturating_sub(1) {
                ctx.painter
                    .line_segment([arc_points[i], arc_points[i + 1]], angle_stroke);
            }

            // Draw radius line from pivot to mouse
            preview::draw_line_to_cursor(ctx, pivot, current_cad);

            // Draw reference line (horizontal from pivot)
            let ref_end = egui::pos2(pivot_screen.x + arc_radius, pivot_screen.y);
            ctx.painter.line_segment(
                [pivot_screen, ref_end],
                egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(150, 150, 150, 100),
                ),
            );

            // Draw pivot marker
            ctx.painter.circle_stroke(
                pivot_screen,
                5.0,
                egui::Stroke::new(2.0, egui::Color32::YELLOW),
            );
        }
    }

    impl_command_common!(RotateCommand);
}
