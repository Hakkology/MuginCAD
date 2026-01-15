use crate::commands::{Command, CommandContext, InputResult, PointResult, parse_point};
use crate::model::{Circle, Entity, Vector2};

#[derive(Debug, Clone)]
pub struct CircleCommand {
    points: Vec<Vector2>,
}

impl CircleCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

impl Command for CircleCommand {
    fn name(&self) -> &'static str {
        "CIRCLE"
    }

    fn initial_prompt(&self) -> String {
        "CIRCLE Specify center point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 2 {
            let center = self.points[0];
            let radius = center.dist(self.points[1]);
            ctx.model
                .add_entity(Entity::Circle(Circle::new(center, radius, ctx.filled_mode)));
            PointResult::Complete
        } else {
            PointResult::NeedMore {
                prompt: "Specify radius point or enter radius:".to_string(),
            }
        }
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        // Try to parse as point first
        if let Some(pos) = parse_point(input) {
            return InputResult::Point(self.push_point(pos, ctx));
        }

        // If we have center, try to parse as radius
        if self.points.len() == 1 {
            if let Ok(radius) = input.parse::<f32>() {
                if radius > 0.0 {
                    let center = self.points[0];
                    ctx.model.add_entity(Entity::Circle(Circle::new(
                        center,
                        radius,
                        ctx.filled_mode,
                    )));
                    return InputResult::Parameter(PointResult::Complete);
                }
            }
        }

        InputResult::Invalid {
            message: format!("Invalid input \"{}\".", input),
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

        if let Some(&center) = points.first() {
            let cad_radius = center.dist(current_cad);
            let screen_radius = cad_radius * ctx.zoom;

            // Draw circle
            ctx.painter
                .circle_stroke(ctx.to_screen(center), screen_radius, preview_stroke);

            // Draw radius line
            ctx.painter.line_segment(
                [ctx.to_screen(center), ctx.to_screen(current_cad)],
                preview_stroke,
            );

            // Draw center marker
            ctx.painter.circle_stroke(
                ctx.to_screen(center),
                4.0,
                egui::Stroke::new(1.5, egui::Color32::YELLOW),
            );
        }
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
