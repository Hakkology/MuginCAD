use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Vector2;

#[derive(Debug, Clone)]
pub struct MoveCommand {
    points: Vec<Vector2>,
    entity_indices: Vec<usize>,
}

impl MoveCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            entity_indices: Vec::new(),
        }
    }
}

impl Command for MoveCommand {
    fn name(&self) -> &'static str {
        "MOVE"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    // Rely on default implementation for can_execute

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
        if let Some(&base) = points.first() {
            ctx.painter.line_segment(
                [ctx.to_screen(base), ctx.to_screen(current_cad)],
                preview_stroke,
            );
            ctx.painter
                .circle_stroke(ctx.to_screen(base), 4.0, preview_stroke);
            ctx.painter
                .circle_filled(ctx.to_screen(current_cad), 3.0, egui::Color32::WHITE);
        }
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
