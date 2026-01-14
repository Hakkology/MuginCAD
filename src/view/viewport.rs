use crate::model::Vector2;

/// Camera/viewport settings for the canvas
#[derive(Debug, Clone)]
pub struct Viewport {
    /// Pan offset (how much the view is shifted)
    pub offset: Vector2,
    /// Zoom level (1.0 = 100%)
    pub zoom: f32,
    /// Is middle mouse button currently dragging
    pub is_panning: bool,
    /// Last mouse position during pan
    pub pan_start: Option<Vector2>,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            offset: Vector2::new(0.0, 0.0),
            zoom: 1.0,
            is_panning: false,
            pan_start: None,
        }
    }
}

impl Viewport {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start panning
    pub fn start_pan(&mut self, screen_pos: Vector2) {
        self.is_panning = true;
        self.pan_start = Some(screen_pos);
    }

    /// Update pan during drag
    pub fn update_pan(&mut self, screen_pos: Vector2) {
        if self.is_panning {
            if let Some(start) = self.pan_start {
                let delta = screen_pos - start;
                self.offset = self.offset + delta;
                self.pan_start = Some(screen_pos);
            }
        }
    }

    /// End panning
    pub fn end_pan(&mut self) {
        self.is_panning = false;
        self.pan_start = None;
    }

    /// Convert screen position to CAD coordinates
    pub fn screen_to_cad(&self, screen_pos: Vector2, screen_center: Vector2) -> Vector2 {
        Vector2::new(
            (screen_pos.x - screen_center.x - self.offset.x) / self.zoom,
            -(screen_pos.y - screen_center.y - self.offset.y) / self.zoom,
        )
    }

    /// Convert CAD coordinates to screen position
    pub fn cad_to_screen(&self, cad_pos: Vector2, screen_center: Vector2) -> Vector2 {
        Vector2::new(
            screen_center.x + cad_pos.x * self.zoom + self.offset.x,
            screen_center.y - cad_pos.y * self.zoom + self.offset.y,
        )
    }

    /// Reset viewport to default
    pub fn reset(&mut self) {
        self.offset = Vector2::new(0.0, 0.0);
        self.zoom = 1.0;
    }

    /// Zoom at a specific screen position (for mouse-centered zoom)
    /// delta > 0 = zoom in, delta < 0 = zoom out
    pub fn zoom_at(&mut self, screen_pos: Vector2, screen_center: Vector2, delta: f32) {
        // Calculate zoom factor
        let zoom_speed = 0.1;
        let zoom_factor = 1.0 + delta * zoom_speed;
        let new_zoom = (self.zoom * zoom_factor).clamp(0.1, 10.0); // 10% to 1000%

        if (new_zoom - self.zoom).abs() < 0.001 {
            return;
        }

        // Get mouse position in CAD coordinates before zoom
        let mouse_cad = self.screen_to_cad(screen_pos, screen_center);

        // Apply new zoom
        self.zoom = new_zoom;

        // Adjust offset to keep mouse position stable
        // After zoom, the same CAD point should be under the mouse
        let new_screen = self.cad_to_screen(mouse_cad, screen_center);
        self.offset.x += screen_pos.x - new_screen.x;
        self.offset.y += screen_pos.y - new_screen.y;
    }

    /// Get zoom as percentage (100 = 100%)
    pub fn zoom_percent(&self) -> i32 {
        (self.zoom * 100.0).round() as i32
    }
}
