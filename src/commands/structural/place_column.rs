use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::structural::column::{Column, Orientation};
use crate::model::{Entity, Vector2};

/// Command to place column instances
#[derive(Debug, Clone)]
pub struct PlaceColumnCommand {
    type_id: String,
    orientation: Orientation,
    points: Vec<Vector2>,
}

impl PlaceColumnCommand {
    pub fn new(type_id: String) -> Self {
        Self {
            type_id,
            orientation: Orientation::Horizontal,
            points: Vec::new(),
        }
    }

    pub fn toggle_orientation(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal,
        };
    }
}

impl Command for PlaceColumnCommand {
    fn name(&self) -> &'static str {
        "COLUMN"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        format!(
            "COLUMN [{}] Click to place (H/V to toggle orientation):",
            self.type_id
        )
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        // Get next index from type manager
        let index = ctx.model.structural_types.next_column_index(&self.type_id);

        let column = Column {
            type_id: self.type_id.clone(),
            position: pos,
            rotation: 0.0,
            orientation: self.orientation,
            index,
        };

        ctx.model.add_entity(Entity::Column(column));

        PointResult::NeedMore {
            prompt: format!(
                "Placed {}-C{}. Click for next or ESC to finish:",
                self.type_id, index
            ),
        }
    }

    fn process_input(
        &mut self,
        input: &str,
        ctx: &mut CommandContext,
    ) -> crate::commands::InputResult {
        // Handle H/V toggle
        let trimmed = input.trim().to_uppercase();
        if trimmed == "H" {
            self.orientation = Orientation::Horizontal;
            return crate::commands::InputResult::Parameter(PointResult::NeedMore {
                prompt: format!(
                    "Orientation: Horizontal. Click to place [{}]:",
                    self.type_id
                ),
            });
        }
        if trimmed == "V" {
            self.orientation = Orientation::Vertical;
            return crate::commands::InputResult::Parameter(PointResult::NeedMore {
                prompt: format!("Orientation: Vertical. Click to place [{}]:", self.type_id),
            });
        }

        // Default: try to parse as point
        if let Some(pos) = crate::commands::parse_point(input) {
            let constrained =
                self.constrain_point(pos, self.get_points().last().copied(), ctx.modifiers);
            crate::commands::InputResult::Point(self.push_point(constrained, ctx))
        } else {
            crate::commands::InputResult::Invalid {
                message: format!(
                    "Invalid input \"{}\". Use H/V to toggle orientation.",
                    input
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
        _points: &[Vector2],
        current_cad: Vector2,
    ) {
        use eframe::egui;

        // Draw column preview at cursor position
        let w = 50.0; // Default preview size
        let h = 50.0;
        let (pw, ph) = match self.orientation {
            Orientation::Horizontal => (w, h),
            Orientation::Vertical => (h, w),
        };

        let half_w = pw / 2.0;
        let half_h = ph / 2.0;
        let corners = [
            Vector2::new(current_cad.x - half_w, current_cad.y - half_h),
            Vector2::new(current_cad.x + half_w, current_cad.y - half_h),
            Vector2::new(current_cad.x + half_w, current_cad.y + half_h),
            Vector2::new(current_cad.x - half_w, current_cad.y + half_h),
        ];

        let screen_corners: Vec<egui::Pos2> = corners.iter().map(|c| ctx.to_screen(*c)).collect();

        ctx.painter.add(egui::Shape::convex_polygon(
            screen_corners,
            egui::Color32::from_rgba_unmultiplied(120, 120, 120, 100),
            egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180),
            ),
        ));

        // Draw crosshair
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
