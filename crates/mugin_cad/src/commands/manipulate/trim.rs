use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::math::geometry;
use crate::model::{Entity, Vector2};

define_command!(TrimCommand);

impl Command for TrimCommand {
    fn name(&self) -> &'static str {
        "Trim"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation // No selection required
    }

    fn initial_prompt(&self) -> String {
        "Click on the portion of line to trim:".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        // Find the line closest to click position
        let tolerance = 10.0;
        let mut best_line_idx: Option<usize> = None;
        let mut best_dist = f32::MAX;

        for (i, entity) in ctx.model.entities.iter().enumerate() {
            if let Entity::Line(line) = entity {
                let dist = geometry::point_to_line_distance(pos, line.start, line.end);
                if dist < tolerance && dist < best_dist {
                    best_dist = dist;
                    best_line_idx = Some(i);
                }
            }
        }

        if let Some(line_idx) = best_line_idx {
            // Find all intersection points with other entities
            let line = if let Entity::Line(l) = &ctx.model.entities[line_idx] {
                l.clone()
            } else {
                return PointResult::NeedMore {
                    prompt: "Click on a line to trim:".to_string(),
                };
            };

            let mut intersections: Vec<Vector2> = Vec::new();

            for (i, entity) in ctx.model.entities.iter().enumerate() {
                if i == line_idx {
                    continue;
                }

                match entity {
                    Entity::Line(other) => {
                        if let Some(pt) = geometry::line_line_intersection(
                            line.start,
                            line.end,
                            other.start,
                            other.end,
                        ) {
                            intersections.push(pt);
                        }
                    }
                    Entity::Circle(circle) => {
                        let pts = geometry::line_circle_intersection(
                            line.start,
                            line.end,
                            circle.center,
                            circle.radius,
                        );
                        intersections.extend(pts);
                    }
                    Entity::Rectangle(rect) => {
                        let corners = [
                            rect.min,
                            Vector2::new(rect.max.x, rect.min.y),
                            rect.max,
                            Vector2::new(rect.min.x, rect.max.y),
                        ];
                        for j in 0..4 {
                            let a = corners[j];
                            let b = corners[(j + 1) % 4];
                            if let Some(pt) =
                                geometry::line_line_intersection(line.start, line.end, a, b)
                            {
                                intersections.push(pt);
                            }
                        }
                    }
                    Entity::Arc(arc) => {
                        let pts = geometry::line_circle_intersection(
                            line.start, line.end, arc.center, arc.radius,
                        );
                        intersections.extend(pts);
                    }
                    Entity::Text(_) => {
                        // Text annotations don't contribute to intersections
                    }
                }
            }

            if intersections.is_empty() {
                return PointResult::NeedMore {
                    prompt: "No intersections found. Click another line:".to_string(),
                };
            }

            let click_t = geometry::project_point_on_line(pos, line.start, line.end);

            // Sort intersections by parameter t along the line
            let mut intersection_ts: Vec<(f32, Vector2)> = intersections
                .iter()
                .map(|&pt| {
                    (
                        geometry::project_point_on_line(pt, line.start, line.end),
                        pt,
                    )
                })
                .filter(|(t, _)| *t > 0.0 && *t < 1.0)
                .collect();

            intersection_ts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            if intersection_ts.is_empty() {
                return PointResult::NeedMore {
                    prompt: "No valid intersections on line segment. Click another line:"
                        .to_string(),
                };
            }

            // Find intersections on either side of click
            let mut left_intersection: Option<Vector2> = None;
            let mut right_intersection: Option<Vector2> = None;

            for (t, pt) in &intersection_ts {
                if *t < click_t {
                    left_intersection = Some(*pt);
                } else if right_intersection.is_none() {
                    right_intersection = Some(*pt);
                }
            }

            // Modify the line based on which side to trim
            if let Entity::Line(ref mut line_entity) = ctx.model.entities[line_idx] {
                match (left_intersection, right_intersection) {
                    (Some(left), Some(_right)) => {
                        line_entity.end = left;
                    }
                    (Some(left), None) => {
                        line_entity.end = left;
                    }
                    (None, Some(right)) => {
                        line_entity.start = right;
                    }
                    (None, None) => {}
                }
            }

            PointResult::NeedMore {
                prompt: "Trimmed! Click another line or press Enter/Escape to exit:".to_string(),
            }
        } else {
            PointResult::NeedMore {
                prompt: "No line found. Click on a line to trim:".to_string(),
            }
        }
    }

    impl_command_common!(TrimCommand);
}
