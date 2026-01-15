use crate::commands::InputModifiers;
// use crate::model::snap::SnapPointType;
use crate::model::Vector2;
use crate::view::rendering::context::DrawContext;
use crate::view::rendering::renderer;
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_canvas(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

    let rect = response.rect;
    let screen_center = Vector2::new(rect.center().x, rect.center().y);

    if vm.tabs.is_empty() {
        return;
    }

    // Gather inputs first
    let modifiers = ui.input(|i| InputModifiers {
        shift: i.modifiers.shift,
        ctrl: i.modifiers.ctrl,
        alt: i.modifiers.alt,
    });

    // Update global modifier state
    vm.set_modifiers(modifiers);

    let middle_pressed = ui.input(|i| i.pointer.middle_down());
    let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);

    let hover_pos = response.hover_pos();
    let dragged = response.dragged();
    let drag_started = response.drag_started();
    let drag_stopped = response.drag_stopped();
    let clicked = response.clicked();
    let secondary_clicked = response.secondary_clicked();
    let r_pressed = ui.input(|i| i.key_pressed(egui::Key::R));

    // Viewport manipulation (Directly on active tab)
    let is_panning;
    let viewport_zoom;
    let viewport_offset;

    {
        let tab = vm.active_tab_mut();
        if let Some(mouse_pos) = hover_pos {
            let screen_pos = Vector2::new(mouse_pos.x, mouse_pos.y);
            if middle_pressed && !tab.viewport.is_panning {
                tab.viewport.start_pan(screen_pos);
            } else if middle_pressed && tab.viewport.is_panning {
                tab.viewport.update_pan(screen_pos);
            } else if !middle_pressed && tab.viewport.is_panning {
                tab.viewport.end_pan();
            }
        } else if !middle_pressed {
            tab.viewport.end_pan();
        }

        if scroll_delta != 0.0 {
            if let Some(mouse_pos) = hover_pos {
                let screen_pos = Vector2::new(mouse_pos.x, mouse_pos.y);
                let zoom_delta = scroll_delta / 50.0;
                tab.viewport.zoom_at(screen_pos, screen_center, zoom_delta);
            }
        }

        is_panning = tab.viewport.is_panning;
        viewport_zoom = tab.viewport.zoom;
        viewport_offset = tab.viewport.offset;
    }

    // Create DrawContext (needs immutable props)
    let ctx = DrawContext {
        painter: &painter,
        zoom: viewport_zoom,
        offset: viewport_offset,
        screen_center,
        ui,
    };

    // Handle Input logic (Calls methods on VM)
    if let Some(mouse_pos) = hover_pos {
        if !is_panning {
            let cad_pos = ctx.to_cad(mouse_pos);
            vm.update_snap(cad_pos, modifiers);
        }
    }

    if !is_panning {
        if drag_started {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let cad_pos = ctx.to_cad(mouse_pos);
                vm.handle_drag_start(cad_pos, modifiers);
            }
        }
        if dragged {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let cad_pos = ctx.to_cad(mouse_pos);
                vm.handle_drag_update(cad_pos);
            }
        }
        if drag_stopped {
            vm.handle_drag_end(modifiers);
        }
        if clicked {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let cad_pos = ctx.to_cad(mouse_pos);
                vm.handle_click(cad_pos, modifiers);
            }
        } else if secondary_clicked {
            vm.cancel_command();
        }

        if r_pressed && !modifiers.ctrl && !modifiers.shift {
            vm.active_tab_mut().executor.toggle_arc_direction();
        }
    }

    // RENDER PHASE (Borrows tab and config)

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

    // We can now borrow tab for rendering
    let tab = vm.active_tab();

    // Construction Axes
    let axis_color = egui::Color32::from_rgb(150, 50, 50);
    let axis_line_stroke = egui::Stroke::new(1.5, axis_color);
    let bubble_radius = 12.0;

    for axis in &tab.model.axis_manager.axes {
        let cad_min = ctx.to_cad(egui::pos2(rect.min.x, rect.max.y));
        let cad_max = ctx.to_cad(egui::pos2(rect.max.x, rect.min.y));
        let (start, end) = axis.get_render_points(cad_min, cad_max);
        let start_screen = ctx.to_screen(start);
        let end_screen = ctx.to_screen(end);
        painter.line_segment([start_screen, end_screen], axis_line_stroke);

        let label_pos = axis.get_label_position(cad_min, cad_max);
        let label_screen = ctx.to_screen(label_pos);
        let bubble_pos = match axis.orientation {
            crate::model::axis::AxisOrientation::Vertical => {
                egui::pos2(label_screen.x, rect.min.y + bubble_radius + 5.0)
            }
            crate::model::axis::AxisOrientation::Horizontal => {
                egui::pos2(rect.min.x + bubble_radius + 5.0, label_screen.y)
            }
        };
        painter.circle_filled(bubble_pos, bubble_radius, axis_color);
        painter.circle_stroke(
            bubble_pos,
            bubble_radius,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
        painter.text(
            bubble_pos,
            egui::Align2::CENTER_CENTER,
            &axis.label,
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );
    }

    // Hover detection - access tab
    let mut hovered_entity_idx = None;
    if let Some(mouse_pos) = hover_pos {
        if !is_panning {
            let cad_mouse = ctx.to_cad(mouse_pos);
            let tolerance = 5.0 / viewport_zoom;
            hovered_entity_idx = tab.model.pick_entity(cad_mouse, tolerance);
        }
    }

    // Selection Box
    if let (Some(start), Some(current)) = (
        tab.selection_manager.selection_rect_start,
        tab.selection_manager.selection_rect_current,
    ) {
        let min_x = start.x.min(current.x);
        let max_x = start.x.max(current.x);
        let min_y = start.y.min(current.y);
        let max_y = start.y.max(current.y);
        let rect_screen = egui::Rect::from_min_max(
            ctx.to_screen(Vector2::new(min_x, max_y)),
            ctx.to_screen(Vector2::new(max_x, min_y)),
        );
        let color = egui::Color32::from_rgba_unmultiplied(100, 100, 255, 30);
        let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 255));
        painter.rect_filled(rect_screen, 0.0, color);
        painter.rect_stroke(rect_screen, 0.0, stroke);
    }

    // Entities
    renderer::render_entities(
        &ctx,
        &tab.model.entities,
        &tab.selection_manager.selected_indices,
        hovered_entity_idx,
    );

    // Cursor and Preview
    if let Some(mouse_pos) = hover_pos {
        if !is_panning {
            let raw_cad = ctx.to_cad(mouse_pos);
            let effective_cad = vm.get_effective_position(raw_cad); // Call VM method

            let cross_stroke = egui::Stroke::new(
                0.5,
                egui::Color32::from_rgba_unmultiplied(200, 200, 200, 100),
            );
            let effective_screen = ctx.to_screen(effective_cad);
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

            // Re-borrow active tab just for snap/preview data?
            // We called vm.get_effective_position which uses active_tab().
            // So we need to use 'tab' for rendering again. 'tab' is immutable borrow.
            let tab = vm.active_tab();

            if let Some(snap) = &tab.current_snap {
                let snap_screen = ctx.to_screen(snap.position);
                let snap_color = match snap.point_type {
                    crate::model::snap::SnapPointType::Endpoint => egui::Color32::GREEN,
                    crate::model::snap::SnapPointType::Center => egui::Color32::YELLOW,
                    crate::model::snap::SnapPointType::Corner => egui::Color32::LIGHT_GREEN,
                    crate::model::snap::SnapPointType::Intersection => egui::Color32::RED,
                    crate::model::snap::SnapPointType::Midpoint => egui::Color32::LIGHT_BLUE,
                    crate::model::snap::SnapPointType::AxisLine => {
                        egui::Color32::from_rgb(255, 128, 0)
                    }
                    crate::model::snap::SnapPointType::Grid => {
                        egui::Color32::from_rgb(200, 200, 200)
                    }
                };
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

            if let Some((cmd, points)) = tab.executor.get_preview_points() {
                let current_cad = if let Some(&last_point) = points.last() {
                    cmd.constrain_point(effective_cad, Some(last_point), modifiers)
                } else {
                    effective_cad
                };
                cmd.draw_preview(&ctx, points, current_cad);
            }
        }
    }

    // Pan indicator
    if is_panning {
        painter.text(
            egui::pos2(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Panning...",
            egui::FontId::default(),
            egui::Color32::WHITE,
        );
    }

    // Zoom indicator
    let zoom_percent = (viewport_zoom * 100.0) as i32;
    let zoom_text = format!("Zoom: {}%", zoom_percent);
    painter.text(
        egui::pos2(rect.min.x + 10.0, rect.max.y - 10.0),
        egui::Align2::LEFT_BOTTOM,
        zoom_text,
        egui::FontId::default(),
        egui::Color32::from_rgba_unmultiplied(200, 200, 200, 180),
    );
}
