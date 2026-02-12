use crate::model::structure::column_type::ColumnType;
use crate::model::structure::material::{Material, MaterialProperties};
use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::{properties, window};

pub fn render_structure_manager(ctx: &egui::Context, vm: &mut CadViewModel) {
    let mut open = vm.structure_manager_open;

    window::window(
        "Structure Manager",
        ctx,
        &mut open,
        [500.0, 400.0],
        true,
        |ui| {
            let tab_id = ui.make_persistent_id("structure_manager_tab");
            let mut active_tab = ui.data(|d| d.get_temp::<usize>(tab_id).unwrap_or(0));

            ui.horizontal(|ui| {
                if ui.selectable_label(active_tab == 0, "Materials").clicked() {
                    active_tab = 0;
                }
                if ui
                    .selectable_label(active_tab == 1, "Column Types")
                    .clicked()
                {
                    active_tab = 1;
                }
            });
            ui.separator();

            ui.data_mut(|d| d.insert_temp(tab_id, active_tab));

            if active_tab == 0 {
                render_materials_tab(ui, vm);
            } else {
                render_column_types_tab(ui, vm);
            }
        },
    );

    vm.structure_manager_open = open;
}

fn render_materials_tab(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let tab = vm.active_tab_mut();
    let definitions = &mut tab.model.definitions;

    // Tools
    ui.horizontal(|ui| {
        if ui.button("➕ Add Concrete").clicked() {
            definitions.add_material(Material::new_concrete(0, "New Concrete", "C25"));
        }
        if ui.button("➕ Add Steel").clicked() {
            definitions.add_material(Material::new_steel(0, "New Steel", "S420", Some(12.0)));
        }
    });
    ui.separator();

    let mut material_ids: Vec<u64> = definitions.materials.keys().cloned().collect();
    material_ids.sort();

    let mut remove_id = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        for id in material_ids {
            if let Some(material) = definitions.get_material_mut(id) {
                ui.push_id(id, |ui| {
                    ui.group(|ui| {
                        // Header: ID, Name, Delete, Color
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("#{}", id))
                                    .strong()
                                    .color(egui::Color32::GRAY),
                            );
                            ui.text_edit_singleline(&mut material.name);

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("🗑").on_hover_text("Delete").clicked() {
                                        remove_id = Some(id);
                                    }

                                    let mut color_arr = [
                                        material.color.0,
                                        material.color.1,
                                        material.color.2,
                                        material.color.3,
                                    ];
                                    if ui
                                        .color_edit_button_srgba_unmultiplied(&mut color_arr)
                                        .changed()
                                    {
                                        material.color = (
                                            color_arr[0],
                                            color_arr[1],
                                            color_arr[2],
                                            color_arr[3],
                                        );
                                    }
                                },
                            );
                        });

                        ui.indent("props", |ui| {
                            // Hatch Pattern
                            ui.horizontal(|ui| {
                                ui.label("Hatch Pattern:").on_hover_text(
                                    "e.g., ANSI31, SOLID, etc. used for section views",
                                );
                                ui.text_edit_singleline(&mut material.hatch_pattern);
                            });

                            // Type-specific properties
                            match &mut material.properties {
                                MaterialProperties::Concrete { class } => {
                                    ui.horizontal(|ui| {
                                        ui.label("Class:");
                                        egui::ComboBox::from_id_salt("concrete_class")
                                            .selected_text(class.as_str())
                                            .show_ui(ui, |ui| {
                                                let classes = [
                                                    "C16", "C20", "C25", "C30", "C35", "C40", "C50",
                                                ];
                                                for c in classes {
                                                    ui.selectable_value(class, c.to_string(), c);
                                                }
                                            });
                                    });
                                }
                                MaterialProperties::Steel { grade, diameter_mm } => {
                                    ui.horizontal(|ui| {
                                        ui.label("Grade:");
                                        egui::ComboBox::from_id_salt("steel_grade")
                                            .selected_text(grade.as_str())
                                            .show_ui(ui, |ui| {
                                                let grades = [
                                                    "S220", "S235", "S275", "S355", "S420", "S500",
                                                    "S600", "S700",
                                                ];
                                                for g in grades {
                                                    ui.selectable_value(grade, g.to_string(), g);
                                                }
                                            });
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Diameter:");
                                        let current_d = diameter_mm.unwrap_or(0.0);
                                        let mut selected_d = current_d;

                                        egui::ComboBox::from_id_salt("steel_diameter")
                                            .selected_text(format!("Ø {:.0}", current_d))
                                            .show_ui(ui, |ui| {
                                                let diameters = [
                                                    8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 22.0,
                                                    25.0, 28.0, 32.0,
                                                ];
                                                for d in diameters {
                                                    ui.selectable_value(
                                                        &mut selected_d,
                                                        d,
                                                        format!("Ø {:.0}", d),
                                                    );
                                                }
                                            });

                                        if (selected_d - current_d).abs() > 0.001 {
                                            *diameter_mm = Some(selected_d);
                                        }
                                    });
                                }
                                _ => {
                                    ui.label("Generic Material");
                                }
                            }
                        });
                    });
                });
                ui.add_space(4.0);
            }
        }
    });

    if let Some(id) = remove_id {
        definitions.remove_material(id);
    }
}

fn render_column_types_tab(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let tab = vm.active_tab_mut();

    // Separate materials into Concrete and Steel lists for dropdowns
    let mut concrete_options = Vec::new();
    let mut steel_options = Vec::new();

    for (id, m) in &tab.model.definitions.materials {
        match m.properties {
            MaterialProperties::Concrete { .. } => concrete_options.push((*id, m.name.clone())),
            MaterialProperties::Steel { .. } => steel_options.push((*id, m.name.clone())),
            _ => {}
        }
    }

    let definitions = &mut tab.model.definitions;

    // Tools
    ui.horizontal(|ui| {
        if ui.button("➕ Add Column Type").clicked() {
            if concrete_options.is_empty() || steel_options.is_empty() {
                // Warn if materials are missing (since we removed defaults)
                // We'll just add one with 0 IDs, user has to fix it or add materials.
            }
            let def_conc = concrete_options.first().map(|(id, _)| *id).unwrap_or(0);
            let def_steel = steel_options.first().map(|(id, _)| *id).unwrap_or(0);
            definitions.add_column_type(ColumnType::new(
                0,
                "New Column",
                30.0,
                50.0,
                def_conc,
                def_steel,
            ));
        }

        if concrete_options.is_empty() || steel_options.is_empty() {
            ui.label(
                egui::RichText::new("⚠ Please add Concrete and Steel materials first!")
                    .color(egui::Color32::RED),
            );
        }
    });
    ui.separator();

    let mut col_ids: Vec<u64> = definitions.column_types.keys().cloned().collect();
    col_ids.sort();

    let mut remove_id = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        for id in col_ids {
            if let Some(col) = definitions.get_column_type_mut(id) {
                ui.push_id(id, |ui| {
                    ui.group(|ui| {
                        // Header
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("#{}", id))
                                    .strong()
                                    .color(egui::Color32::GRAY),
                            );
                            ui.text_edit_singleline(&mut col.name);
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("🗑").on_hover_text("Delete").clicked() {
                                        remove_id = Some(id);
                                    }
                                },
                            );
                        });

                        ui.indent("col_details", |ui| {
                            egui::Grid::new("col_grid")
                                .num_columns(2)
                                .spacing([40.0, 8.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    // --- Section: Geometry ---
                                    ui.label(egui::RichText::new("Geometry").strong());
                                    ui.end_row();

                                    ui.label("Width (cm)");
                                    ui.add(
                                        egui::DragValue::new(&mut col.width)
                                            .speed(1.0)
                                            .suffix(" cm"),
                                    );
                                    ui.end_row();

                                    ui.label("Depth (cm)");
                                    ui.add(
                                        egui::DragValue::new(&mut col.depth)
                                            .speed(1.0)
                                            .suffix(" cm"),
                                    );
                                    ui.end_row();

                                    // --- Section: Materials ---
                                    ui.label(egui::RichText::new("Materials").strong());
                                    ui.end_row();

                                    ui.label("Concrete");
                                    let cur_conc = concrete_options
                                        .iter()
                                        .find(|(i, _)| *i == col.concrete_material_id)
                                        .map(|(_, n)| n.as_str())
                                        .unwrap_or("Select...");
                                    egui::ComboBox::from_id_salt(format!("conc_{}", id))
                                        .selected_text(cur_conc)
                                        .show_ui(ui, |ui| {
                                            for (mid, name) in &concrete_options {
                                                ui.selectable_value(
                                                    &mut col.concrete_material_id,
                                                    *mid,
                                                    name,
                                                );
                                            }
                                        });
                                    ui.end_row();

                                    ui.label("Rebar");
                                    let cur_steel = steel_options
                                        .iter()
                                        .find(|(i, _)| *i == col.rebar_material_id)
                                        .map(|(_, n)| n.as_str())
                                        .unwrap_or("Select...");
                                    egui::ComboBox::from_id_salt(format!("rebar_{}", id))
                                        .selected_text(cur_steel)
                                        .show_ui(ui, |ui| {
                                            for (mid, name) in &steel_options {
                                                ui.selectable_value(
                                                    &mut col.rebar_material_id,
                                                    *mid,
                                                    name,
                                                );
                                            }
                                        });
                                    ui.end_row();
                                });

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            // --- Section: Longitudinal Reinforcement ---
                            ui.label(egui::RichText::new("Longitudinal Reinforcement").strong());
                            egui::Grid::new("long_rebar_grid")
                                .num_columns(2)
                                .spacing([40.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label("Diameter");
                                    ui.add(
                                        egui::DragValue::new(&mut col.long_bar_diameter)
                                            .speed(1.0)
                                            .suffix(" mm"),
                                    );
                                    ui.end_row();

                                    ui.label("Count (X-Face)");
                                    ui.add(egui::DragValue::new(&mut col.long_bars_x).speed(1));
                                    ui.end_row();

                                    ui.label("Count (Y-Face)");
                                    ui.add(egui::DragValue::new(&mut col.long_bars_y).speed(1));
                                    ui.end_row();
                                });

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            // --- Section: Transverse Reinforcement (Stirrups) ---
                            ui.label(egui::RichText::new("Transverse Reinforcement").strong());
                            egui::Grid::new("stirrup_grid")
                                .num_columns(2)
                                .spacing([40.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label("Stirrup Diameter");
                                    ui.add(
                                        egui::DragValue::new(&mut col.stirrup_diameter)
                                            .speed(1.0)
                                            .suffix(" mm"),
                                    );
                                    ui.end_row();

                                    ui.label("Support Spacing"); // Sıklaştırma
                                    ui.add(
                                        egui::DragValue::new(&mut col.stirrup_spacing_supp)
                                            .speed(1.0)
                                            .suffix(" cm"),
                                    );
                                    ui.end_row();

                                    ui.label("Middle Spacing"); // Orta
                                    ui.add(
                                        egui::DragValue::new(&mut col.stirrup_spacing_mid)
                                            .speed(1.0)
                                            .suffix(" cm"),
                                    );
                                    ui.end_row();

                                    ui.label("Ties (Çiroz)");
                                    ui.checkbox(&mut col.has_ties, "Enabled");
                                    ui.end_row();
                                });
                        });
                    });
                });
                ui.add_space(4.0);
            }
        }
    });

    if let Some(id) = remove_id {
        definitions.remove_column_type(id);
    }
}
