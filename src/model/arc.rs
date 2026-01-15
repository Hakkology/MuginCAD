use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Arc entity - a portion of a circle defined by center, radius, start and end angles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc {
    pub center: Vector2,
    pub radius: f32,
    pub start_angle: f32, // radians
    pub end_angle: f32,   // radians
    pub filled: bool,
}

impl Arc {
    pub fn new(
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        filled: bool,
    ) -> Self {
        Self {
            center,
            radius,
            start_angle,
            end_angle,
            filled,
        }
    }

    /// Create arc from 3 points: center, start point, end point
    pub fn from_three_points(center: Vector2, start: Vector2, end: Vector2, filled: bool) -> Self {
        Self::from_three_points_directed(center, start, end, filled, false)
    }

    /// Create arc from 3 points with direction control
    /// If clockwise is true, arc goes CW from start to end, otherwise CCW
    pub fn from_three_points_directed(
        center: Vector2,
        start: Vector2,
        end: Vector2,
        filled: bool,
        clockwise: bool,
    ) -> Self {
        let radius = ((start.x - center.x).powi(2) + (start.y - center.y).powi(2)).sqrt();
        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let end_angle = (end.y - center.y).atan2(end.x - center.x);

        if clockwise {
            // For CW, swap start and end angles
            Self {
                center,
                radius,
                start_angle: end_angle,
                end_angle: start_angle,
                filled,
            }
        } else {
            Self {
                center,
                radius,
                start_angle,
                end_angle,
                filled,
            }
        }
    }

    /// Get start point on the arc
    pub fn start_point(&self) -> Vector2 {
        Vector2::new(
            self.center.x + self.radius * self.start_angle.cos(),
            self.center.y + self.radius * self.start_angle.sin(),
        )
    }

    /// Get end point on the arc
    pub fn end_point(&self) -> Vector2 {
        Vector2::new(
            self.center.x + self.radius * self.end_angle.cos(),
            self.center.y + self.radius * self.end_angle.sin(),
        )
    }

    /// Hit test for arc
    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        let dist = (dx * dx + dy * dy).sqrt();

        // Check if near the arc radius
        if (dist - self.radius).abs() > tolerance {
            return false;
        }

        // Check if the angle is within the arc
        let angle = dy.atan2(dx);
        self.angle_in_range(angle)
    }

    /// Check if angle is within the arc range
    fn angle_in_range(&self, angle: f32) -> bool {
        let mut start = self.start_angle;
        let mut end = self.end_angle;
        let mut test = angle;

        // Normalize to [0, 2π]
        let two_pi = std::f32::consts::PI * 2.0;
        while start < 0.0 {
            start += two_pi;
        }
        while end < 0.0 {
            end += two_pi;
        }
        while test < 0.0 {
            test += two_pi;
        }
        start %= two_pi;
        end %= two_pi;
        test %= two_pi;

        if start <= end {
            test >= start && test <= end
        } else {
            // Arc crosses 0
            test >= start || test <= end
        }
    }
}
