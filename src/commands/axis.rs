use crate::commands::{Command, CommandCategory, CommandContext, InputResult, PointResult};
use crate::model::Vector2;
use crate::model::axis::AxisOrientation;

#[derive(Debug, Clone)]
pub struct AxisCommand {
    orientation: Option<AxisOrientation>,
    points: Vec<Vector2>,
}

impl AxisCommand {
    pub fn new() -> Self {
        Self {
            orientation: None,
            points: Vec::new(),
        }
    }
}

impl Command for AxisCommand {
    fn name(&self) -> &'static str {
        "AXIS"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "AXIS Enter orientation (H=horizontal, V=vertical):".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        if self.orientation.is_none() {
            // Can't push point without orientation
            return PointResult::NeedMore {
                prompt: "Enter orientation first (H or V):".to_string(),
            };
        }

        self.points.push(pos);

        // Add axis based on orientation
        match self.orientation.unwrap() {
            AxisOrientation::Vertical => {
                ctx.model.axis_manager.add_vertical(pos.x);
            }
            AxisOrientation::Horizontal => {
                ctx.model.axis_manager.add_horizontal(pos.y);
            }
        }

        // Stay in command for multiple axes
        PointResult::NeedMore {
            prompt: format!(
                "Specify {} axis position or Enter to finish:",
                if self.orientation == Some(AxisOrientation::Vertical) {
                    "vertical"
                } else {
                    "horizontal"
                }
            ),
        }
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        let clean = input.trim().to_lowercase();

        // If no orientation yet, parse it
        if self.orientation.is_none() {
            match clean.as_str() {
                "h" | "horizontal" => {
                    self.orientation = Some(AxisOrientation::Horizontal);
                    return InputResult::Parameter(PointResult::NeedMore {
                        prompt: "AXIS (H) Click position or enter Y coordinate:".to_string(),
                    });
                }
                "v" | "vertical" => {
                    self.orientation = Some(AxisOrientation::Vertical);
                    return InputResult::Parameter(PointResult::NeedMore {
                        prompt: "AXIS (V) Click position or enter X coordinate:".to_string(),
                    });
                }
                _ => {
                    return InputResult::Invalid {
                        message: "Enter H for horizontal or V for vertical".to_string(),
                    };
                }
            }
        }

        // Try to parse as coordinate
        if let Ok(coord) = clean.parse::<f32>() {
            match self.orientation.unwrap() {
                AxisOrientation::Vertical => {
                    ctx.model.axis_manager.add_vertical(coord);
                    self.points.push(Vector2::new(coord, 0.0));
                }
                AxisOrientation::Horizontal => {
                    ctx.model.axis_manager.add_horizontal(coord);
                    self.points.push(Vector2::new(0.0, coord));
                }
            }
            return InputResult::Parameter(PointResult::NeedMore {
                prompt: format!(
                    "Specify next {} axis position or Enter to finish:",
                    if self.orientation == Some(AxisOrientation::Vertical) {
                        "vertical"
                    } else {
                        "horizontal"
                    }
                ),
            });
        }

        // Try to parse as point (x,y)
        if let Some(pos) = crate::commands::parse_point(input) {
            return InputResult::Point(self.push_point(pos, ctx));
        }

        InputResult::Invalid {
            message: format!(
                "Invalid input \"{}\". Enter coordinate or click position.",
                input
            ),
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
