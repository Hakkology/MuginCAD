use crate::commands::{Command, CommandCategory, CommandContext, InputResult, PointResult};
use crate::model::shapes::arc::Arc;
use crate::model::{Entity, Vector2};

/// Arc command - draws an arc from 3 points: center, start point, end point
#[derive(Debug, Clone)]
pub struct ArcCommand {
    points: Vec<Vector2>,
    filled: bool,
    /// If true, arc goes clockwise (CW), otherwise counter-clockwise (CCW)
    pub clockwise: bool,
}

impl ArcCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            filled: false,
            clockwise: false, // Default to CCW
        }
    }

    /// Toggle the arc direction between CW and CCW
    pub fn toggle_direction(&mut self) {
        self.clockwise = !self.clockwise;
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
        "ARC Specify center point:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);
        self.filled = ctx.filled_mode;

        match self.points.len() {
            1 => PointResult::NeedMore {
                prompt: "Specify start point of arc:".to_string(),
            },
            2 => PointResult::NeedMore {
                prompt: format!(
                    "Specify end point [{}] (type 'r' to reverse):",
                    if self.clockwise { "CW" } else { "CCW" }
                ),
            },
            3 => {
                let arc = Arc::from_three_points_directed(
                    self.points[0], // center
                    self.points[1], // start
                    self.points[2], // end
                    self.filled,
                    self.clockwise,
                );
                ctx.model.add_entity(Entity::Arc(arc));
                PointResult::Complete
            }
            _ => PointResult::Complete,
        }
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        let clean = input.trim().to_lowercase();

        // Handle 'r' or 'reverse' to toggle direction (only when we have 2 points)
        if self.points.len() == 2 && (clean == "r" || clean == "reverse") {
            self.toggle_direction();
            return InputResult::Parameter(PointResult::NeedMore {
                prompt: format!(
                    "Specify end point [{}] (type 'r' to reverse):",
                    if self.clockwise { "CW" } else { "CCW" }
                ),
            });
        }

        // Fall back to default point parsing
        if let Some(pos) = crate::commands::parse_point(input) {
            InputResult::Point(self.push_point(pos, ctx))
        } else {
            InputResult::Invalid {
                message: format!(
                    "Invalid input \"{}\". Enter point or 'r' to reverse.",
                    input
                ),
            }
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}
