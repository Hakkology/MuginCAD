use crate::model::Entity;
use crate::model::Vector2;
use crate::view::context::DrawContext;
use eframe::egui;

pub fn render_entities(
    ctx: &DrawContext,
    entities: &[Entity],
    selected_indices: &std::collections::HashSet<usize>,
    hovered_entity_idx: Option<usize>,
) {
    for (i, entity) in entities.iter().enumerate() {
        let is_selected = selected_indices.contains(&i);
        let is_hovered = Some(i) == hovered_entity_idx;

        let color = if is_selected {
            egui::Color32::GOLD
        } else if is_hovered {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_rgb(0, 255, 255)
        };

        let stroke_width = if is_selected { 2.5 } else { 1.5 };

        match entity {
            Entity::Line(line) => {
                ctx.painter.line_segment(
                    [ctx.to_screen(line.start), ctx.to_screen(line.end)],
                    egui::Stroke::new(stroke_width, color),
                );

                // Draw length label if enabled
                if line.show_length {
                    // Logic moved from canvas.rs
                    // We need to access viewport_zoom from ctx.zoom
                    let viewport_zoom = ctx.zoom;

                    // Logic copied and adapted from canvas.rs
                    let tolerance = 5.0 / viewport_zoom;
                    let smart_offset = line.calculate_smart_offset(tolerance);

                    let world_label_pos = line.midpoint() + smart_offset + line.label_offset;

                    // Dimension Lines
                    let dx = line.end.x - line.start.x;
                    let dy = line.end.y - line.start.y;
                    let len = (dx * dx + dy * dy).sqrt();

                    if len > 0.001 {
                        let smart_len = (smart_offset.x.powi(2) + smart_offset.y.powi(2)).sqrt();
                        let (perp_offset_x, perp_offset_y) = if smart_len > 0.001 {
                            let nx = smart_offset.x / smart_len;
                            let ny = smart_offset.y / smart_len;
                            let user_proj = line.label_offset.x * nx + line.label_offset.y * ny;
                            let total_perp = smart_len + user_proj;
                            (nx * total_perp, ny * total_perp)
                        } else {
                            (smart_offset.x, smart_offset.y)
                        };
                        let perp_vec = Vector2::new(perp_offset_x, perp_offset_y);

                        let ext_start_pos = line.start + perp_vec;
                        let ext_end_pos = line.end + perp_vec;

                        let s_line_start = ctx.to_screen(line.start);
                        let s_line_end = ctx.to_screen(line.end);
                        let s_ext_start = ctx.to_screen(ext_start_pos);
                        let s_ext_end = ctx.to_screen(ext_end_pos);

                        let dim_color = egui::Color32::from_rgb(150, 150, 150);
                        let dim_stroke = egui::Stroke::new(1.0, dim_color);

                        ctx.painter
                            .line_segment([s_line_start, s_ext_start], dim_stroke);
                        ctx.painter
                            .line_segment([s_line_end, s_ext_end], dim_stroke);

                        // Dimension Line
                        let tx = dx / len;
                        let ty = dy / len;
                        let label_rel_x = world_label_pos.x - ext_start_pos.x;
                        let label_rel_y = world_label_pos.y - ext_start_pos.y;
                        let label_proj_t = label_rel_x * tx + label_rel_y * ty;

                        let t_min = 0.0f32.min(label_proj_t);
                        let t_max = len.max(label_proj_t);

                        let dim_start_world = Vector2::new(
                            ext_start_pos.x + tx * t_min,
                            ext_start_pos.y + ty * t_min,
                        );
                        let dim_end_world = Vector2::new(
                            ext_start_pos.x + tx * t_max,
                            ext_start_pos.y + ty * t_max,
                        );

                        ctx.painter.line_segment(
                            [ctx.to_screen(dim_start_world), ctx.to_screen(dim_end_world)],
                            dim_stroke,
                        );
                    }

                    // Draw Text
                    let label_pos = ctx.to_screen(world_label_pos);

                    let screen_start = ctx.to_screen(line.start);
                    let screen_end = ctx.to_screen(line.end);
                    let screen_dx = screen_end.x - screen_start.x;
                    let screen_dy = screen_end.y - screen_start.y;
                    let screen_angle = screen_dy.atan2(screen_dx);

                    let adjusted_angle = if screen_angle.abs() > std::f32::consts::FRAC_PI_2 {
                        screen_angle + std::f32::consts::PI
                    } else {
                        screen_angle
                    };

                    let length_text = format!("{:.2}", line.length());
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
            Entity::Circle(circle) => {
                let screen_radius = circle.radius * ctx.zoom;
                if circle.filled {
                    ctx.painter.circle_filled(
                        ctx.to_screen(circle.center),
                        screen_radius,
                        color.linear_multiply(0.3),
                    );
                }
                ctx.painter.circle_stroke(
                    ctx.to_screen(circle.center),
                    screen_radius,
                    egui::Stroke::new(stroke_width, color),
                );
            }
            Entity::Rectangle(rect) => {
                let rect_screen = egui::Rect::from_min_max(
                    ctx.to_screen(Vector2::new(rect.min.x, rect.max.y)),
                    ctx.to_screen(Vector2::new(rect.max.x, rect.min.y)),
                );
                if rect.filled {
                    ctx.painter
                        .rect_filled(rect_screen, 0.0, color.linear_multiply(0.3));
                }
                ctx.painter
                    .rect_stroke(rect_screen, 0.0, egui::Stroke::new(stroke_width, color));
            }
            Entity::Arc(arc) => {
                let segments = 32;
                let mut angle_range = arc.end_angle - arc.start_angle;
                if angle_range < 0.0 {
                    angle_range += std::f32::consts::PI * 2.0;
                }
                let angle_step = angle_range / segments as f32;

                let mut points = Vec::with_capacity(segments + 1);
                for i in 0..=segments {
                    let angle = arc.start_angle + angle_step * i as f32;
                    let pt = Vector2::new(
                        arc.center.x + arc.radius * angle.cos(),
                        arc.center.y + arc.radius * angle.sin(),
                    );
                    points.push(ctx.to_screen(pt));
                }

                if arc.filled {
                    let mut fill_points = vec![ctx.to_screen(arc.center)];
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
            Entity::Text(text) => {
                let text_pos = ctx.to_screen(text.position);
                let text_color = egui::Color32::from_rgb(
                    text.style.color[0],
                    text.style.color[1],
                    text.style.color[2],
                );
                let final_color = if is_selected {
                    egui::Color32::GOLD
                } else {
                    text_color
                };

                if text.anchor_points.len() >= 2 {
                    let start = ctx.to_screen(text.anchor_points[0]);
                    let end = ctx.to_screen(text.anchor_points[1]);
                    ctx.painter.line_segment(
                        [start, end],
                        egui::Stroke::new(1.0, final_color.linear_multiply(0.5)),
                    );
                    ctx.painter.circle_filled(start, 3.0, final_color);
                    ctx.painter.circle_filled(end, 3.0, final_color);
                }

                let font_size = text.style.font_size * ctx.zoom;
                let font_id = egui::FontId::proportional(font_size.max(8.0).min(48.0));

                let galley = ctx
                    .painter
                    .layout_no_wrap(text.text.clone(), font_id, final_color);
                let text_size = galley.size();

                let angle = -text.rotation;
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
    }
}
