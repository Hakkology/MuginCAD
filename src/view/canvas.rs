use crate::model::{Entity, Vector2};
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_canvas(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

    let rect = response.rect;

    // Convert mouse position to CAD coordinates
    if response.clicked() {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let cad_pos = Vector2::new(
                mouse_pos.x - rect.center().x,
                -(mouse_pos.y - rect.center().y),
            );
            vm.handle_click(cad_pos);
        }
    }

    // Grid (optional but nice for CAD)
    let grid_size = 50.0;
    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 40));

    // Simple coordinate transformation (center of screen is 0,0)
    let to_screen = |pos: Vector2| {
        egui::pos2(
            rect.center().x + pos.x,
            rect.center().y - pos.y, // Y is up in CAD
        )
    };

    // Draw grid
    let mut x = (rect.min.x / grid_size).floor() * grid_size;
    while x < rect.max.x {
        painter.line_segment(
            [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
            stroke,
        );
        x += grid_size;
    }
    let mut y = (rect.min.y / grid_size).floor() * grid_size;
    while y < rect.max.y {
        painter.line_segment(
            [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
            stroke,
        );
        y += grid_size;
    }

    // Axes
    painter.line_segment(
        [
            egui::pos2(rect.min.x, rect.center().y),
            egui::pos2(rect.max.x, rect.center().y),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60)),
    );
    painter.line_segment(
        [
            egui::pos2(rect.center().x, rect.min.y),
            egui::pos2(rect.center().x, rect.max.y),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60)),
    );

    // Draw entities
    let mut hovered_entity_idx = None;
    if let Some(mouse_pos) = response.hover_pos() {
        let cad_mouse = Vector2::new(
            mouse_pos.x - rect.center().x,
            -(mouse_pos.y - rect.center().y),
        );
        hovered_entity_idx = vm.model.pick_entity(cad_mouse, 5.0);
    }

    for (i, entity) in vm.model.entities.iter().enumerate() {
        let is_selected = Some(i) == vm.selected_entity_idx;
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
                painter.line_segment(
                    [to_screen(line.start), to_screen(line.end)],
                    egui::Stroke::new(stroke_width, color),
                );
            }
            Entity::Circle(circle) => {
                let center_screen = to_screen(circle.center);
                let radius_pixels = circle.radius;
                if circle.filled {
                    painter.circle_filled(center_screen, radius_pixels, color.linear_multiply(0.3));
                }
                painter.circle_stroke(
                    center_screen,
                    radius_pixels,
                    egui::Stroke::new(stroke_width, color),
                );
            }
            Entity::Rectangle(rect) => {
                let rect_screen = egui::Rect::from_min_max(
                    to_screen(Vector2::new(rect.min.x, rect.max.y)),
                    to_screen(Vector2::new(rect.max.x, rect.min.y)),
                );
                if rect.filled {
                    painter.rect_filled(rect_screen, 0.0, color.linear_multiply(0.3));
                }
                painter.rect_stroke(rect_screen, 0.0, egui::Stroke::new(stroke_width, color));
            }
        }
    }

    // Cursor Crosshairs
    if let Some(mouse_pos) = response.hover_pos() {
        let cross_stroke = egui::Stroke::new(
            0.5,
            egui::Color32::from_rgba_unmultiplied(200, 200, 200, 100),
        );
        // Horizontal
        painter.line_segment(
            [
                egui::pos2(rect.min.x, mouse_pos.y),
                egui::pos2(rect.max.x, mouse_pos.y),
            ],
            cross_stroke,
        );
        // Vertical
        painter.line_segment(
            [
                egui::pos2(mouse_pos.x, rect.min.y),
                egui::pos2(mouse_pos.x, rect.max.y),
            ],
            cross_stroke,
        );

        // Previews
        let preview_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
        );

        if let crate::viewmodel::CommandState::WaitingForPoints { cmd, points } = &vm.state {
            if let Some(&last_point) = points.last() {
                let current_cad = Vector2::new(
                    mouse_pos.x - rect.center().x,
                    -(mouse_pos.y - rect.center().y),
                );

                match cmd {
                    crate::viewmodel::CommandType::Line => {
                        painter.line_segment(
                            [to_screen(last_point), to_screen(current_cad)],
                            preview_stroke,
                        );
                    }
                    crate::viewmodel::CommandType::Circle => {
                        let center = points[0];
                        let radius = center.dist(current_cad);
                        painter.circle_stroke(to_screen(center), radius, preview_stroke);
                    }
                    crate::viewmodel::CommandType::Rectangle => {
                        let start = points[0];
                        let min =
                            Vector2::new(start.x.min(current_cad.x), start.y.min(current_cad.y));
                        let max =
                            Vector2::new(start.x.max(current_cad.x), start.y.max(current_cad.y));
                        let rect_screen = egui::Rect::from_min_max(
                            to_screen(Vector2::new(min.x, max.y)),
                            to_screen(Vector2::new(max.x, min.y)),
                        );
                        painter.rect_stroke(rect_screen, 0.0, preview_stroke);
                    }
                }
            }
        }
    }
}
