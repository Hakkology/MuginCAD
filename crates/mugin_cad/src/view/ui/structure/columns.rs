use crate::model::structure::column_type::ColumnType;
use crate::model::structure::material::MaterialProperties;
use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::{properties, window};

pub fn render_column_manager(ctx: &egui::Context, vm: &mut CadViewModel) {
    let mut open = vm.column_manager_open;

    // Reduced width to 850.0 as requested
    window::window("Column Types", ctx, &mut open, [850.0, 600.0], true, |ui| {
        render_column_ui(ctx, ui, vm);
    });

    vm.column_manager_open = open;
}

fn render_column_ui(ctx: &egui::Context, ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let tab = vm.active_tab_mut();
    let definitions = &mut tab.model.definitions;

    // Collect material options once for the frame
    let mut concrete_options = Vec::new();
    let mut steel_options = Vec::new();
    for (mid, m) in &definitions.materials {
        match m.properties {
            MaterialProperties::Concrete { .. } => concrete_options.push((*mid, m.name.clone())),
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
    // We iterate over keys to avoid borrowing issues
    let mut col_ids: Vec<u64> = definitions.column_types.keys().cloned().collect();
    col_ids.sort();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for id in col_ids {
            if let Some(col) = definitions.column_types.get_mut(&id) {
                render_column_card(ui, col, &concrete_options, &steel_options);
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
}

/// Renders a single column type as a card with Preview (Left) and Details (Right)
fn render_column_card(
    ui: &mut egui::Ui,
    col: &mut ColumnType,
    concrete_options: &[(u64, String)],
    steel_options: &[(u64, String)],
) {
    egui::Frame::group(ui.style())
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // LEFT: Preview
                // Fixed size for preview area
                let (response, painter) =
                    ui.allocate_painter(egui::vec2(80.0, 120.0), egui::Sense::hover());

                // Background
                painter.rect_filled(response.rect, 4.0, egui::Color32::from_gray(30));

                // Draw Column Preview
                let rect = response.rect;
                // Pad a bit
                let draw_rect = rect.shrink(8.0);

                let max_dim = col.width.max(col.depth).max(1.0);
                // Scale to fit draw_rect
                let scale_w = draw_rect.width() / max_dim;
                let scale_h = draw_rect.height() / max_dim;
                let scale = scale_w.min(scale_h);

                let w = col.width * scale;
                let h = col.depth * scale;
                let center = draw_rect.center();
                let col_rect = egui::Rect::from_center_size(center, egui::vec2(w, h));

                painter.rect_filled(col_rect, 0.0, egui::Color32::from_rgb(100, 100, 120));
                painter.rect_stroke(col_rect, 0.0, egui::Stroke::new(2.0, egui::Color32::WHITE));

                // Rebar dots (corners)
                let bar_r = 2.0;
                let inset = 4.0;
                let min = col_rect.min;
                let max = col_rect.max;
                painter.circle_filled(min + egui::vec2(inset, inset), bar_r, egui::Color32::RED);
                painter.circle_filled(max - egui::vec2(inset, inset), bar_r, egui::Color32::RED);
                painter.circle_filled(
                    egui::pos2(min.x + inset, max.y - inset),
                    bar_r,
                    egui::Color32::RED,
                );
                painter.circle_filled(
                    egui::pos2(max.x - inset, min.y + inset),
                    bar_r,
                    egui::Color32::RED,
                );

                // Ties visual (if enabled)
                if col.has_ties {
                    painter.rect_stroke(
                        col_rect.shrink(inset),
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::YELLOW),
                    );
                }

                ui.add_space(12.0);

                // RIGHT: Details
                ui.vertical(|ui| {
                    // Header (Name + ID)
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("#{}", col.id)).weak());
                        ui.heading(&col.name);
                    });
                    ui.separator();

                    // Detailed Form reused
                    render_details_form(ui, col, concrete_options, steel_options, false);
                });
            });
        });
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

            // Col 1: Geometry (Left) - Centered Separator approximation
            ui.horizontal(|ui| {
                ui.set_min_width(360.0);

                ui.label(egui::RichText::new("Geometry:").strong());
                ui.add(
                    egui::DragValue::new(&mut col.width)
                        .speed(1.0)
                        .suffix("cm")
                        .prefix("Width: "),
                );
                ui.add(
                    egui::DragValue::new(&mut col.depth)
                        .speed(1.0)
                        .suffix("cm")
                        .prefix("Depth: "),
                );
            });

            // Col 2: Separator
            ui.add(egui::Separator::default().vertical());

            // Col 3: Concrete (Right)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
            });
            ui.end_row();

            // --- ROW 2: Longitudinal ---

            // Col 1: Label + Material (Left)
            ui.horizontal(|ui| {
                ui.set_min_width(360.0);

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
            });

            // Col 2: Separator
            ui.add(egui::Separator::default().vertical());

            // Col 3: Details (Right)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
            });
            ui.end_row();

            // --- ROW 3: Transverse (Ties) ---

            // Col 1: Checkbox + Material (Left)
            ui.horizontal(|ui| {
                ui.set_min_width(360.0);

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
            });

            // Col 2: Separator
            if col.has_ties {
                ui.add(egui::Separator::default().vertical());
            } else {
                ui.label("");
            }

            // Col 3: Spacing (Right)
            if col.has_ties {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                });
            } else {
                ui.label("");
            }
            ui.end_row();
        });
}
