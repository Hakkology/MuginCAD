use crate::commands::preview;
use crate::commands::{Command, CommandCategory, CommandContext, InputResult, PointResult};
use crate::model::shapes::arc::Arc;
use crate::model::{Entity, Vector2};

define_command!(ArcCommand,
    filled: bool = false,
    clockwise: bool = false
);

impl ArcCommand {
    /// Toggle the arc direction between CW and CCW
    pub fn toggle_direction(&mut self) {
        self.clockwise = !self.clockwise;
    }
}

impl Command for ArcCommand {
    fn name(&self) -> &'static str {
        "Arc"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "ARC Specify center point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);
        self.filled = ctx.filled_mode;

        match self.points.len() {
            1 => PointResult::NeedMore {
                prompt: "Specify start point of arc:".to_string(),
            },
            2 => PointResult::NeedMore {
                prompt: format!(
                    "Specify end point [{}] (type 'r' to reverse):",
                    if self.clockwise { "CW" } else { "CCW" }
                ),
            },
            3 => {
                let arc = Arc::from_three_points_directed(
                    self.points[0], // center
                    self.points[1], // start
                    self.points[2], // end
                    self.filled,
                    self.clockwise,
                );
                ctx.model.add_entity(Entity::Arc(arc));
                PointResult::Complete
            }
            _ => PointResult::Complete,
        }
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        let clean = input.trim().to_lowercase();

        // Handle 'r' or 'reverse' to toggle direction (only when we have 2 points)
        if self.points.len() == 2 && (clean == "r" || clean == "reverse") {
            self.toggle_direction();
            return InputResult::Parameter(PointResult::NeedMore {
                prompt: format!(
                    "Specify end point [{}] (type 'r' to reverse):",
                    if self.clockwise { "CW" } else { "CCW" }
                ),
            });
        }

        // Fall back to default point parsing
        if let Some(pos) = crate::commands::parse_point(input) {
            InputResult::Point(self.push_point(pos, ctx))
        } else {
            InputResult::Invalid {
                message: format!(
                    "Invalid input \"{}\". Enter point or 'r' to reverse.",
                    input
                ),
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

        match points.len() {
            1 => {
                let center = points[0];
                preview::draw_line_to_cursor(ctx, center, current_cad);
                preview::draw_center_marker(ctx, center);
                let radius = center.dist(current_cad) * ctx.zoom;
                ctx.painter.circle_stroke(
                    ctx.to_screen(center),
                    radius,
                    egui::Stroke::new(
                        0.5,
                        egui::Color32::from_rgba_unmultiplied(150, 150, 150, 80),
                    ),
                );
            }
            2 => {
                let center = points[0];
                let start = points[1];
                let start_angle = (start.y - center.y).atan2(start.x - center.x);
                let end_angle = (current_cad.y - center.y).atan2(current_cad.x - center.x);

                let is_clockwise = self.clockwise;

                let segments = 32;
                let angle_range = if is_clockwise {
                    let mut range = start_angle - end_angle;
                    if range < 0.0 {
                        range += std::f32::consts::PI * 2.0;
                    }
                    -range
                } else {
                    let mut range = end_angle - start_angle;
                    if range < 0.0 {
                        range += std::f32::consts::PI * 2.0;
                    }
                    range
                };

                let angle_step = angle_range / segments as f32;
                let radius = center.dist(start);

                let mut arc_points = Vec::with_capacity(segments + 1);
                for i in 0..=segments {
                    let angle = start_angle + angle_step * i as f32;
                    let pt = Vector2::new(
                        center.x + radius * angle.cos(),
                        center.y + radius * angle.sin(),
                    );
                    arc_points.push(ctx.to_screen(pt));
                }

                let stroke = preview::preview_stroke();
                for i in 0..arc_points.len() - 1 {
                    ctx.painter
                        .line_segment([arc_points[i], arc_points[i + 1]], stroke);
                }

                // Center and start markers
                preview::draw_point_marker(ctx, center, egui::Color32::YELLOW);
                preview::draw_point_marker(ctx, start, egui::Color32::GREEN);
                preview::draw_line_to_cursor(ctx, center, current_cad);
            }
            _ => {}
        }
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }

    impl_command_common!(ArcCommand);
}
