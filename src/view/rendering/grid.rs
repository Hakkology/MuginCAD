use crate::model::Vector2;
use crate::view::rendering::context::DrawContext;
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_grid_and_axes(ctx: &DrawContext, vm: &CadViewModel, rect: egui::Rect) {
    let painter = ctx.painter;
    let viewport_zoom = ctx.zoom;

    // Grid
    if vm.config.grid_config.show_grid {
        let grid_size = vm.config.grid_config.grid_size * viewport_zoom;
        let c = vm.config.grid_config.grid_color;
        let grid_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(c[0], c[1], c[2]));
        let origin_screen = ctx.to_screen(Vector2::new(0.0, 0.0));

        let mut x = rect.min.x + (origin_screen.x - rect.min.x).rem_euclid(grid_size);
        while x < rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                grid_stroke,
            );
            x += grid_size;
        }
        let mut y = rect.min.y + (origin_screen.y - rect.min.y).rem_euclid(grid_size);
        while y < rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                grid_stroke,
            );
            y += grid_size;
        }
    }

    // Axes
    let origin_screen = ctx.to_screen(Vector2::new(0.0, 0.0));
    let ac = vm.config.grid_config.axis_color;
    let axis_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(ac[0], ac[1], ac[2]));

    if origin_screen.y >= rect.min.y && origin_screen.y <= rect.max.y {
        painter.line_segment(
            [
                egui::pos2(rect.min.x, origin_screen.y),
                egui::pos2(rect.max.x, origin_screen.y),
            ],
            axis_stroke,
        );
    }
    if origin_screen.x >= rect.min.x && origin_screen.x <= rect.max.x {
        painter.line_segment(
            [
                egui::pos2(origin_screen.x, rect.min.y),
                egui::pos2(origin_screen.x, rect.max.y),
            ],
            axis_stroke,
        );
    }
}
