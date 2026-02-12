use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::structural::beam::Beam;
use crate::model::{Entity, Vector2};

/// Command to place beam instances
#[derive(Debug, Clone)]
pub struct PlaceBeamCommand {
    type_id: String,
    points: Vec<Vector2>,
    /// Dimensions from selected type
    width: f32,
    height: f32,
}

impl PlaceBeamCommand {
    pub fn new(type_id: String, width: f32, height: f32) -> Self {
        Self {
            type_id,
            points: Vec::new(),
            width,
            height,
        }
    }
}

impl Command for PlaceBeamCommand {
    fn name(&self) -> &'static str {
        "BEAM"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        format!(
            "BEAM [{}] ({}x{}) Click first point:",
            self.type_id, self.width, self.height
        )
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 1 {
            PointResult::NeedMore {
                prompt: "Specify end point:".to_string(),
            }
        } else {
            // Create beam from start to end
            let start = self.points[0];
            let end = pos;

            let index = ctx.model.structural_types.next_beam_index(&self.type_id);

            let beam = Beam {
                type_id: self.type_id.clone(),
                start,
                end,
                index,
            };

            ctx.model.add_entity(Entity::Beam(beam));

            // Reset for next beam
            self.points.clear();

            PointResult::NeedMore {
                prompt: format!(
                    "Placed {}-B{}. Click first point for next or ESC:",
                    self.type_id, index
                ),
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

        if let Some(&start) = points.first() {
            // Draw beam preview with width
            let dx = current_cad.x - start.x;
            let dy = current_cad.y - start.y;
            let length = (dx * dx + dy * dy).sqrt();

            if length > 0.01 {
                // Calculate perpendicular offset for beam width
                let nx = -dy / length;
                let ny = dx / length;
                let half_w = self.width / 2.0;

                let corners = [
                    Vector2::new(start.x + nx * half_w, start.y + ny * half_w),
                    Vector2::new(start.x - nx * half_w, start.y - ny * half_w),
                    Vector2::new(current_cad.x - nx * half_w, current_cad.y - ny * half_w),
                    Vector2::new(current_cad.x + nx * half_w, current_cad.y + ny * half_w),
                ];

                let screen_corners: Vec<egui::Pos2> =
                    corners.iter().map(|c| ctx.to_screen(*c)).collect();

                ctx.painter.add(egui::Shape::convex_polygon(
                    screen_corners,
                    egui::Color32::from_rgba_unmultiplied(100, 100, 100, 80),
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 150),
                    ),
                ));
            }

            // Draw center line
            ctx.painter.line_segment(
                [ctx.to_screen(start), ctx.to_screen(current_cad)],
                egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(200, 200, 200, 200),
                ),
            );

            // Draw start point
            ctx.painter
                .circle_filled(ctx.to_screen(start), 4.0, egui::Color32::WHITE);
        }

        // Draw cursor crosshair
        let center = ctx.to_screen(current_cad);
        let size = 8.0;
        ctx.painter.line_segment(
            [
                egui::pos2(center.x - size, center.y),
                egui::pos2(center.x + size, center.y),
            ],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
        ctx.painter.line_segment(
            [
                egui::pos2(center.x, center.y - size),
                egui::pos2(center.x, center.y + size),
            ],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
