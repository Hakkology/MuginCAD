use crate::commands::{
    Command, CommandCategory, CommandContext, InputResult, PointResult, parse_point,
};
use crate::model::Vector2;

#[derive(Debug, Clone)]
pub struct ScaleCommand {
    points: Vec<Vector2>,
    entity_indices: Vec<usize>,
}

impl ScaleCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            entity_indices: Vec::new(),
        }
    }
}

impl Command for ScaleCommand {
    fn name(&self) -> &'static str {
        "SCALE"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    fn cannot_execute_message(&self) -> String {
        "No entities selected. Select entities first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        "SCALE Specify base point:".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_indices = ctx.selected_indices.iter().cloned().collect();
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.points.push(pos);

        if self.points.len() == 1 {
            PointResult::NeedMore {
                prompt: "Specify scale factor or second point:".to_string(),
            }
        } else {
            // Calculate scale factor from distance ratio
            let base = self.points[0];
            let to = pos;

            if !self.entity_indices.is_empty() {
                // Use the first entity to calculate the scale factor
                // This provides a consistent feel based on the first selected item
                let first_idx = self.entity_indices[0];
                let scale_factor = if let Some(entity) = ctx.model.entities.get(first_idx) {
                    let center = entity.center();
                    let base_dist = base.dist(center);
                    let to_dist = to.dist(center);

                    if base_dist > 0.1 {
                        Some(to_dist / base_dist)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(factor) = scale_factor {
                    for &idx in &self.entity_indices {
                        if let Some(entity) = ctx.model.entities.get_mut(idx) {
                            // Scale each entity around its own center
                            let center = entity.center();
                            entity.scale(center, factor);
                        }
                    }
                }
            }

            PointResult::Complete
        }
    }

    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        // Try to parse as point first
        if let Some(pos) = parse_point(input) {
            return InputResult::Point(self.push_point(pos, ctx));
        }

        // Try to parse as scale factor
        if self.points.len() == 1 {
            if let Ok(factor) = input.parse::<f32>() {
                if factor > 0.0 {
                    let base = self.points[0];

                    for &idx in &self.entity_indices {
                        if let Some(entity) = ctx.model.entities.get_mut(idx) {
                            // Scale around the base point specified by user
                            entity.scale(base, factor);
                        }
                    }

                    return InputResult::Parameter(PointResult::Complete);
                }
            }
        }

        InputResult::Invalid {
            message: format!("Invalid input \"{}\".", input),
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
