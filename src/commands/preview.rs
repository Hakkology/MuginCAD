use crate::model::Vector2;
use crate::view::rendering::context::DrawContext;
use eframe::egui;

/// Standart preview stroke (beyaz, yarı-saydam)
pub fn preview_stroke() -> egui::Stroke {
    egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
    )
}

/// İki nokta arası preview çizgisi çiz
pub fn draw_line_to_cursor(ctx: &DrawContext, from: Vector2, to: Vector2) {
    ctx.painter
        .line_segment([ctx.to_screen(from), ctx.to_screen(to)], preview_stroke());
}

/// Merkez işareti çiz (sarı daire, 4px)
pub fn draw_center_marker(ctx: &DrawContext, center: Vector2) {
    ctx.painter.circle_stroke(
        ctx.to_screen(center),
        4.0,
        egui::Stroke::new(1.5, egui::Color32::YELLOW),
    );
}

/// Dolu nokta işareti çiz (istenen renkte, 3px)
pub fn draw_point_marker(ctx: &DrawContext, point: Vector2, color: egui::Color32) {
    ctx.painter.circle_filled(ctx.to_screen(point), 3.0, color);
}

/// Boyut yazısı çiz (turuncu, 11pt)
pub fn draw_dimension_text(ctx: &DrawContext, pos: egui::Pos2, text: String) {
    let dim_color = egui::Color32::from_rgb(255, 200, 100);
    let dim_font = egui::FontId::proportional(11.0);
    ctx.painter
        .text(pos, egui::Align2::CENTER_CENTER, text, dim_font, dim_color);
}
