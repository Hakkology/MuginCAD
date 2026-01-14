use crate::commands::{Command, CommandContext, PointResult};
use crate::model::{Entity, Line, Vector2};

#[derive(Debug, Clone)]
pub struct LineCommand {
    points: Vec<Vector2>,
}

impl LineCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

impl Command for LineCommand {
    fn name(&self) -> &'static str {
        "LINE"
    }

    fn initial_prompt(&self) -> String {
        "LINE Specify first point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        // Apply ortho constraint if shift is pressed
        let constrained_pos = if let Some(&last) = self.points.last() {
            self.constrain_point(pos, Some(last), ctx.modifiers)
        } else {
            pos
        };

        self.points.push(constrained_pos);

        if self.points.len() >= 2 {
            let start = self.points[self.points.len() - 2];
            let end = self.points[self.points.len() - 1];
            ctx.model.add_entity(Entity::Line(Line::new(start, end)));

            PointResult::NeedMore {
                prompt: "Specify next point (Shift for ortho):".to_string(),
            }
        } else {
            PointResult::NeedMore {
                prompt: "Specify next point (Shift for ortho):".to_string(),
            }
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
