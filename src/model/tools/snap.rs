use crate::model::{CadModel, Entity, Vector2};

/// Types of snap points
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnapPointType {
    /// Endpoint of a line
    Endpoint,
    /// Center of a circle or rectangle
    Center,
    /// Corner of a rectangle
    Corner,
    /// Intersection between two entities
    Intersection,
    /// Midpoint of a line
    Midpoint,
    /// Point on an axis line
    AxisLine,
    /// Grid intersection point
    Grid,
}

/// A snap point with its position and type
#[derive(Debug, Clone, Copy)]
pub struct SnapPoint {
    pub position: Vector2,
    pub point_type: SnapPointType,
}

impl SnapPoint {
    pub fn new(position: Vector2, point_type: SnapPointType) -> Self {
        Self {
            position,
            point_type,
        }
    }
}

use crate::model::config::AppConfig;

/// Snap system that finds snap points from entities
pub struct SnapSystem;

impl SnapSystem {
    pub fn new() -> Self {
        Self
    }

    /// Find the nearest snap point to a position
    pub fn find_nearest(
        &self,
        pos: Vector2,
        model: &CadModel,
        config: &AppConfig,
    ) -> Option<SnapPoint> {
        let mut nearest: Option<(SnapPoint, f32)> = None;
        let tolerance = config.snap_config.tolerance;

        // 1. Entity Snaps
        for entity in &model.entities {
            for snap_point in self.get_entity_snap_points(entity) {
                let dist = pos.dist(snap_point.position);
                if dist <= tolerance {
                    if nearest.is_none() || dist < nearest.unwrap().1 {
                        nearest = Some((snap_point, dist));
                    }
                }
            }
        }

        // 2. Intersection Snaps
        for (i, entity_a) in model.entities.iter().enumerate() {
            for entity_b in model.entities.iter().skip(i + 1) {
                for intersection in self.find_intersections(entity_a, entity_b) {
                    let dist = pos.dist(intersection);
                    if dist <= tolerance {
                        let snap_point = SnapPoint::new(intersection, SnapPointType::Intersection);
                        if nearest.is_none() || dist < nearest.unwrap().1 {
                            nearest = Some((snap_point, dist));
                        }
                    }
                }
            }
        }

        // 3. Axis Snaps (Akslar)
        // Check intersections between axes (Axis Intersections)
        for (i, axis_a) in model.axis_manager.axes.iter().enumerate() {
            for axis_b in model.axis_manager.axes.iter().skip(i + 1) {
                // If one is vertical and other horizontal, they intersect
                if axis_a.orientation != axis_b.orientation {
                    let intersection = match axis_a.orientation {
                        crate::model::axis::AxisOrientation::Vertical => {
                            Vector2::new(axis_a.position, axis_b.position)
                        }
                        crate::model::axis::AxisOrientation::Horizontal => {
                            Vector2::new(axis_b.position, axis_a.position)
                        }
                    };

                    let dist = pos.dist(intersection);
                    if dist <= tolerance {
                        let snap_point = SnapPoint::new(intersection, SnapPointType::Intersection);
                        if nearest.is_none() || dist < nearest.unwrap().1 {
                            nearest = Some((snap_point, dist));
                        }
                    }
                }
            }

            // Snap to axis line itself (projection)
            match axis_a.orientation {
                crate::model::axis::AxisOrientation::Vertical => {
                    if (pos.x - axis_a.position).abs() <= tolerance {
                        let snap_point = SnapPoint::new(
                            Vector2::new(axis_a.position, pos.y),
                            SnapPointType::AxisLine,
                        );
                        let dist = (pos.x - axis_a.position).abs();
                        if nearest.is_none() || dist < nearest.unwrap().1 {
                            nearest = Some((snap_point, dist));
                        }
                    }
                }
                crate::model::axis::AxisOrientation::Horizontal => {
                    if (pos.y - axis_a.position).abs() <= tolerance {
                        let snap_point = SnapPoint::new(
                            Vector2::new(pos.x, axis_a.position),
                            SnapPointType::AxisLine,
                        );
                        let dist = (pos.y - axis_a.position).abs();
                        if nearest.is_none() || dist < nearest.unwrap().1 {
                            nearest = Some((snap_point, dist));
                        }
                    }
                }
            }
        }

        // 4. Snap to Grid (if enabled)
        if config.snap_config.snap_to_grid {
            let grid_size = config.grid_config.grid_size;
            let grid_x = (pos.x / grid_size).round() * grid_size;
            let grid_y = (pos.y / grid_size).round() * grid_size;
            let grid_point = Vector2::new(grid_x, grid_y);

            let dist = pos.dist(grid_point);
            if dist <= tolerance {
                let snap_point = SnapPoint::new(grid_point, SnapPointType::Grid);
                if nearest.is_none() || dist < nearest.unwrap().1 {
                    nearest = Some((snap_point, dist));
                }
            }
        }

        nearest.map(|(sp, _)| sp)
    }

    /// Get all snap points from an entity
    fn get_entity_snap_points(&self, entity: &Entity) -> Vec<SnapPoint> {
        let mut points = Vec::new();

        match entity {
            Entity::Line(line) => {
                // Endpoints
                points.push(SnapPoint::new(line.start, SnapPointType::Endpoint));
                points.push(SnapPoint::new(line.end, SnapPointType::Endpoint));
                // Midpoint
                let mid = Vector2::new(
                    (line.start.x + line.end.x) / 2.0,
                    (line.start.y + line.end.y) / 2.0,
                );
                points.push(SnapPoint::new(mid, SnapPointType::Midpoint));
            }
            Entity::Circle(circle) => {
                // Center
                points.push(SnapPoint::new(circle.center, SnapPointType::Center));
                // Quadrant points (N, S, E, W)
                points.push(SnapPoint::new(
                    Vector2::new(circle.center.x + circle.radius, circle.center.y),
                    SnapPointType::Endpoint,
                ));
                points.push(SnapPoint::new(
                    Vector2::new(circle.center.x - circle.radius, circle.center.y),
                    SnapPointType::Endpoint,
                ));
                points.push(SnapPoint::new(
                    Vector2::new(circle.center.x, circle.center.y + circle.radius),
                    SnapPointType::Endpoint,
                ));
                points.push(SnapPoint::new(
                    Vector2::new(circle.center.x, circle.center.y - circle.radius),
                    SnapPointType::Endpoint,
                ));
            }
            Entity::Rectangle(rect) => {
                // Four corners
                points.push(SnapPoint::new(rect.min, SnapPointType::Corner));
                points.push(SnapPoint::new(rect.max, SnapPointType::Corner));
                points.push(SnapPoint::new(
                    Vector2::new(rect.min.x, rect.max.y),
                    SnapPointType::Corner,
                ));
                points.push(SnapPoint::new(
                    Vector2::new(rect.max.x, rect.min.y),
                    SnapPointType::Corner,
                ));
                // Center
                let center = Vector2::new(
                    (rect.min.x + rect.max.x) / 2.0,
                    (rect.min.y + rect.max.y) / 2.0,
                );
                points.push(SnapPoint::new(center, SnapPointType::Center));
            }
            Entity::Arc(arc) => {
                // Center
                points.push(SnapPoint::new(arc.center, SnapPointType::Center));
                // Start and end points
                points.push(SnapPoint::new(arc.start_point(), SnapPointType::Endpoint));
                points.push(SnapPoint::new(arc.end_point(), SnapPointType::Endpoint));
            }
            Entity::Text(text) => {
                // Text position
                points.push(SnapPoint::new(text.position, SnapPointType::Center));
                // Anchor points for measurements
                for pt in &text.anchor_points {
                    points.push(SnapPoint::new(*pt, SnapPointType::Endpoint));
                }
            }
        }

        points
    }

    /// Find intersection points between two entities
    fn find_intersections(&self, a: &Entity, b: &Entity) -> Vec<Vector2> {
        let mut intersections = Vec::new();

        match (a, b) {
            (Entity::Line(l1), Entity::Line(l2)) => {
                if let Some(pt) = self.line_line_intersection(l1.start, l1.end, l2.start, l2.end) {
                    intersections.push(pt);
                }
            }
            (Entity::Line(line), Entity::Circle(circle))
            | (Entity::Circle(circle), Entity::Line(line)) => {
                intersections.extend(self.line_circle_intersection(
                    line.start,
                    line.end,
                    circle.center,
                    circle.radius,
                ));
            }
            (Entity::Line(line), Entity::Rectangle(rect))
            | (Entity::Rectangle(rect), Entity::Line(line)) => {
                // Check intersection with all 4 edges
                let corners = [
                    (rect.min, Vector2::new(rect.max.x, rect.min.y)),
                    (Vector2::new(rect.max.x, rect.min.y), rect.max),
                    (rect.max, Vector2::new(rect.min.x, rect.max.y)),
                    (Vector2::new(rect.min.x, rect.max.y), rect.min),
                ];
                for (c1, c2) in corners {
                    if let Some(pt) = self.line_line_intersection(line.start, line.end, c1, c2) {
                        intersections.push(pt);
                    }
                }
            }
            (Entity::Circle(c1), Entity::Circle(c2)) => {
                intersections.extend(
                    self.circle_circle_intersection(c1.center, c1.radius, c2.center, c2.radius),
                );
            }
            _ => {
                // Other combinations can be added as needed
            }
        }

        intersections
    }

    /// Line-line intersection
    fn line_line_intersection(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
    ) -> Option<Vector2> {
        let d = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
        if d.abs() < 1e-10 {
            return None; // Parallel
        }

        let t = ((p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x)) / d;
        let u = -((p1.x - p2.x) * (p1.y - p3.y) - (p1.y - p2.y) * (p1.x - p3.x)) / d;

        // Check if intersection is within both line segments
        if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            Some(Vector2::new(
                p1.x + t * (p2.x - p1.x),
                p1.y + t * (p2.y - p1.y),
            ))
        } else {
            None
        }
    }

    /// Line-circle intersection
    fn line_circle_intersection(
        &self,
        p1: Vector2,
        p2: Vector2,
        center: Vector2,
        radius: f32,
    ) -> Vec<Vector2> {
        let mut result = Vec::new();

        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let fx = p1.x - center.x;
        let fy = p1.y - center.y;

        let a = dx * dx + dy * dy;
        let b = 2.0 * (fx * dx + fy * dy);
        let c = fx * fx + fy * fy - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let sqrt_disc = discriminant.sqrt();

            for t in [(-b - sqrt_disc) / (2.0 * a), (-b + sqrt_disc) / (2.0 * a)] {
                if t >= 0.0 && t <= 1.0 {
                    result.push(Vector2::new(p1.x + t * dx, p1.y + t * dy));
                }
            }
        }

        result
    }

    /// Circle-circle intersection
    fn circle_circle_intersection(
        &self,
        c1: Vector2,
        r1: f32,
        c2: Vector2,
        r2: f32,
    ) -> Vec<Vector2> {
        let mut result = Vec::new();

        let d = c1.dist(c2);

        // No intersection cases
        if d > r1 + r2 || d < (r1 - r2).abs() || d == 0.0 {
            return result;
        }

        let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
        let h_sq = r1 * r1 - a * a;

        if h_sq < 0.0 {
            return result;
        }

        let h = h_sq.sqrt();

        let px = c1.x + a * (c2.x - c1.x) / d;
        let py = c1.y + a * (c2.y - c1.y) / d;

        result.push(Vector2::new(
            px + h * (c2.y - c1.y) / d,
            py - h * (c2.x - c1.x) / d,
        ));

        if h > 1e-10 {
            result.push(Vector2::new(
                px - h * (c2.y - c1.y) / d,
                py + h * (c2.x - c1.x) / d,
            ));
        }

        result
    }
}
