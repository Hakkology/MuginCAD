use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::{Entity, Vector2};

define_manipulation_command!(CopyCommand,
    copied_entities: Vec<Entity> = Vec::new(),
    is_cut: bool = false
);

impl CopyCommand {
    pub fn new_cut() -> Self {
        Self {
            points: Vec::new(),
            entity_ids: Vec::new(),
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
        self.entity_ids = ctx.model.get_top_level_selected_ids(&ctx.selected_ids);
        // Clone the selected entities
        for &id in &self.entity_ids {
            if let Some(entity) = ctx.model.find_by_id(id) {
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
                let ids_set: std::collections::HashSet<u64> =
                    self.entity_ids.iter().cloned().collect();
                ctx.model.remove_entities_by_ids(&ids_set);
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
        use crate::commands::preview;
        use eframe::egui;

        if let Some(&base) = points.first() {
            let color = egui::Color32::from_rgb(100, 200, 255);
            ctx.painter.line_segment(
                [ctx.to_screen(base), ctx.to_screen(current_cad)],
                egui::Stroke::new(1.5, color),
            );
            ctx.painter
                .circle_stroke(ctx.to_screen(base), 4.0, egui::Stroke::new(1.0, color));
            preview::draw_point_marker(ctx, current_cad, color);
        }
    }

    impl_command_common!(CopyCommand);
}
