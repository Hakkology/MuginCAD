use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::Entity;
use crate::model::Vector2;
use crate::model::math::geometry;

define_command!(MeasureAreaCommand);

impl Command for MeasureAreaCommand {
    fn name(&self) -> &'static str {
        "Measure Area"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn initial_prompt(&self) -> String {
        "Click inside a closed region to measure Area:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        if let Some((_, vertices)) = geometry::find_closed_region(&ctx.model, pos) {
            let area = geometry::calculate_polygon_area(&vertices);
            let centroid = geometry::calculate_centroid(&vertices);

            let annotation = crate::model::TextAnnotation::new_area(centroid, area, vertices);
            ctx.model.add_entity(Entity::Text(annotation));

            PointResult::Complete
        } else {
            PointResult::NeedMore {
                prompt: "Region not closed or empty. Try another point.".to_string(),
            }
        }
    }

    impl_command_common!(MeasureAreaCommand);
}
