use crate::model::Vector2;
use crate::model::shapes::{
    annotation::TextAnnotation, arc::Arc, circle::Circle, line::Line, rectangle::Rectangle,
};
use crate::model::{Entity, Shape};
use crate::view::rendering::context::DrawContext;
use eframe::egui;

/// Trait for entities that can be rendered on the canvas
pub trait Renderable {
    fn render(&self, ctx: &DrawContext, is_selected: bool, is_hovered: bool);
}

fn get_base_style(is_selected: bool, is_hovered: bool) -> (egui::Color32, f32) {
    if is_selected {
        (egui::Color32::GOLD, 2.5)
    } else if is_hovered {
        (egui::Color32::WHITE, 1.5)
    } else {
        (egui::Color32::from_rgb(0, 255, 255), 1.5)
    }
}

impl Renderable for Line {
    fn render(&self, ctx: &DrawContext, is_selected: bool, is_hovered: bool) {
        let (color, stroke_width) = get_base_style(is_selected, is_hovered);

        ctx.painter.line_segment(
            [ctx.to_screen(self.start), ctx.to_screen(self.end)],
            egui::Stroke::new(stroke_width, color),
        );

        if self.show_length {
            let viewport_zoom = ctx.zoom;
            let tolerance = 5.0 / viewport_zoom;
            let smart_offset = self.calculate_smart_offset(tolerance);
            let world_label_pos = self.midpoint() + smart_offset + self.label_offset;

            // Dimension Lines logic
            let dx = self.end.x - self.start.x;
            let dy = self.end.y - self.start.y;
            let len = (dx * dx + dy * dy).sqrt();

            if len > 0.001 {
                let smart_len = (smart_offset.x.powi(2) + smart_offset.y.powi(2)).sqrt();
                let (perp_offset_x, perp_offset_y) = if smart_len > 0.001 {
                    let nx = smart_offset.x / smart_len;
                    let ny = smart_offset.y / smart_len;
                    let user_proj = self.label_offset.x * nx + self.label_offset.y * ny;
                    let total_perp = smart_len + user_proj;
                    (nx * total_perp, ny * total_perp)
                } else {
                    (smart_offset.x, smart_offset.y)
                };
                let perp_vec = Vector2::new(perp_offset_x, perp_offset_y);

                let ext_start_pos = self.start + perp_vec;
                let ext_end_pos = self.end + perp_vec;

                let s_line_start = ctx.to_screen(self.start);
                let s_line_end = ctx.to_screen(self.end);
                let s_ext_start = ctx.to_screen(ext_start_pos);
                let s_ext_end = ctx.to_screen(ext_end_pos);

                let dim_color = egui::Color32::from_rgb(150, 150, 150);
                let dim_stroke = egui::Stroke::new(1.0, dim_color);

                ctx.painter
                    .line_segment([s_line_start, s_ext_start], dim_stroke);
                ctx.painter
                    .line_segment([s_line_end, s_ext_end], dim_stroke);

                let tx = dx / len;
                let ty = dy / len;
                let label_rel_x = world_label_pos.x - ext_start_pos.x;
                let label_rel_y = world_label_pos.y - ext_start_pos.y;
                let label_proj_t = label_rel_x * tx + label_rel_y * ty;

                let t_min = 0.0f32.min(label_proj_t);
                let t_max = len.max(label_proj_t);

                let dim_start_world =
                    Vector2::new(ext_start_pos.x + tx * t_min, ext_start_pos.y + ty * t_min);
                let dim_end_world =
                    Vector2::new(ext_start_pos.x + tx * t_max, ext_start_pos.y + ty * t_max);

                ctx.painter.line_segment(
                    [ctx.to_screen(dim_start_world), ctx.to_screen(dim_end_world)],
                    dim_stroke,
                );
            }

            // Text
            let label_pos = ctx.to_screen(world_label_pos);
            let screen_start = ctx.to_screen(self.start);
            let screen_end = ctx.to_screen(self.end);
            let screen_dx = screen_end.x - screen_start.x;
            let screen_dy = screen_end.y - screen_start.y;
            let screen_angle = screen_dy.atan2(screen_dx);

            let adjusted_angle = if screen_angle.abs() > std::f32::consts::FRAC_PI_2 {
                screen_angle + std::f32::consts::PI
            } else {
                screen_angle
            };

            let length_text = format!("{:.2}", self.length());
            let label_color = if is_selected {
                egui::Color32::GOLD
            } else {
                egui::Color32::from_rgb(255, 200, 100)
            };

            let font_id = egui::FontId::proportional(12.0);
            let galley = ctx
                .painter
                .layout_no_wrap(length_text, font_id, label_color);
            let text_size = galley.size();

            let half_w = text_size.x / 2.0;
            let half_h = text_size.y / 2.0;
            let cos_a = adjusted_angle.cos();
            let sin_a = adjusted_angle.sin();
            let rot_x = half_w * cos_a - half_h * sin_a;
            let rot_y = half_w * sin_a + half_h * cos_a;

            let final_pos = egui::pos2(label_pos.x - rot_x, label_pos.y - rot_y);

            ctx.painter.add(egui::epaint::TextShape {
                pos: final_pos,
                galley,
                underline: egui::Stroke::NONE,
                fallback_color: label_color,
                override_text_color: Some(label_color),
                opacity_factor: 1.0,
                angle: adjusted_angle,
            });
        }
    }
}

impl Renderable for Circle {
    fn render(&self, ctx: &DrawContext, is_selected: bool, is_hovered: bool) {
        let (color, stroke_width) = get_base_style(is_selected, is_hovered);

        let screen_radius = self.radius * ctx.zoom;
        if self.filled {
            ctx.painter.circle_filled(
                ctx.to_screen(self.center),
                screen_radius,
                color.linear_multiply(0.3),
            );
        }
        ctx.painter.circle_stroke(
            ctx.to_screen(self.center),
            screen_radius,
            egui::Stroke::new(stroke_width, color),
        );
    }
}

impl Renderable for Rectangle {
    fn render(&self, ctx: &DrawContext, is_selected: bool, is_hovered: bool) {
        let (color, stroke_width) = get_base_style(is_selected, is_hovered);

        let rect_screen = egui::Rect::from_min_max(
            ctx.to_screen(Vector2::new(self.min.x, self.max.y)),
            ctx.to_screen(Vector2::new(self.max.x, self.min.y)),
        );
        if self.filled {
            ctx.painter
                .rect_filled(rect_screen, 0.0, color.linear_multiply(0.3));
        }
        ctx.painter
            .rect_stroke(rect_screen, 0.0, egui::Stroke::new(stroke_width, color));
    }
}

impl Renderable for Arc {
    fn render(&self, ctx: &DrawContext, is_selected: bool, is_hovered: bool) {
        let (color, stroke_width) = get_base_style(is_selected, is_hovered);

        let segments = 32;
        let mut angle_range = self.end_angle - self.start_angle;
        if angle_range < 0.0 {
            angle_range += std::f32::consts::PI * 2.0;
        }
        let angle_step = angle_range / segments as f32;

        let mut points = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let angle = self.start_angle + angle_step * i as f32;
            let pt = Vector2::new(
                self.center.x + self.radius * angle.cos(),
                self.center.y + self.radius * angle.sin(),
            );
            points.push(ctx.to_screen(pt));
        }

        if self.filled {
            let mut fill_points = vec![ctx.to_screen(self.center)];
            fill_points.extend(points.iter().cloned());
            ctx.painter.add(egui::Shape::convex_polygon(
                fill_points,
                color.linear_multiply(0.3),
                egui::Stroke::NONE,
            ));
        }

        for i in 0..points.len() - 1 {
            ctx.painter.line_segment(
                [points[i], points[i + 1]],
                egui::Stroke::new(stroke_width, color),
            );
        }
    }
}

impl Renderable for TextAnnotation {
    fn render(&self, ctx: &DrawContext, is_selected: bool, _is_hovered: bool) {
        let text_color = egui::Color32::from_rgb(
            self.style.color[0],
            self.style.color[1],
            self.style.color[2],
        );
        let final_color = if is_selected {
            egui::Color32::GOLD
        } else {
            text_color
        };

        // Calculate text size first
        let font_size = self.style.font_size * ctx.zoom;
        let font_id = egui::FontId::proportional(font_size.max(8.0).min(48.0));
        let galley = ctx
            .painter
            .layout_no_wrap(self.text.clone(), font_id, final_color);
        let text_size = galley.size();

        if self.anchor_points.len() >= 2
            && matches!(
                self.annotation_type,
                crate::model::shapes::annotation::AnnotationType::Distance
            )
        {
            let p1 = self.anchor_points[0];
            let p2 = self.anchor_points[1];
            let label_pos = self.position;

            let d = p2 - p1;
            let len = d.length();
            if len > 0.001 {
                let u = d / len;
                let v = label_pos - p1;

                // Project v onto u to find parallel component t
                let t = v.dot(u);
                // Perpendicular vector from line to label
                let mut perp = v - u * t;

                // Add margin between text and line (line should be "below" text)
                if perp.length_squared() > 0.0001 {
                    let dir = perp.normalized();
                    let margin = 3.0 / ctx.zoom;
                    let half_h_world = (text_size.y / 2.0) / ctx.zoom;
                    // Move line away from text towards the object properties
                    perp = perp - dir * (half_h_world + margin);
                }

                // Points on the dimension line (original anchors projection)
                let dim_p1_proj = p1 + perp;
                let dim_p2_proj = p2 + perp;

                let dim_color = final_color.linear_multiply(0.6);
                let dim_stroke = egui::Stroke::new(1.0, dim_color);

                // Extension lines
                let perp_len_sq = perp.length_squared();
                if perp_len_sq > 0.0001 {
                    let perp_dir = perp.normalized();
                    let zoom_factor = 1.0 / ctx.zoom;
                    let overshoot = perp_dir * (5.0 * zoom_factor);
                    let gap = perp_dir * (2.0 * zoom_factor);

                    let ext1_start = p1 + gap;
                    let ext1_end = dim_p1_proj + overshoot;

                    let ext2_start = p2 + gap;
                    let ext2_end = dim_p2_proj + overshoot;

                    ctx.painter.line_segment(
                        [ctx.to_screen(ext1_start), ctx.to_screen(ext1_end)],
                        dim_stroke,
                    );
                    ctx.painter.line_segment(
                        [ctx.to_screen(ext2_start), ctx.to_screen(ext2_end)],
                        dim_stroke,
                    );
                }

                // Draw main dimension line with extension to cover text
                // text_size is in screen pixels, convert to world units
                let half_w_world = (text_size.x / 2.0) / ctx.zoom * 1.1; // 10% padding

                let line_start_t = 0.0f32.min(t - half_w_world);
                let line_end_t = len.max(t + half_w_world);

                let line_p1 = dim_p1_proj + u * line_start_t;
                let line_p2 = dim_p1_proj + u * line_end_t;

                ctx.painter
                    .line_segment([ctx.to_screen(line_p1), ctx.to_screen(line_p2)], dim_stroke);

                // Draw ticks at original intersections
                ctx.painter
                    .circle_filled(ctx.to_screen(dim_p1_proj), 2.0, dim_color);
                ctx.painter
                    .circle_filled(ctx.to_screen(dim_p2_proj), 2.0, dim_color);
            }
        } else if self.anchor_points.len() >= 2 {
            let start = ctx.to_screen(self.anchor_points[0]);
            let end = ctx.to_screen(self.anchor_points[1]);
            ctx.painter.line_segment(
                [start, end],
                egui::Stroke::new(1.0, final_color.linear_multiply(0.5)),
            );
            ctx.painter.circle_filled(start, 3.0, final_color);
            ctx.painter.circle_filled(end, 3.0, final_color);
        }

        let text_pos = ctx.to_screen(self.position);
        let angle = -self.rotation;
        let half_w = text_size.x / 2.0;
        let half_h = text_size.y / 2.0;
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let rot_x = half_w * cos_a - half_h * sin_a;
        let rot_y = half_w * sin_a + half_h * cos_a;

        let final_pos = egui::pos2(text_pos.x - rot_x, text_pos.y - rot_y);

        ctx.painter.add(egui::epaint::TextShape {
            pos: final_pos,
            galley,
            underline: egui::Stroke::NONE,
            fallback_color: final_color,
            override_text_color: Some(final_color),
            opacity_factor: 1.0,
            angle,
        });
    }
}

impl Renderable for Entity {
    fn render(&self, ctx: &DrawContext, is_selected: bool, is_hovered: bool) {
        match &self.shape {
            Shape::Line(e) => e.render(ctx, is_selected, is_hovered),
            Shape::Circle(e) => e.render(ctx, is_selected, is_hovered),
            Shape::Rectangle(e) => e.render(ctx, is_selected, is_hovered),
            Shape::Arc(e) => e.render(ctx, is_selected, is_hovered),
            Shape::Text(e) => e.render(ctx, is_selected, is_hovered),
            Shape::None => {}
        }
        // Render children recursively
        for child in &self.children {
            child.render(ctx, is_selected, is_hovered);
        }
    }
}
