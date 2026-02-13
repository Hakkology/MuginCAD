use crate::model::structure::beam::BeamAnchor;
use crate::model::structure::beam_type::BeamType;
use crate::model::structure::column_type::ColumnType;
use eframe::egui;

/// Draws a structural column using the given painter and transform.
///
/// * `painter`: The egui painter to draw with.
/// * `center`: The screen-space center position of the column.
/// * `rotation`: The rotation angle in radians.
/// * `scale`: The scaling factor (pixels per model unit).
/// * `col`: The column type definition containing dimensions and reinforcement details.
/// * `alpha_mul`: Transparency multiplier (0.0 - 1.0), useful for ghosts/previews.
pub fn draw_column(
    painter: &egui::Painter,
    center: egui::Pos2,
    rotation: f32,
    scale: f32,
    col: &ColumnType,
    alpha_mul: f32,
) {
    let width_px = col.width * scale;
    let depth_px = col.depth * scale; // Fixed: height -> depth

    // Helper to rotate a point around (0,0) and then translate to center
    let transform = |x: f32, y: f32| -> egui::Pos2 {
        let (sin, cos) = rotation.sin_cos();
        // Standard rotation matrix
        let rx = x * cos - y * sin;
        let ry = x * sin + y * cos;
        egui::pos2(center.x + rx, center.y + ry)
    };

    // 1. Draw Concrete Body
    // ---------------------
    // Get corners: TL, TR, BR, BL
    let hw = width_px / 2.0;
    let hh = depth_px / 2.0;

    // Note: In screen space Y is down, but typically for shape drawing relative to center
    // it handles conveniently if we just think in local coords.
    let corners = [
        transform(-hw, -hh),
        transform(hw, -hh),
        transform(hw, hh),
        transform(-hw, hh),
    ];

    // Use hardcoded concrete color for visual consistency for now
    let concrete_color = egui::Color32::from_rgb(150, 150, 160);

    let fill_color = concrete_color.linear_multiply(0.3 * alpha_mul);
    let stroke_color = concrete_color.linear_multiply(alpha_mul);

    painter.add(egui::Shape::convex_polygon(
        corners.to_vec(),
        fill_color,
        egui::Stroke::new(2.0, stroke_color),
    ));

    // 2. Draw Rebars (Longitudinal)
    // -----------------------------
    // Calculate cover in pixels
    let cover_px = 3.0 * scale; // Assuming 3cm cover? Adjust based on data if available
    // Currently ColumnType doesn't strictly define cover, we can estimate or hardcode for visual
    let bar_radius_px = (col.long_bar_diameter / 2.0 * 0.1) * scale; // mm -> cm -> px
    let bar_radius_px = bar_radius_px.max(1.5); // Min size for visibility

    let inner_w = width_px - 2.0 * cover_px;
    let inner_h = depth_px - 2.0 * cover_px;

    let nx = col.long_bars_x.max(2);
    let ny = col.long_bars_y.max(2);

    let dx = if nx > 1 {
        inner_w / (nx - 1) as f32
    } else {
        0.0
    };
    let dy = if ny > 1 {
        inner_h / (ny - 1) as f32
    } else {
        0.0
    };

    // Use dark gray for steel
    let bar_color = egui::Color32::from_rgb(50, 50, 50);
    let bar_fill = bar_color.linear_multiply(alpha_mul);

    // Draw top/bottom rows (along X)
    // Top row (y = -inner_h/2)
    for i in 0..nx {
        let x = -inner_w / 2.0 + i as f32 * dx;
        let y = -inner_h / 2.0;
        painter.circle_filled(transform(x, y), bar_radius_px, bar_fill);
    }
    // Bottom row (y = inner_h/2)
    for i in 0..nx {
        let x = -inner_w / 2.0 + i as f32 * dx;
        let y = inner_h / 2.0;
        painter.circle_filled(transform(x, y), bar_radius_px, bar_fill);
    }

    // Draw side rows (along Y), skipping corners (already drawn)
    if ny > 2 {
        for j in 1..(ny - 1) {
            let y = -inner_h / 2.0 + j as f32 * dy;
            // Left side
            let x_left = -inner_w / 2.0;
            painter.circle_filled(transform(x_left, y), bar_radius_px, bar_fill);
            // Right side
            let x_right = inner_w / 2.0;
            painter.circle_filled(transform(x_right, y), bar_radius_px, bar_fill);
        }
    }

    // 3. Draw Ties (Stirrups)
    // -----------------------
    if col.has_ties {
        let tie_padding = cover_px - (0.8 * scale); // Just slightly outside bars
        let tie_w = width_px - 2.0 * tie_padding;
        let tie_h = depth_px - 2.0 * tie_padding;
        let thw = tie_w / 2.0;
        let thh = tie_h / 2.0;

        let tie_corners = [
            transform(-thw, -thh),
            transform(thw, -thh),
            transform(thw, thh),
            transform(-thw, thh),
            transform(-thw, -thh), // Close loop
        ];

        let tie_color = egui::Color32::from_rgb(100, 100, 200).linear_multiply(alpha_mul);
        painter.add(egui::Shape::line(
            tie_corners.to_vec(),
            egui::Stroke::new(1.0, tie_color),
        ));
    }
}

pub fn draw_beam(
    painter: &egui::Painter,
    start: egui::Pos2,
    end: egui::Pos2,
    scale: f32,
    beam_type: &BeamType,
    alpha_mul: f32,
    anchor: BeamAnchor,
) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let len_px = (dx * dx + dy * dy).sqrt();

    if len_px < 0.1 {
        return;
    }

    let dir = egui::Vec2::new(dx / len_px, dy / len_px);
    let perp = egui::Vec2::new(-dir.y, dir.x);

    let width_px = beam_type.width * scale;
    let half_w = width_px / 2.0;

    let offset_px = match anchor {
        BeamAnchor::Center => 0.0,
        BeamAnchor::Top => half_w,
        BeamAnchor::Bottom => -half_w,
    };

    // Corners of the beam body
    let points = [
        start + perp * (offset_px + half_w),
        end + perp * (offset_px + half_w),
        end + perp * (offset_px - half_w),
        start + perp * (offset_px - half_w),
    ];

    let concrete_color = egui::Color32::from_rgb(180, 180, 190);
    let fill = concrete_color.linear_multiply(0.2 * alpha_mul);
    let stroke = concrete_color.linear_multiply(alpha_mul);

    painter.add(egui::Shape::convex_polygon(
        points.to_vec(),
        fill,
        egui::Stroke::new(1.5, stroke),
    ));

    // --- REBAR VISUALIZATION ---
    if alpha_mul > 0.1 {
        let cover_px = 3.0 * scale; // 3cm concrete cover
        let inner_hw = half_w - cover_px;

        if inner_hw > 0.0 {
            let rebar_color = egui::Color32::from_rgb(60, 60, 70);
            let rebar_stroke = egui::Stroke::new(1.0, rebar_color.linear_multiply(alpha_mul));

            // Offsets for rebar relative to center points
            let top_offset = offset_px + inner_hw;
            let bot_offset = offset_px - inner_hw;

            // Top bars
            if beam_type.top_bar_count > 0 {
                painter.line_segment(
                    [start + perp * top_offset, end + perp * top_offset],
                    rebar_stroke,
                );
            }

            // Bottom bars
            if beam_type.bottom_bar_count > 0 {
                painter.line_segment(
                    [start + perp * bot_offset, end + perp * bot_offset],
                    rebar_stroke,
                );
            }

            // Side bars
            if beam_type.side_bar_count > 0 {
                let count = beam_type.side_bar_count;
                let step = (inner_hw * 2.0) / (count + 1) as f32;
                for i in 1..=count {
                    let side_offset = offset_px + inner_hw - (i as f32 * step);
                    painter.line_segment(
                        [start + perp * side_offset, end + perp * side_offset],
                        rebar_stroke,
                    );
                }
            }
        }
    }

    // Draw center line
    painter.line_segment(
        [start, end],
        egui::Stroke::new(0.5, stroke.linear_multiply(0.5)),
    );
}
