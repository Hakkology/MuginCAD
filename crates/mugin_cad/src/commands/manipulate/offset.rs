use crate::commands::{Command, CommandCategory, CommandContext, InputResult, PointResult};
use crate::model::{Entity, Line, Vector2};

define_manipulation_command!(OffsetCommand,
    selected_lines: Vec<(usize, Line)> = Vec::new(),
    offset_distance: Option<f32> = None
);

impl OffsetCommand {
    /// Calculate perpendicular offset for a line
    fn offset_line(&self, line: &Line, distance: f32, side_point: Vector2) -> Line {
        let dx = line.end.x - line.start.x;
        let dy = line.end.y - line.start.y;
        let len = (dx * dx + dy * dy).sqrt();

        if len < 0.0001 {
            return line.clone();
        }

        // Calculate perpendicular normal (normalized)
        let nx = -dy / len;
        let ny = dx / len;

        // Determine which side to offset based on side_point
        let mid = line.midpoint();
        let to_side = Vector2::new(side_point.x - mid.x, side_point.y - mid.y);

        // Dot product to determine sign
        let dot = to_side.x * nx + to_side.y * ny;
        let sign = if dot >= 0.0 { 1.0 } else { -1.0 };

        let offset_x = nx * distance * sign;
        let offset_y = ny * distance * sign;

        Line::new(
            Vector2::new(line.start.x + offset_x, line.start.y + offset_y),
            Vector2::new(line.end.x + offset_x, line.end.y + offset_y),
        )
    }
}

impl Command for OffsetCommand {
    fn name(&self) -> &'static str {
        "OFFSET"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    fn cannot_execute_message(&self) -> String {
        "No lines selected. Select lines first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        "OFFSET Specify offset distance:".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_indices = ctx.selected_indices.iter().cloned().collect();
        // Only collect lines from selected entities
        for &idx in &self.entity_indices {
            if let Some(Entity::Line(line)) = ctx.model.entities.get(idx) {
                self.selected_lines.push((idx, line.clone()));
            }
        }
    }

    fn can_execute(&self, ctx: &CommandContext) -> bool {
        // Check if there's at least one line in the selection
        ctx.selected_indices
            .iter()
            .any(|&idx| matches!(ctx.model.entities.get(idx), Some(Entity::Line(_))))
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        // If we don't have a distance yet, use the point to calculate distance from first line
        if self.offset_distance.is_none() {
            if let Some((_, line)) = self.selected_lines.first() {
                let dist = pos.dist_to_line(line.start, line.end);
                self.offset_distance = Some(dist);
                self.points.push(pos);
                return PointResult::NeedMore {
                    prompt: format!("Offset distance: {:.2}. Click side to offset:", dist),
                };
            }
        }

        // We have distance, use point to determine side and create offset lines
        self.points.push(pos);

        if let Some(distance) = self.offset_distance {
            for (_, line) in &self.selected_lines {
                let offset_line = self.offset_line(line, distance, pos);
                ctx.model.add_entity(Entity::Line(offset_line));
            }
        }

        PointResult::Complete
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        // Try to parse as distance first
        if self.offset_distance.is_none() {
            if let Ok(dist) = input.parse::<f32>() {
                if dist > 0.0 {
                    self.offset_distance = Some(dist);
                    return InputResult::Parameter(PointResult::NeedMore {
                        prompt: format!("Offset distance: {:.2}. Click side to offset:", dist),
                    });
                } else {
                    return InputResult::Invalid {
                        message: "Offset distance must be positive.".to_string(),
                    };
                }
            }
        }

        // Fall back to default point parsing
        if let Some(pos) = crate::commands::parse_point(input) {
            InputResult::Point(self.push_point(pos, ctx))
        } else {
            InputResult::Invalid {
                message: format!(
                    "Invalid input \"{}\". Enter distance or coordinates.",
                    input
                ),
            }
        }
    }

    impl_command_common!(OffsetCommand);
}
