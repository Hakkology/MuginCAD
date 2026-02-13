use crate::model::structure::column_type::ColumnType;
use crate::model::structure::material::MaterialProperties;
use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::{card, window};

pub fn render_column_manager(ctx: &egui::Context, vm: &mut CadViewModel) {
    let mut open = vm.column_manager_open;

    // Reduced width to 850.0 as requested
    window::window("Column Types", ctx, &mut open, [850.0, 600.0], true, |ui| {
        render_column_ui(ctx, ui, vm);
    });

    vm.column_manager_open = open;
}

fn render_column_ui(ctx: &egui::Context, ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let mut resize_events = Vec::new();

    {
        let tab = vm.active_tab_mut();
        let definitions = &mut tab.model.definitions;

        // Collect material options once for the frame
        let mut concrete_options = Vec::new();
        let mut steel_options = Vec::new();
        for (mid, m) in &definitions.materials {
            match m.properties {
                MaterialProperties::Concrete { .. } => {
                    concrete_options.push((*mid, m.name.clone()))
                }
                MaterialProperties::Steel { .. } => steel_options.push((*mid, m.name.clone())),
                _ => {}
            }
        }
        // Sort for consistent order
        concrete_options.sort_by(|a, b| a.1.cmp(&b.1));
        steel_options.sort_by(|a, b| a.1.cmp(&b.1));

        // Popup State Handling
        let create_popup_id = egui::Id::new("show_create_col_popup");
        let mut show_create = ui.data(|d| d.get_temp::<bool>(create_popup_id).unwrap_or(false));

        ui.horizontal(|ui| {
            if ui.button("➕ New Column").clicked() {
                show_create = true;
                ui.data_mut(|d| d.insert_temp(create_popup_id, true));

                // Default defaults
                let mut def_conc = 0;
                let mut def_steel = 0;

                if let Some(first) = concrete_options.first() {
                    def_conc = first.0;
                }
                if let Some(first) = steel_options.first() {
                    def_steel = first.0;
                }

                // Use same steel for both defaults initially
                let mut new_col =
                    ColumnType::new(0, "New Column", 30.0, 50.0, def_conc, def_steel, def_steel);

                ui.data_mut(|d| d.insert_temp(egui::Id::new("new_col_state"), new_col));
            }
        });
        ui.separator();

        // Main Content: Card List
        let mut col_ids: Vec<u64> = definitions.column_types.keys().cloned().collect();
        col_ids.sort();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for id in col_ids {
                if let Some(col) = definitions.column_types.get_mut(&id) {
                    let old_w = col.width;
                    let old_d = col.depth;

                    render_column_card(ui, col, &concrete_options, &steel_options);

                    if (col.width - old_w).abs() > 0.001 || (col.depth - old_d).abs() > 0.001 {
                        resize_events.push((id, old_w, old_d, col.width, col.depth));
                    }

                    ui.add_space(8.0);
                }
            }
        });

        // Handle Create Popup
        let mut should_close = false;
        let mut final_col = None;

        let closed = window::modal("Create New Column", ctx, &mut show_create, |ui| {
            let new_col_id = egui::Id::new("new_col_state");
            let mut new_col = ui
                .data(|d| d.get_temp::<ColumnType>(new_col_id))
                .unwrap_or_else(|| ColumnType::new(0, "Err", 30.0, 30.0, 0, 0, 0));

            ui.set_min_width(820.0); // Fits inside 850 window
            ui.heading("Define New Column");
            ui.separator();

            render_details_form(ui, &mut new_col, &concrete_options, &steel_options, true);

            ui.data_mut(|d| d.insert_temp(new_col_id, new_col.clone()));

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    final_col = Some(new_col);
                    return true;
                }
                if ui.button("Cancel").clicked() {
                    return true;
                }
                false
            })
            .inner
        });

        if closed {
            should_close = true;
        }

        if should_close {
            ui.data_mut(|d| d.insert_temp(create_popup_id, false));
            if let Some(col) = final_col {
                definitions.add_column_type(col);
            }
        }
    } // End of borrow scope for tab/definitions

    // Process Resizes
    if !resize_events.is_empty() {
        let tab = vm.active_tab_mut();
        for (id, old_w, old_d, new_w, new_d) in resize_events {
            apply_column_resize(&mut tab.model, id, old_w, old_d, new_w, new_d);
        }
    }
}

fn apply_column_resize(
    model: &mut crate::model::CadModel,
    type_id: u64,
    old_w: f32,
    old_d: f32,
    new_w: f32,
    new_d: f32,
) {
    // Iterate over all entities, find columns with this type, and update them
    use crate::model::Shape;
    use crate::model::structure::column::ColumnAnchor;

    for entity in model.entities.iter_mut() {
        if let Shape::Column(col) = &mut entity.shape {
            if col.column_type_id == type_id {
                // Update dimensions
                col.width = new_w;
                col.height = new_d; // col.height corresponds to depth in Type

                // Update Position based on Anchor
                // Delta size
                // We need to move the CENTER to keep the ANCHOR fixed.

                let (sin, cos) = col.rotation.sin_cos();
                let rotate =
                    |x: f32, y: f32| -> (f32, f32) { (x * cos - y * sin, x * sin + y * cos) };

                let get_local_anchor = |w: f32, h: f32, anchor: ColumnAnchor| -> (f32, f32) {
                    let hw = w / 2.0;
                    let hh = h / 2.0;
                    match anchor {
                        ColumnAnchor::Center => (0.0, 0.0),
                        ColumnAnchor::TopLeft => (-hw, -hh),
                        ColumnAnchor::TopRight => (hw, -hh),
                        ColumnAnchor::BottomRight => (hw, hh),
                        ColumnAnchor::BottomLeft => (-hw, hh),
                    }
                };

                let (ax_old, ay_old) = get_local_anchor(old_w, old_d, col.anchor);
                let (ax_new, ay_new) = get_local_anchor(new_w, new_d, col.anchor);

                // The local anchor point coordinates changed because width/height changed.
                // We want the GLOBAL position of the anchor to be the same.
                // Global_Pos = Center + Rotated(Local_Pos)
                // Center_Old + Rot(Local_Old) = Center_New + Rot(Local_New)
                // Center_New = Center_Old + Rot(Local_Old) - Rot(Local_New)
                // Center_New = Center_Old + Rot(Local_Old - Local_New)

                let diff_x = ax_old - ax_new;
                let diff_y = ay_old - ay_new;

                let (dx_rot, dy_rot) = rotate(diff_x, diff_y);

                col.center.x += dx_rot;
                col.center.y += dy_rot;
            }
        }
    }
}

/// Renders a single column type as a card with Preview (Left) and Details (Right)
fn render_column_card(
    ui: &mut egui::Ui,
    col: &mut ColumnType,
    concrete_options: &[(u64, String)],
    steel_options: &[(u64, String)],
) {
    let prev_col = col.clone();
    card::Card::new(
        move |ui| {
            draw_column_preview(ui, &prev_col);
        },
        |ui| {
            // Header (Name + ID)
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("#{}", col.id)).weak());
                ui.heading(&col.name);
            });
            ui.separator();

            // Detailed Form reused
            render_details_form(ui, col, concrete_options, steel_options, false);
        },
    )
    .show(ui);
}

/// Helper to draw the column preview
fn draw_column_preview(ui: &mut egui::Ui, col: &ColumnType) {
    // Fixed size for preview area
    let (response, painter) = ui.allocate_painter(egui::vec2(120.0, 120.0), egui::Sense::hover());

    // Background
    painter.rect_filled(response.rect, 4.0, egui::Color32::from_gray(30));

    // Draw Column Preview
    let rect = response.rect;
    let draw_rect = rect.shrink(2.0);

    let max_dim = col.width.max(col.depth).max(1.0);
    // Scale to fit draw_rect. Use 80% to leave room for rotation if we had it, but mostly padding.
    let scale_w = draw_rect.width() / max_dim;
    let scale_h = draw_rect.height() / max_dim;
    let scale = scale_w.min(scale_h) * 0.9;

    let center = draw_rect.center();

    // Use shared renderer
    crate::view::rendering::structure::draw_column(&painter, center, 0.0, scale, col, 1.0);
}

fn render_details_form(
    ui: &mut egui::Ui,
    col: &mut ColumnType,
    concrete_options: &[(u64, String)],
    steel_options: &[(u64, String)],
    is_popup: bool,
) {
    if is_popup {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut col.name);
        });
        ui.add_space(8.0);
    }

    // Grid: Left (Left Align) | Separator | Right (Right Align)
    egui::Grid::new(format!("col_grid_{}", col.id))
        .num_columns(3)
        .spacing([8.0, 8.0])
        .min_col_width(0.0) // We control width manually inside cells
        .show(ui, |ui| {
            // --- ROW 1: Geometry | Concrete ---
            render_grid_row(
                ui,
                |ui| {
                    ui.label(egui::RichText::new("Geometry:").strong());

                    if ui
                        .add(
                            egui::DragValue::new(&mut col.width)
                                .speed(1.0)
                                .suffix("cm")
                                .prefix("Width: "),
                        )
                        .changed()
                    {
                        // We rely on the parent checking for changes by comparing values before/after
                    }

                    if ui
                        .add(
                            egui::DragValue::new(&mut col.depth)
                                .speed(1.0)
                                .suffix("cm")
                                .prefix("Depth: "),
                        )
                        .changed()
                    {
                        // We rely on the parent checking for changes by comparing values before/after
                    }
                },
                |ui| {
                    let cur_conc = concrete_options
                        .iter()
                        .find(|(i, _)| *i == col.concrete_material_id)
                        .map(|(_, n)| n.as_str())
                        .unwrap_or("Select...");
                    egui::ComboBox::from_id_salt(format!("conc_{}", col.id))
                        .width(110.0)
                        .selected_text(cur_conc)
                        .show_ui(ui, |ui| {
                            for (mid, name) in concrete_options {
                                ui.selectable_value(&mut col.concrete_material_id, *mid, name);
                            }
                        });
                    ui.label(egui::RichText::new("Concrete Class:").strong());
                },
                true,
            );

            // --- ROW 2: Longitudinal ---
            render_grid_row(
                ui,
                |ui| {
                    ui.label(egui::RichText::new("Longitudinal:").strong());
                    ui.label("Material:");
                    let cur_long = steel_options
                        .iter()
                        .find(|(i, _)| *i == col.long_rebar_material_id)
                        .map(|(_, n)| n.as_str())
                        .unwrap_or("Select...");
                    egui::ComboBox::from_id_salt(format!("long_{}", col.id))
                        .width(100.0)
                        .selected_text(cur_long)
                        .show_ui(ui, |ui| {
                            for (mid, name) in steel_options {
                                ui.selectable_value(&mut col.long_rebar_material_id, *mid, name);
                            }
                        });
                },
                |ui| {
                    ui.add(
                        egui::DragValue::new(&mut col.long_bars_y)
                            .speed(1)
                            .prefix("Count Y: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut col.long_bars_x)
                            .speed(1)
                            .prefix("Count X: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut col.long_bar_diameter)
                            .speed(1.0)
                            .suffix("mm")
                            .prefix("Ø"),
                    );
                    ui.label(egui::RichText::new("Details:").strong());
                },
                true,
            );

            // --- ROW 3: Transverse (Ties) ---
            let has_ties = col.has_ties;
            render_grid_row(
                ui,
                |ui| {
                    ui.checkbox(
                        &mut col.has_ties,
                        egui::RichText::new("Transverse (Ties)").strong(),
                    );
                    if col.has_ties {
                        ui.label("Material:");
                        let cur_tie = steel_options
                            .iter()
                            .find(|(i, _)| *i == col.tie_material_id)
                            .map(|(_, n)| n.as_str())
                            .unwrap_or("Select...");
                        egui::ComboBox::from_id_salt(format!("tie_{}", col.id))
                            .width(100.0)
                            .selected_text(cur_tie)
                            .show_ui(ui, |ui| {
                                for (mid, name) in steel_options {
                                    ui.selectable_value(&mut col.tie_material_id, *mid, name);
                                }
                            });
                    }
                },
                |ui| {
                    if has_ties {
                        ui.add(
                            egui::DragValue::new(&mut col.tie_spacing_mid)
                                .speed(1.0)
                                .suffix("cm"),
                        );
                        ui.label("/");
                        ui.add(
                            egui::DragValue::new(&mut col.tie_spacing_supp)
                                .speed(1.0)
                                .suffix("cm"),
                        );
                        ui.label("Spacing:");
                        ui.add(
                            egui::DragValue::new(&mut col.tie_diameter)
                                .speed(1.0)
                                .suffix("mm")
                                .prefix("Ø"),
                        );
                        ui.label(egui::RichText::new("Details:").strong());
                    }
                },
                has_ties,
            );
        });
}

fn render_grid_row(
    ui: &mut egui::Ui,
    add_left: impl FnOnce(&mut egui::Ui),
    add_right: impl FnOnce(&mut egui::Ui),
    show_separator: bool,
) {
    ui.horizontal(|ui| {
        ui.set_min_width(360.0);
        add_left(ui);
    });

    if show_separator {
        ui.add(egui::Separator::default().vertical());
    } else {
        ui.label("");
    }

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        add_right(ui);
    });
    ui.end_row();
}
