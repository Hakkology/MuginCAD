use crate::commands::{Command, CommandContext, PointResult};
use crate::model::{Entity, Rectangle, Vector2};

#[derive(Debug, Clone)]
pub struct RectangleCommand {
    points: Vec<Vector2>,
}

impl RectangleCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

impl Command for RectangleCommand {
    fn name(&self) -> &'static str {
        "RECTANGLE"
    }

    fn initial_prompt(&self) -> String {
        "RECTANGLE Specify first corner:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 2 {
            let p1 = self.points[0];
            let p2 = self.points[1];
            let min = Vector2::new(p1.x.min(p2.x), p1.y.min(p2.y));
            let max = Vector2::new(p1.x.max(p2.x), p1.y.max(p2.y));
            ctx.model
                .add_entity(Entity::Rectangle(Rectangle::new(min, max, ctx.filled_mode)));
            PointResult::Complete
        } else {
            PointResult::NeedMore {
                prompt: "Specify other corner:".to_string(),
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
