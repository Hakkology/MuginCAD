use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::{Entity, Vector2};

#[derive(Debug, Clone)]
pub struct CopyCommand {
    points: Vec<Vector2>,
    entity_indices: Vec<usize>,
    copied_entities: Vec<Entity>,
    is_cut: bool,
}

impl CopyCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            entity_indices: Vec::new(),
            copied_entities: Vec::new(),
            is_cut: false,
        }
    }

    pub fn new_cut() -> Self {
        Self {
            points: Vec::new(),
            entity_indices: Vec::new(),
            copied_entities: Vec::new(),
            is_cut: true,
        }
    }
}

impl Command for CopyCommand {
    fn name(&self) -> &'static str {
        if self.is_cut { "CUT" } else { "COPY" }
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    fn cannot_execute_message(&self) -> String {
        "No entities selected. Select entities first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        if self.is_cut {
            "CUT Specify base point:".to_string()
        } else {
            "COPY Specify base point:".to_string()
        }
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_indices = ctx.selected_indices.iter().cloned().collect();
        // Clone the selected entities
        for &idx in &self.entity_indices {
            if let Some(entity) = ctx.model.entities.get(idx) {
                self.copied_entities.push(entity.clone());
            }
        }
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 1 {
            PointResult::NeedMore {
                prompt: "Specify destination point:".to_string(),
            }
        } else {
            // Calculate delta and create copies
            let from = self.points[0];
            let to = pos;
            let delta = to - from;

            // Add copied entities with translation
            for mut entity in self.copied_entities.clone() {
                entity.translate(delta);
                ctx.model.add_entity(entity);
            }

            // If cut, delete the originals
            if self.is_cut {
                // Sort indices in descending order to avoid shifting issues
                let mut sorted_indices = self.entity_indices.clone();
                sorted_indices.sort_by(|a, b| b.cmp(a));

                for idx in sorted_indices {
                    if idx < ctx.model.entities.len() {
                        ctx.model.entities.remove(idx);
                    }
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

        if let Some(&base) = points.first() {
            ctx.painter.line_segment(
                [ctx.to_screen(base), ctx.to_screen(current_cad)],
                egui::Stroke::new(1.5, egui::Color32::from_rgb(100, 200, 255)),
            );
            ctx.painter.circle_stroke(
                ctx.to_screen(base),
                4.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255)),
            );
            ctx.painter.circle_filled(
                ctx.to_screen(current_cad),
                3.0,
                egui::Color32::from_rgb(100, 200, 255),
            );
        }
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
