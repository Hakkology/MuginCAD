use crate::commands::preview;
use crate::commands::{
    Command, CommandCategory, CommandContext, InputResult, PointResult, parse_point,
};
use crate::model::Vector2;

define_manipulation_command!(ScaleCommand);

impl Command for ScaleCommand {
    fn name(&self) -> &'static str {
        "SCALE"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    fn cannot_execute_message(&self) -> String {
        "No entities selected. Select entities first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        "SCALE Specify base point:".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_indices = ctx.selected_indices.iter().cloned().collect();
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 1 {
            PointResult::NeedMore {
                prompt: "Specify scale factor or second point:".to_string(),
            }
        } else {
            // Calculate scale factor from distance ratio
            let base = self.points[0];
            let to = pos;

            if !self.entity_indices.is_empty() {
                let first_idx = self.entity_indices[0];
                let scale_factor = if let Some(entity) = ctx.model.entities.get(first_idx) {
                    let center = entity.center();
                    let base_dist = base.dist(center);
                    let to_dist = to.dist(center);

                    if base_dist > 0.1 {
                        Some(to_dist / base_dist)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(factor) = scale_factor {
                    for &idx in &self.entity_indices {
                        if let Some(entity) = ctx.model.entities.get_mut(idx) {
                            let center = entity.center();
                            entity.scale(center, factor);
                        }
                    }
                }
            }

            PointResult::Complete
        }
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        // Try to parse as point first
        if let Some(pos) = parse_point(input) {
            return InputResult::Point(self.push_point(pos, ctx));
        }

        // Try to parse as scale factor
        if self.points.len() == 1 {
            if let Ok(factor) = input.parse::<f32>() {
                if factor > 0.0 {
                    let base = self.points[0];

                    for &idx in &self.entity_indices {
                        if let Some(entity) = ctx.model.entities.get_mut(idx) {
                            entity.scale(base, factor);
                        }
                    }

                    return InputResult::Parameter(PointResult::Complete);
                }
            }
        }

        InputResult::Invalid {
            message: format!("Invalid input \"{}\".", input),
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
            // Draw base marker (filled square)
            let base_screen = ctx.to_screen(base);
            let size = 4.0;
            ctx.painter.rect_filled(
                egui::Rect::from_center_size(base_screen, egui::vec2(size * 2.0, size * 2.0)),
                0.0,
                egui::Color32::WHITE,
            );
        }
    }

    impl_command_common!(ScaleCommand);
}
