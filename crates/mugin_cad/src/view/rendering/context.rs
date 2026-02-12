use crate::model::Vector2;
// use crate::view::viewport::Viewport;
use eframe::egui;

/// Context object passed to rendering functions
pub struct DrawContext<'a> {
    pub painter: &'a egui::Painter,
    // pub viewport: &'a Viewport, // Removed reference to avoid borrow issues
    pub zoom: f32,
    pub offset: Vector2,
    pub screen_center: Vector2,
}

impl<'a> DrawContext<'a> {
    pub fn to_screen(&self, pos: Vector2) -> egui::Pos2 {
        let zoom = self.zoom;
        let offset = self.offset;

        egui::pos2(
            self.screen_center.x + pos.x * zoom + offset.x,
            self.screen_center.y - pos.y * zoom + offset.y,
        )
    }

    pub fn to_cad(&self, screen_pos: egui::Pos2) -> Vector2 {
        let zoom = self.zoom;
        let offset = self.offset;

        Vector2::new(
            (screen_pos.x - self.screen_center.x - offset.x) / zoom,
            -(screen_pos.y - self.screen_center.y - offset.y) / zoom,
        )
    }
}
