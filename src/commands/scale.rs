use crate::commands::{
    Command, CommandCategory, CommandContext, InputResult, PointResult, parse_point,
};
use crate::model::Vector2;

#[derive(Debug, Clone)]
pub struct ScaleCommand {
    points: Vec<Vector2>,
    entity_idx: Option<usize>,
}

impl ScaleCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            entity_idx: None,
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

    fn can_execute(&self, ctx: &CommandContext) -> bool {
        ctx.selected_entity_idx.is_some()
    }

    fn cannot_execute_message(&self) -> String {
        "No entity selected. Select an entity first.".to_string()
    }

    fn initial_prompt(&self) -> String {
        "SCALE Specify base point:".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.entity_idx = ctx.selected_entity_idx;
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

            // Get entity center for scaling
            if let Some(idx) = self.entity_idx {
                if let Some(entity) = ctx.model.entities.get(idx) {
                    let center = entity.center();
                    let base_dist = base.dist(center);
                    let to_dist = to.dist(center);

                    if base_dist > 0.1 {
                        let scale_factor = to_dist / base_dist;
                        if let Some(entity) = ctx.model.entities.get_mut(idx) {
                            entity.scale(center, scale_factor);
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

                    if let Some(idx) = self.entity_idx {
                        if let Some(entity) = ctx.model.entities.get_mut(idx) {
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
