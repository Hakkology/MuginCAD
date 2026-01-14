use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Vector2;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct RotateCommand {
    points: Vec<Vector2>,
    entity_indices: Vec<usize>,
}

impl RotateCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            entity_indices: Vec::new(),
        }
    }
}

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
        self.entity_indices = ctx.selected_indices.iter().cloned().collect();
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

            for &idx in &self.entity_indices {
                if let Some(entity) = ctx.model.entities.get_mut(idx) {
                    entity.rotate(pivot, angle);
                }
            }

            PointResult::Complete
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
