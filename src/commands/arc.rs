use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::arc::Arc;
use crate::model::{Entity, Vector2};

/// Arc command - draws an arc from 3 points: center, start point, end point
#[derive(Debug, Clone)]
pub struct ArcCommand {
    points: Vec<Vector2>,
    filled: bool,
}

impl ArcCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            filled: false,
        }
    }
}

impl Command for ArcCommand {
    fn name(&self) -> &'static str {
        "Arc"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "Specify center point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);
        self.filled = ctx.filled_mode;

        match self.points.len() {
            1 => PointResult::NeedMore {
                prompt: "Specify start point of arc:".to_string(),
            },
            2 => PointResult::NeedMore {
                prompt: "Specify end point of arc:".to_string(),
            },
            3 => {
                let arc = Arc::from_three_points(
                    self.points[0], // center
                    self.points[1], // start
                    self.points[2], // end
                    self.filled,
                );
                ctx.model.add_entity(Entity::Arc(arc));
                PointResult::Complete
            }
            _ => PointResult::Complete,
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
