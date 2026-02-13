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
