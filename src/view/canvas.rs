use crate::commands::InputModifiers;
use crate::model::snap::SnapPointType;
use crate::model::{Entity, Vector2};
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_canvas(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

    let rect = response.rect;
    let screen_center = Vector2::new(rect.center().x, rect.center().y);

    // Update modifiers from UI
    let modifiers = ui.input(|i| InputModifiers {
        shift: i.modifiers.shift,
        ctrl: i.modifiers.ctrl,
        alt: i.modifiers.alt,
    });
    vm.set_modifiers(modifiers);

    // Handle middle mouse button panning
    let middle_pressed = ui.input(|i| i.pointer.middle_down());
    if let Some(mouse_pos) = response.hover_pos() {
        let screen_pos = Vector2::new(mouse_pos.x, mouse_pos.y);

        if middle_pressed && !vm.viewport.is_panning {
            vm.viewport.start_pan(screen_pos);
        } else if middle_pressed && vm.viewport.is_panning {
            vm.viewport.update_pan(screen_pos);
        } else if !middle_pressed && vm.viewport.is_panning {
            vm.viewport.end_pan();
        }
    } else if !middle_pressed {
        vm.viewport.end_pan();
    }

    // Handle mouse wheel zoom
    let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
    if scroll_delta != 0.0 {
        if let Some(mouse_pos) = response.hover_pos() {
            let screen_pos = Vector2::new(mouse_pos.x, mouse_pos.y);
            let zoom_delta = scroll_delta / 50.0; // Normalize scroll speed
            vm.viewport.zoom_at(screen_pos, screen_center, zoom_delta);
        }
    }

    // Get viewport state for coordinate transformations
    let viewport_offset = vm.viewport.offset;
    let viewport_zoom = vm.viewport.zoom;
    let zoom_percent = vm.viewport.zoom_percent();
    let is_panning = vm.viewport.is_panning;

    // Coordinate transformation functions (using captured values, not references)
    let to_screen = |pos: Vector2| -> egui::Pos2 {
        egui::pos2(
            screen_center.x + pos.x * viewport_zoom + viewport_offset.x,
            screen_center.y - pos.y * viewport_zoom + viewport_offset.y,
        )
    };

    let to_cad = |screen_pos: egui::Pos2| -> Vector2 {
        Vector2::new(
            (screen_pos.x - screen_center.x - viewport_offset.x) / viewport_zoom,
            -(screen_pos.y - screen_center.y - viewport_offset.y) / viewport_zoom,
        )
    };

    // Update snap point if mouse is hovering (and not panning)
    if let Some(mouse_pos) = response.hover_pos() {
        if !is_panning {
            let cad_pos = to_cad(mouse_pos);
            vm.update_snap(cad_pos, modifiers);
        }
    }

    // Handle input (clicks and drags) when not panning
    if !is_panning {
        // Handle drag events
        if response.drag_started() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let cad_pos = to_cad(mouse_pos);
                vm.handle_drag_start(cad_pos, modifiers);
            }
        }

        if response.dragged() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let cad_pos = to_cad(mouse_pos);
                vm.handle_drag_update(cad_pos);
            }
        }

        if response.drag_stopped() {
            vm.handle_drag_end(modifiers);
        }

        // Handle clicks
        if response.clicked() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let cad_pos = to_cad(mouse_pos);
                vm.handle_click(cad_pos, modifiers);
            }
        } else if response.secondary_clicked() {
            vm.cancel_command();
        }
    }

    // Draw grid (with viewport offset)
    if vm.config.grid_config.show_grid {
        let grid_size = vm.config.grid_config.grid_size * viewport_zoom;
        let c = vm.config.grid_config.grid_color;
        let grid_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(c[0], c[1], c[2]));

        // Calculate grid origin in screen space
        let origin_screen = to_screen(Vector2::new(0.0, 0.0));

        // Draw vertical grid lines
        let mut x = rect.min.x + (origin_screen.x - rect.min.x).rem_euclid(grid_size);
        while x < rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                grid_stroke,
            );
            x += grid_size;
        }

        // Draw horizontal grid lines
        let mut y = rect.min.y + (origin_screen.y - rect.min.y).rem_euclid(grid_size);
        while y < rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                grid_stroke,
            );
            y += grid_size;
        }
    }

    let origin_screen = to_screen(Vector2::new(0.0, 0.0));

    // Axes (at origin, with viewport)
    let ac = vm.config.grid_config.axis_color;
    let axis_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(ac[0], ac[1], ac[2]));

    // X axis (horizontal line through origin)
    if origin_screen.y >= rect.min.y && origin_screen.y <= rect.max.y {
        painter.line_segment(
            [
                egui::pos2(rect.min.x, origin_screen.y),
                egui::pos2(rect.max.x, origin_screen.y),
            ],
            axis_stroke,
        );
    }

    // Y axis (vertical line through origin)
    if origin_screen.x >= rect.min.x && origin_screen.x <= rect.max.x {
        painter.line_segment(
            [
                egui::pos2(origin_screen.x, rect.min.y),
                egui::pos2(origin_screen.x, rect.max.y),
            ],
            axis_stroke,
        );
    }

    // Draw construction axes (akslar)
    let axis_color = egui::Color32::from_rgb(150, 50, 50); // Dark red
    let axis_line_stroke = egui::Stroke::new(1.5, axis_color);
    let bubble_radius = 12.0;

    for axis in &vm.model.axis_manager.axes {
        // Get viewport bounds in CAD coordinates
        let cad_min = to_cad(egui::pos2(rect.min.x, rect.max.y));
        let cad_max = to_cad(egui::pos2(rect.max.x, rect.min.y));

        // Get render points
        let (start, end) = axis.get_render_points(cad_min, cad_max);
        let start_screen = to_screen(start);
        let end_screen = to_screen(end);

        // Draw the axis line (dashed effect with segments)
        painter.line_segment([start_screen, end_screen], axis_line_stroke);

        // Get label position
        let label_pos = axis.get_label_position(cad_min, cad_max);
        let label_screen = to_screen(label_pos);

        // Position bubble with offset from edge
        let bubble_pos = match axis.orientation {
            crate::model::axis::AxisOrientation::Vertical => {
                egui::pos2(label_screen.x, rect.min.y + bubble_radius + 5.0)
            }
            crate::model::axis::AxisOrientation::Horizontal => {
                egui::pos2(rect.min.x + bubble_radius + 5.0, label_screen.y)
            }
        };

        // Draw bubble (circle with label)
        painter.circle_filled(bubble_pos, bubble_radius, axis_color);
        painter.circle_stroke(
            bubble_pos,
            bubble_radius,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );

        // Draw label text
        painter.text(
            bubble_pos,
            egui::Align2::CENTER_CENTER,
            &axis.label,
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );
    }

    // Hover detection
    let mut hovered_entity_idx = None;
    if let Some(mouse_pos) = response.hover_pos() {
        if !is_panning {
            let cad_mouse = to_cad(mouse_pos);
            // Adjust tolerance for zoom level
            let tolerance = 5.0 / viewport_zoom;
            hovered_entity_idx = vm.model.pick_entity(cad_mouse, tolerance);
        }
    }

    // Draw drag selection box
    if let (Some(start), Some(current)) = (vm.selection_rect_start, vm.selection_rect_current) {
        let min_x = start.x.min(current.x);
        let max_x = start.x.max(current.x);
        let min_y = start.y.min(current.y);
        let max_y = start.y.max(current.y);

        let rect_screen = egui::Rect::from_min_max(
            to_screen(Vector2::new(min_x, max_y)),
            to_screen(Vector2::new(max_x, min_y)),
        );

        let color = egui::Color32::from_rgba_unmultiplied(100, 100, 255, 30);
        let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 255));

        painter.rect_filled(rect_screen, 0.0, color);
        painter.rect_stroke(rect_screen, 0.0, stroke);
    }

    // Draw entities
    for (i, entity) in vm.model.entities.iter().enumerate() {
        let is_selected = vm.selected_indices.contains(&i);
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
                let screen_radius = circle.radius * viewport_zoom;
                if circle.filled {
                    painter.circle_filled(
                        to_screen(circle.center),
                        screen_radius,
                        color.linear_multiply(0.3),
                    );
                }
                painter.circle_stroke(
                    to_screen(circle.center),
                    screen_radius,
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

    // Cursor and Preview (only when not panning)
    if let Some(mouse_pos) = response.hover_pos() {
        if !is_panning {
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
                    SnapPointType::AxisLine => egui::Color32::from_rgb(255, 128, 0), // Orange
                    SnapPointType::Grid => egui::Color32::from_rgb(200, 200, 200),   // Light Gray
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
                    let current_cad =
                        cmd.constrain_point(effective_cad, Some(last_point), modifiers);

                    match cmd.name() {
                        "LINE" => {
                            painter.line_segment(
                                [to_screen(last_point), to_screen(current_cad)],
                                preview_stroke,
                            );
                        }
                        "CIRCLE" => {
                            let center = points[0];
                            let radius = center.dist(current_cad) * viewport_zoom;
                            painter.circle_stroke(to_screen(center), radius, preview_stroke);
                        }
                        "RECTANGLE" => {
                            let start = points[0];
                            let min = Vector2::new(
                                start.x.min(current_cad.x),
                                start.y.min(current_cad.y),
                            );
                            let max = Vector2::new(
                                start.x.max(current_cad.x),
                                start.y.max(current_cad.y),
                            );
                            let rect_screen = egui::Rect::from_min_max(
                                to_screen(Vector2::new(min.x, max.y)),
                                to_screen(Vector2::new(max.x, min.y)),
                            );
                            painter.rect_stroke(rect_screen, 0.0, preview_stroke);
                        }
                        "MOVE" => {
                            // Draw a line from base point to destination
                            let base = points[0];
                            painter.line_segment(
                                [to_screen(base), to_screen(current_cad)],
                                preview_stroke,
                            );
                            // Draw small circles at endpoints
                            painter.circle_stroke(to_screen(base), 4.0, preview_stroke);
                            painter.circle_filled(
                                to_screen(current_cad),
                                3.0,
                                egui::Color32::WHITE,
                            );
                        }
                        "ROTATE" => {
                            // Draw a line from pivot to mouse indicating rotation
                            let pivot = points[0];
                            let pivot_screen = to_screen(pivot);

                            // Calculate angle and radius
                            let dx = current_cad.x - pivot.x;
                            let dy = current_cad.y - pivot.y;
                            let angle = dy.atan2(dx);
                            let radius = (dx * dx + dy * dy).sqrt() * viewport_zoom;

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
                                let py = pivot_screen.y - arc_radius * a.sin(); // Y is inverted
                                arc_points.push(egui::pos2(px, py));
                            }

                            // Draw arc as line segments
                            for i in 0..arc_points.len().saturating_sub(1) {
                                painter.line_segment(
                                    [arc_points[i], arc_points[i + 1]],
                                    egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 100)),
                                );
                            }

                            // Draw radius line from pivot to mouse
                            painter.line_segment(
                                [pivot_screen, to_screen(current_cad)],
                                preview_stroke,
                            );

                            // Draw reference line (horizontal from pivot)
                            let ref_end = egui::pos2(pivot_screen.x + arc_radius, pivot_screen.y);
                            painter.line_segment(
                                [pivot_screen, ref_end],
                                egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgba_unmultiplied(150, 150, 150, 100),
                                ),
                            );

                            // Draw pivot marker
                            painter.circle_stroke(
                                pivot_screen,
                                5.0,
                                egui::Stroke::new(2.0, egui::Color32::YELLOW),
                            );
                        }

                        "SCALE" => {
                            // Draw a line from base point to mouse indicating scale reference
                            let base = points[0];
                            painter.line_segment(
                                [to_screen(base), to_screen(current_cad)],
                                preview_stroke,
                            );
                            // Draw base marker (filled square)
                            let base_screen = to_screen(base);
                            let size = 4.0;
                            painter.rect_filled(
                                egui::Rect::from_center_size(
                                    base_screen,
                                    egui::vec2(size * 2.0, size * 2.0),
                                ),
                                0.0,
                                egui::Color32::WHITE,
                            );
                        }
                        "COPY" | "CUT" => {
                            // Draw a dashed line from base point to destination
                            let base = points[0];
                            painter.line_segment(
                                [to_screen(base), to_screen(current_cad)],
                                egui::Stroke::new(1.5, egui::Color32::from_rgb(100, 200, 255)),
                            );
                            // Draw markers
                            painter.circle_stroke(
                                to_screen(base),
                                4.0,
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255)),
                            );
                            painter.circle_filled(
                                to_screen(current_cad),
                                3.0,
                                egui::Color32::from_rgb(100, 200, 255),
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Draw pan indicator when panning
    if is_panning {
        painter.text(
            egui::pos2(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Panning...",
            egui::FontId::default(),
            egui::Color32::WHITE,
        );
    }

    // Draw zoom indicator (bottom-left corner)
    let zoom_text = format!("Zoom: {}%", zoom_percent);
    painter.text(
        egui::pos2(rect.min.x + 10.0, rect.max.y - 10.0),
        egui::Align2::LEFT_BOTTOM,
        zoom_text,
        egui::FontId::default(),
        egui::Color32::from_rgba_unmultiplied(200, 200, 200, 180),
    );
}
