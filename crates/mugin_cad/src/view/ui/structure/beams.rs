use crate::model::structure::beam_type::{BeamRebarZone, BeamType};
use crate::model::structure::material::MaterialProperties;
use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::{card, window};

pub fn render_beam_manager(ctx: &egui::Context, vm: &mut CadViewModel) {
    let mut open = vm.beam_manager_open;

    window::window("Beam Types", ctx, &mut open, [850.0, 600.0], true, |ui| {
        render_beam_ui(ctx, ui, vm);
    });

    vm.beam_manager_open = open;
}

fn render_beam_ui(ctx: &egui::Context, ui: &mut egui::Ui, vm: &mut CadViewModel) {
    {
        let tab = vm.active_tab_mut();
        let definitions = &mut tab.model.definitions;

        // Collect material options
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
        concrete_options.sort_by(|a, b| a.1.cmp(&b.1));
        steel_options.sort_by(|a, b| a.1.cmp(&b.1));

        // Popup State Handling
        let create_popup_id = egui::Id::new("show_create_beam_popup");
        let mut show_create = ui.data(|d| d.get_temp::<bool>(create_popup_id).unwrap_or(false));

        ui.horizontal(|ui| {
            if ui.button("➕ New Beam").clicked() {
                show_create = true;
                ui.data_mut(|d| d.insert_temp(create_popup_id, true));

                // Default materials
                let def_conc = concrete_options.first().map(|o| o.0).unwrap_or(0);
                let def_steel = steel_options.first().map(|o| o.0).unwrap_or(0);

                let new_beam = BeamType::new(0, "New Beam", 25.0, 50.0, def_conc, def_steel);
                ui.data_mut(|d| d.insert_temp(egui::Id::new("new_beam_state"), new_beam));
            }
        });
        ui.separator();

        // Main Content: Card List
        let mut beam_ids: Vec<u64> = definitions.beam_types.keys().cloned().collect();
        beam_ids.sort();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for id in beam_ids {
                if let Some(beam) = definitions.beam_types.get_mut(&id) {
                    render_beam_card(ui, beam, &concrete_options, &steel_options);
                    ui.add_space(8.0);
                }
            }
        });

        // Handle Create Popup
        let mut should_close = false;
        let mut final_beam = None;

        let closed = window::modal("Create New Beam", ctx, &mut show_create, |ui| {
            let new_beam_id = egui::Id::new("new_beam_state");
            let mut new_beam = ui
                .data(|d| d.get_temp::<BeamType>(new_beam_id))
                .unwrap_or_else(|| BeamType::new(0, "Err", 25.0, 50.0, 0, 0));

            ui.set_min_width(820.0);
            ui.heading("Define New Beam");
            ui.separator();

            render_beam_details_form(ui, &mut new_beam, &concrete_options, &steel_options, true);

            ui.data_mut(|d| d.insert_temp(new_beam_id, new_beam.clone()));

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    final_beam = Some(new_beam);
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
            if let Some(beam) = final_beam {
                definitions.add_beam_type(beam);
            }
        }
    }
}

fn render_beam_card(
    ui: &mut egui::Ui,
    beam: &mut BeamType,
    concrete_options: &[(u64, String)],
    steel_options: &[(u64, String)],
) {
    let prev_beam = beam.clone();
    card::Card::new(
        move |ui| {
            draw_beam_preview_ui(ui, &prev_beam);
        },
        |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("#{}", beam.id)).weak());
                ui.heading(&beam.name);
            });
            ui.separator();
            render_beam_details_form(ui, beam, concrete_options, steel_options, false);
        },
    )
    .show(ui);
}

fn draw_beam_preview_ui(ui: &mut egui::Ui, beam: &BeamType) {
    let (response, painter) = ui.allocate_painter(egui::vec2(120.0, 120.0), egui::Sense::hover());
    painter.rect_filled(response.rect, 4.0, egui::Color32::from_gray(30));

    // Basic placeholder preview for now
    let center = response.rect.center();
    let max_dim = beam.width.max(beam.height).max(1.0);
    let scale = (response.rect.width() * 0.8) / max_dim;

    let w = beam.width * scale;
    let h = beam.height * scale;

    painter.rect_stroke(
        egui::Rect::from_center_size(center, egui::vec2(w, h)),
        1.0,
        egui::Stroke::new(1.5, egui::Color32::from_rgb(150, 150, 160)),
    );
}

fn render_beam_details_form(
    ui: &mut egui::Ui,
    beam: &mut BeamType,
    concrete_options: &[(u64, String)],
    steel_options: &[(u64, String)],
    is_popup: bool,
) {
    if is_popup {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut beam.name);
        });
        ui.add_space(8.0);
    }

    egui::Grid::new(format!("beam_grid_{}", beam.id))
        .num_columns(3)
        .spacing([8.0, 8.0])
        .show(ui, |ui| {
            // ROW 1: Geometry & Materials
            render_grid_row(
                ui,
                |ui| {
                    ui.label(egui::RichText::new("Geometry:").strong());
                    ui.add(
                        egui::DragValue::new(&mut beam.width)
                            .suffix("cm")
                            .prefix("W: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut beam.height)
                            .suffix("cm")
                            .prefix("H: "),
                    );
                },
                |ui| {
                    let cur_conc = concrete_options
                        .iter()
                        .find(|(i, _)| *i == beam.concrete_material_id)
                        .map(|(_, n)| n.as_str())
                        .unwrap_or("Select...");
                    egui::ComboBox::from_id_salt(format!("conc_{}", beam.id))
                        .width(110.0)
                        .selected_text(cur_conc)
                        .show_ui(ui, |ui| {
                            for (mid, name) in concrete_options {
                                ui.selectable_value(&mut beam.concrete_material_id, *mid, name);
                            }
                        });
                    ui.label(egui::RichText::new("Concrete:").strong());
                },
                true,
            );

            // ROW 2: Longitudinal
            render_grid_row(
                ui,
                |ui| {
                    ui.label(egui::RichText::new("Longitudinal:").strong());
                    ui.label("Steel:");
                    let cur_steel = steel_options
                        .iter()
                        .find(|(i, _)| *i == beam.steel_material_id)
                        .map(|(_, n)| n.as_str())
                        .unwrap_or("Select...");
                    egui::ComboBox::from_id_salt(format!("steel_{}", beam.id))
                        .width(100.0)
                        .selected_text(cur_steel)
                        .show_ui(ui, |ui| {
                            for (mid, name) in steel_options {
                                ui.selectable_value(&mut beam.steel_material_id, *mid, name);
                            }
                        });
                },
                |ui| {
                    ui.add(egui::DragValue::new(&mut beam.top_bar_count).prefix("Top: "));
                    ui.add(
                        egui::DragValue::new(&mut beam.top_bar_diameter)
                            .suffix("mm")
                            .prefix("Ø"),
                    );
                    ui.label("/");
                    ui.add(egui::DragValue::new(&mut beam.bottom_bar_count).prefix("Bot: "));
                    ui.add(
                        egui::DragValue::new(&mut beam.bottom_bar_diameter)
                            .suffix("mm")
                            .prefix("Ø"),
                    );
                    ui.label(egui::RichText::new("Bars:").strong());
                },
                true,
            );

            // ROW 3: Side Bars & Zones
            render_grid_row(
                ui,
                |ui| {
                    ui.label(egui::RichText::new("Side Bars:").strong());
                    ui.add(
                        egui::DragValue::new(&mut beam.side_bar_count).prefix("Count (per side): "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut beam.side_bar_diameter)
                            .suffix("mm")
                            .prefix("Ø"),
                    );
                },
                |ui| {
                    ui.add(
                        egui::DragValue::new(&mut beam.support_zone_ratio)
                            .speed(0.01)
                            .range(0.0..=0.5)
                            .prefix("Ratio: "),
                    );
                    ui.label(egui::RichText::new("L/x Support:").strong());
                },
                true,
            );

            // ROW 4: Transverse (Ties) - Three Zones
            render_grid_row(
                ui,
                |ui| {
                    ui.label(egui::RichText::new("Ties (A/B/C):").strong());
                    render_zone_editor(ui, &mut beam.zone_left, "Left");
                    render_zone_editor(ui, &mut beam.zone_mid, "Mid");
                    render_zone_editor(ui, &mut beam.zone_right, "Right");
                },
                |_ui| {
                    // Empty right, combined in left for zone layout
                },
                false,
            );
        });
}

fn render_zone_editor(ui: &mut egui::Ui, zone: &mut BeamRebarZone, label: &str) {
    ui.group(|ui| {
        ui.label(label);
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut zone.tie_diameter)
                    .suffix("mm")
                    .prefix("Ø"),
            );
            ui.add(
                egui::DragValue::new(&mut zone.tie_spacing)
                    .suffix("cm")
                    .prefix("s:"),
            );
        });
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
