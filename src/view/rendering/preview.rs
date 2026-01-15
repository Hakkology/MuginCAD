use crate::commands::Command;
use crate::model::Vector2;
use crate::view::rendering::context::DrawContext;
use eframe::egui;

pub fn draw_preview(
    ctx: &DrawContext,
    cmd: &dyn Command,
    points: &[Vector2],
    current_cad: Vector2,
) {
    let preview_stroke = egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
    );

    match cmd.name() {
        "LINE" => {
            if let Some(&last_point) = points.last() {
                ctx.painter.line_segment(
                    [ctx.to_screen(last_point), ctx.to_screen(current_cad)],
                    preview_stroke,
                );
            }
        }
        "CIRCLE" => {
            if let Some(&center) = points.first() {
                let cad_radius = center.dist(current_cad);
                let screen_radius = cad_radius * ctx.zoom;

                // Draw circle
                ctx.painter
                    .circle_stroke(ctx.to_screen(center), screen_radius, preview_stroke);

                // Draw radius line
                ctx.painter.line_segment(
                    [ctx.to_screen(center), ctx.to_screen(current_cad)],
                    preview_stroke,
                );

                // Draw center marker
                ctx.painter.circle_stroke(
                    ctx.to_screen(center),
                    4.0,
                    egui::Stroke::new(1.5, egui::Color32::YELLOW),
                );
            }
        }
        "RECTANGLE" => {
            if let Some(&start) = points.first() {
                let min = Vector2::new(start.x.min(current_cad.x), start.y.min(current_cad.y));
                let max = Vector2::new(start.x.max(current_cad.x), start.y.max(current_cad.y));
                let rect_screen = egui::Rect::from_min_max(
                    ctx.to_screen(Vector2::new(min.x, max.y)),
                    ctx.to_screen(Vector2::new(max.x, min.y)),
                );
                ctx.painter.rect_stroke(rect_screen, 0.0, preview_stroke);

                let width = (max.x - min.x).abs();
                let height = (max.y - min.y).abs();
                let dim_color = egui::Color32::from_rgb(255, 200, 100);
                let dim_font = egui::FontId::proportional(11.0);

                let bottom_mid = ctx.to_screen(Vector2::new((min.x + max.x) / 2.0, min.y));
                ctx.painter.text(
                    egui::pos2(bottom_mid.x, bottom_mid.y + 14.0),
                    egui::Align2::CENTER_CENTER,
                    format!("W: {:.2}", width),
                    dim_font.clone(),
                    dim_color,
                );

                let right_mid = ctx.to_screen(Vector2::new(max.x, (min.y + max.y) / 2.0));
                ctx.painter.text(
                    egui::pos2(right_mid.x + 30.0, right_mid.y),
                    egui::Align2::CENTER_CENTER,
                    format!("H: {:.2}", height),
                    dim_font.clone(),
                    dim_color,
                );
            }
        }
        "Arc" => {
            match points.len() {
                1 => {
                    let center = points[0];
                    ctx.painter.line_segment(
                        [ctx.to_screen(center), ctx.to_screen(current_cad)],
                        preview_stroke,
                    );
                    ctx.painter.circle_stroke(
                        ctx.to_screen(center),
                        4.0,
                        egui::Stroke::new(1.5, egui::Color32::YELLOW),
                    );
                    let radius = center.dist(current_cad) * ctx.zoom;
                    ctx.painter.circle_stroke(
                        ctx.to_screen(center),
                        radius,
                        egui::Stroke::new(
                            0.5,
                            egui::Color32::from_rgba_unmultiplied(150, 150, 150, 80),
                        ),
                    );
                }
                2 => {
                    let center = points[0];
                    let start = points[1];
                    // let radius = center.dist(start);
                    let start_angle = (start.y - center.y).atan2(start.x - center.x);
                    let end_angle = (current_cad.y - center.y).atan2(current_cad.x - center.x);

                    let is_clockwise = if let Some(arc_cmd) = cmd
                        .as_any()
                        .and_then(|a| a.downcast_ref::<crate::commands::arc::ArcCommand>())
                    {
                        arc_cmd.clockwise
                    } else {
                        false
                    };

                    let segments = 32;
                    let angle_range = if is_clockwise {
                        let mut range = start_angle - end_angle;
                        if range < 0.0 {
                            range += std::f32::consts::PI * 2.0;
                        }
                        -range
                    } else {
                        let mut range = end_angle - start_angle;
                        if range < 0.0 {
                            range += std::f32::consts::PI * 2.0;
                        }
                        range
                    };

                    let angle_step = angle_range / segments as f32;
                    let radius = center.dist(start); // Should match start point distance

                    let mut arc_points = Vec::with_capacity(segments + 1);
                    for i in 0..=segments {
                        let angle = start_angle + angle_step * i as f32;
                        let pt = Vector2::new(
                            center.x + radius * angle.cos(),
                            center.y + radius * angle.sin(),
                        );
                        arc_points.push(ctx.to_screen(pt));
                    }

                    for i in 0..arc_points.len() - 1 {
                        ctx.painter
                            .line_segment([arc_points[i], arc_points[i + 1]], preview_stroke);
                    }

                    // Center and start markers
                    ctx.painter
                        .circle_filled(ctx.to_screen(center), 3.0, egui::Color32::YELLOW);
                    ctx.painter
                        .circle_filled(ctx.to_screen(start), 3.0, egui::Color32::GREEN);
                    ctx.painter.line_segment(
                        [ctx.to_screen(center), ctx.to_screen(current_cad)],
                        preview_stroke,
                    );
                }
                _ => {}
            }
        }
        "MOVE" => {
            if let Some(&base) = points.first() {
                ctx.painter.line_segment(
                    [ctx.to_screen(base), ctx.to_screen(current_cad)],
                    preview_stroke,
                );
                ctx.painter
                    .circle_stroke(ctx.to_screen(base), 4.0, preview_stroke);
                ctx.painter
                    .circle_filled(ctx.to_screen(current_cad), 3.0, egui::Color32::WHITE);
            }
        }
        "ROTATE" => {
            if let Some(&pivot) = points.first() {
                let pivot_screen = ctx.to_screen(pivot);

                // Calculate angle and radius
                let dx = current_cad.x - pivot.x;
                let dy = current_cad.y - pivot.y;
                let angle = dy.atan2(dx);
                // Correctly use zoom from context
                let radius = (dx * dx + dy * dy).sqrt() * ctx.zoom;

                // Draw arc from 0 to current angle
                let arc_radius = radius.min(80.0); // Cap arc size for visibility
                let num_segments = 32;
                let start_angle = 0.0_f32;
                let end_angle = angle;

                // Generate arc points
                let mut arc_points = Vec::new();
                for i in 0..=num_segments {
                    let t = i as f32 / num_segments as f32;
                    let a = start_angle + (end_angle - start_angle) * t;
                    let px = pivot_screen.x + arc_radius * a.cos();
                    let py = pivot_screen.y - arc_radius * a.sin(); // Egui Y is inverted relative to standard cartesian math if we just add sin? 
                    // Wait, standard math: y = r*sin(a).
                    // text rotation logic used: pos.y - rot_y.
                    // canvas.rs legacy: py = pivot_screen.y - arc_radius * a.sin();
                    // So we preserve that.
                    arc_points.push(egui::pos2(px, py));
                }

                // Draw arc as line segments
                for i in 0..arc_points.len().saturating_sub(1) {
                    ctx.painter.line_segment(
                        [arc_points[i], arc_points[i + 1]],
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 100)),
                    );
                }

                // Draw radius line from pivot to mouse
                ctx.painter
                    .line_segment([pivot_screen, ctx.to_screen(current_cad)], preview_stroke);

                // Draw reference line (horizontal from pivot)
                let ref_end = egui::pos2(pivot_screen.x + arc_radius, pivot_screen.y);
                ctx.painter.line_segment(
                    [pivot_screen, ref_end],
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(150, 150, 150, 100),
                    ),
                );

                // Draw pivot marker
                ctx.painter.circle_stroke(
                    pivot_screen,
                    5.0,
                    egui::Stroke::new(2.0, egui::Color32::YELLOW),
                );
            }
        }
        "SCALE" => {
            if let Some(&base) = points.first() {
                ctx.painter.line_segment(
                    [ctx.to_screen(base), ctx.to_screen(current_cad)],
                    preview_stroke,
                );
                // Draw base marker (filled square)
                let base_screen = ctx.to_screen(base);
                let size = 4.0;
                ctx.painter.rect_filled(
                    egui::Rect::from_center_size(base_screen, egui::vec2(size * 2.0, size * 2.0)),
                    0.0,
                    egui::Color32::WHITE,
                );
            }
        }
        "COPY" | "CUT" => {
            if let Some(&base) = points.first() {
                ctx.painter.line_segment(
                    [ctx.to_screen(base), ctx.to_screen(current_cad)],
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(100, 200, 255)),
                );
                ctx.painter.circle_stroke(
                    ctx.to_screen(base),
                    4.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255)),
                );
                ctx.painter.circle_filled(
                    ctx.to_screen(current_cad),
                    3.0,
                    egui::Color32::from_rgb(100, 200, 255),
                );
            }
        }
        _ => {}
    }
}
