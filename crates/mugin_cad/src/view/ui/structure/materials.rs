use crate::model::structure::material::{Material, MaterialProperties};
use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::window;

pub fn render_materials_manager(ctx: &egui::Context, vm: &mut CadViewModel) {
    let mut open = vm.materials_manager_open;

    window::window(
        "Materials Manager",
        ctx,
        &mut open,
        [500.0, 400.0],
        true,
        |ui| {
            render_materials_tab(ui, vm);
        },
    );

    vm.materials_manager_open = open;
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
