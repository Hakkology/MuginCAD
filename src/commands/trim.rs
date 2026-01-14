use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::{Entity, Vector2};

/// Trim command - trims lines at intersection points
#[derive(Debug, Clone)]
pub struct TrimCommand {
    points: Vec<Vector2>,
}

impl TrimCommand {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

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
                let dist = point_to_line_distance(pos, line.start, line.end);
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
                        if let Some(pt) =
                            line_line_intersection(line.start, line.end, other.start, other.end)
                        {
                            intersections.push(pt);
                        }
                    }
                    Entity::Circle(circle) => {
                        let pts = line_circle_intersection(
                            line.start,
                            line.end,
                            circle.center,
                            circle.radius,
                        );
                        intersections.extend(pts);
                    }
                    Entity::Rectangle(rect) => {
                        // Rectangle has 4 edges
                        let corners = [
                            rect.min,
                            Vector2::new(rect.max.x, rect.min.y),
                            rect.max,
                            Vector2::new(rect.min.x, rect.max.y),
                        ];
                        for j in 0..4 {
                            let a = corners[j];
                            let b = corners[(j + 1) % 4];
                            if let Some(pt) = line_line_intersection(line.start, line.end, a, b) {
                                intersections.push(pt);
                            }
                        }
                    }
                }
            }

            if intersections.is_empty() {
                return PointResult::NeedMore {
                    prompt: "No intersections found. Click another line:".to_string(),
                };
            }

            // Find the nearest intersection to the click point
            // and determine which side of the line to keep
            let click_t = project_point_on_line(pos, line.start, line.end);

            // Sort intersections by parameter t along the line
            let mut intersection_ts: Vec<(f32, Vector2)> = intersections
                .iter()
                .map(|&pt| (project_point_on_line(pt, line.start, line.end), pt))
                .filter(|(t, _)| *t > 0.0 && *t < 1.0) // Only intersections within the line segment
                .collect();

            intersection_ts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            if intersection_ts.is_empty() {
                return PointResult::NeedMore {
                    prompt: "No valid intersections on line segment. Click another line:"
                        .to_string(),
                };
            }

            // Find which segment the click is in and trim accordingly
            // The click divides the line - we remove the portion containing the click

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
                        // Click is between two intersections - trim to left intersection
                        line_entity.end = left;
                    }
                    (Some(left), None) => {
                        // Click is after last intersection - trim from left
                        line_entity.end = left;
                    }
                    (None, Some(right)) => {
                        // Click is before first intersection - trim from right
                        line_entity.start = right;
                    }
                    (None, None) => {
                        // This shouldn't happen if we have valid intersections
                    }
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

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

/// Calculate distance from a point to a line segment
fn point_to_line_distance(p: Vector2, a: Vector2, b: Vector2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let len_sq = ab.x * ab.x + ab.y * ab.y;

    if len_sq == 0.0 {
        return ((p.x - a.x).powi(2) + (p.y - a.y).powi(2)).sqrt();
    }

    let t = ((ap.x * ab.x + ap.y * ab.y) / len_sq).clamp(0.0, 1.0);
    let closest = Vector2::new(a.x + t * ab.x, a.y + t * ab.y);
    ((p.x - closest.x).powi(2) + (p.y - closest.y).powi(2)).sqrt()
}

/// Project point onto line and return parameter t (0 = start, 1 = end)
fn project_point_on_line(p: Vector2, a: Vector2, b: Vector2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let len_sq = ab.x * ab.x + ab.y * ab.y;
    if len_sq == 0.0 {
        return 0.0;
    }
    (ap.x * ab.x + ap.y * ab.y) / len_sq
}

/// Line-line intersection (returns None if parallel or no intersection within segments)
fn line_line_intersection(a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2) -> Option<Vector2> {
    let d1 = a2 - a1;
    let d2 = b2 - b1;
    let cross = d1.x * d2.y - d1.y * d2.x;

    if cross.abs() < 1e-10 {
        return None; // Parallel
    }

    let d = b1 - a1;
    let t = (d.x * d2.y - d.y * d2.x) / cross;
    let u = (d.x * d1.y - d.y * d1.x) / cross;

    // Check if intersection is within both line segments
    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        Some(Vector2::new(a1.x + t * d1.x, a1.y + t * d1.y))
    } else {
        None
    }
}

/// Line-circle intersection (returns 0, 1, or 2 points)
fn line_circle_intersection(
    p1: Vector2,
    p2: Vector2,
    center: Vector2,
    radius: f32,
) -> Vec<Vector2> {
    let d = p2 - p1;
    let f = p1 - center;

    let a = d.x * d.x + d.y * d.y;
    let b = 2.0 * (f.x * d.x + f.y * d.y);
    let c = f.x * f.x + f.y * f.y - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return Vec::new();
    }

    let mut results = Vec::new();
    let sqrt_disc = discriminant.sqrt();

    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);

    if t1 >= 0.0 && t1 <= 1.0 {
        results.push(Vector2::new(p1.x + t1 * d.x, p1.y + t1 * d.y));
    }
    if t2 >= 0.0 && t2 <= 1.0 && (t2 - t1).abs() > 1e-6 {
        results.push(Vector2::new(p1.x + t2 * d.x, p1.y + t2 * d.y));
    }

    results
}
