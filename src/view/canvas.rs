use crate::commands::InputModifiers;
use crate::model::snap::SnapPointType;
use crate::model::{Entity, Vector2};
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_canvas(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

    let rect = response.rect;

    // Update modifiers from UI
    let modifiers = ui.input(|i| InputModifiers {
        shift: i.modifiers.shift,
        ctrl: i.modifiers.ctrl,
        alt: i.modifiers.alt,
    });
    vm.set_modifiers(modifiers);

    // Coordinate transformation
    let to_screen = |pos: Vector2| egui::pos2(rect.center().x + pos.x, rect.center().y - pos.y);

    let to_cad = |screen_pos: egui::Pos2| {
        Vector2::new(
            screen_pos.x - rect.center().x,
            -(screen_pos.y - rect.center().y),
        )
    };

    // Update snap point if mouse is hovering
    if let Some(mouse_pos) = response.hover_pos() {
        let cad_pos = to_cad(mouse_pos);
        vm.update_snap(cad_pos, modifiers);
    }

    // Handle clicks
    if response.clicked() {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let cad_pos = to_cad(mouse_pos);
            vm.handle_click(cad_pos);
        }
    } else if response.secondary_clicked() {
        vm.cancel_command();
    }

    // Draw grid
    let grid_size = 50.0;
    let grid_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 40));

    let mut x = (rect.min.x / grid_size).floor() * grid_size;
    while x < rect.max.x {
        painter.line_segment(
            [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
            grid_stroke,
        );
        x += grid_size;
    }
    let mut y = (rect.min.y / grid_size).floor() * grid_size;
    while y < rect.max.y {
        painter.line_segment(
            [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
            grid_stroke,
        );
        y += grid_size;
    }

    // Axes
    let axis_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60));
    painter.line_segment(
        [
            egui::pos2(rect.min.x, rect.center().y),
            egui::pos2(rect.max.x, rect.center().y),
        ],
        axis_stroke,
    );
    painter.line_segment(
        [
            egui::pos2(rect.center().x, rect.min.y),
            egui::pos2(rect.center().x, rect.max.y),
        ],
        axis_stroke,
    );

    // Hover detection
    let mut hovered_entity_idx = None;
    if let Some(mouse_pos) = response.hover_pos() {
        let cad_mouse = to_cad(mouse_pos);
        hovered_entity_idx = vm.model.pick_entity(cad_mouse, 5.0);
    }

    // Draw entities
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
                if circle.filled {
                    painter.circle_filled(
                        to_screen(circle.center),
                        circle.radius,
                        color.linear_multiply(0.3),
                    );
                }
                painter.circle_stroke(
                    to_screen(circle.center),
                    circle.radius,
                    egui::Stroke::new(stroke_width, color),
                );
            }
            Entity::Rectangle(rect_entity) => {
                let rect_screen = egui::Rect::from_min_max(
                    to_screen(Vector2::new(rect_entity.min.x, rect_entity.max.y)),
                    to_screen(Vector2::new(rect_entity.max.x, rect_entity.min.y)),
                );
                if rect_entity.filled {
                    painter.rect_filled(rect_screen, 0.0, color.linear_multiply(0.3));
                }
                painter.rect_stroke(rect_screen, 0.0, egui::Stroke::new(stroke_width, color));
            }
        }
    }

    // Cursor and Preview
    if let Some(mouse_pos) = response.hover_pos() {
        let raw_cad = to_cad(mouse_pos);

        // Get effective position (with snap if Ctrl is pressed)
        let effective_cad = vm.get_effective_position(raw_cad);

        // Draw crosshairs at effective position
        let cross_stroke = egui::Stroke::new(
            0.5,
            egui::Color32::from_rgba_unmultiplied(200, 200, 200, 100),
        );
        let effective_screen = to_screen(effective_cad);
        painter.line_segment(
            [
                egui::pos2(rect.min.x, effective_screen.y),
                egui::pos2(rect.max.x, effective_screen.y),
            ],
            cross_stroke,
        );
        painter.line_segment(
            [
                egui::pos2(effective_screen.x, rect.min.y),
                egui::pos2(effective_screen.x, rect.max.y),
            ],
            cross_stroke,
        );

        // Draw snap indicator if snapping
        if let Some(snap) = &vm.current_snap {
            let snap_screen = to_screen(snap.position);
            let snap_color = match snap.point_type {
                SnapPointType::Endpoint => egui::Color32::GREEN,
                SnapPointType::Center => egui::Color32::YELLOW,
                SnapPointType::Corner => egui::Color32::LIGHT_GREEN,
                SnapPointType::Intersection => egui::Color32::RED,
                SnapPointType::Midpoint => egui::Color32::LIGHT_BLUE,
            };

            // Draw snap marker (diamond shape)
            let size = 8.0;
            painter.add(egui::Shape::convex_polygon(
                vec![
                    egui::pos2(snap_screen.x, snap_screen.y - size),
                    egui::pos2(snap_screen.x + size, snap_screen.y),
                    egui::pos2(snap_screen.x, snap_screen.y + size),
                    egui::pos2(snap_screen.x - size, snap_screen.y),
                ],
                snap_color.linear_multiply(0.3),
                egui::Stroke::new(2.0, snap_color),
            ));
        }

        // Command preview with modifier support
        let preview_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
        );

        if let Some((cmd, points)) = vm.executor.get_preview_points() {
            if let Some(&last_point) = points.last() {
                // Apply constraints for preview
                let current_cad = cmd.constrain_point(effective_cad, Some(last_point), modifiers);

                match cmd.name() {
                    "LINE" => {
                        painter.line_segment(
                            [to_screen(last_point), to_screen(current_cad)],
                            preview_stroke,
                        );
                    }
                    "CIRCLE" => {
                        let center = points[0];
                        let radius = center.dist(current_cad);
                        painter.circle_stroke(to_screen(center), radius, preview_stroke);
                    }
                    "RECTANGLE" => {
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
                    _ => {}
                }
            }
        }
    }
}
