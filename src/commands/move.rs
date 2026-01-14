use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Vector2;

#[derive(Debug, Clone)]
pub struct MoveCommand {
    points: Vec<Vector2>,
    entity_idx: Option<usize>,
}

impl MoveCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            entity_idx: None,
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

    fn can_execute(&self, ctx: &CommandContext) -> bool {
        ctx.selected_entity_idx.is_some()
    }

    fn cannot_execute_message(&self) -> String {
        "No entity selected. Select an entity first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        "MOVE Specify base point:".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_idx = ctx.selected_entity_idx;
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

            if let Some(idx) = self.entity_idx {
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

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
