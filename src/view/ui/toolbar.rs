use crate::viewmodel::CadViewModel;
use eframe::egui;

/// Render the left toolbar with icon buttons for commands (2 columns)
pub fn render_toolbar(ctx: &egui::Context, vm: &mut CadViewModel) {
    egui::SidePanel::left("toolbar")
        .resizable(false)
        .default_width(88.0) // Wider for 2 columns
        .frame(
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(35, 35, 35))
                .inner_margin(4.0),
        )
        .show(ctx, |ui| {
            if vm.tabs.is_empty() {
                ui.label("No Project");
                return;
            }

            let tab = vm.active_tab_mut();
            let has_selection = !tab.selection_manager.selected_indices.is_empty();
            let btn_size = egui::vec2(36.0, 36.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // ===== TRANSFORM SECTION =====
                ui.label(
                    egui::RichText::new("Transform")
                        .small()
                        .color(egui::Color32::from_gray(150)),
                );
                ui.columns(2, |cols| {
                    // Column 1
                    if cols[0]
                        .add_enabled(has_selection, egui::Button::new("M").min_size(btn_size))
                        .on_hover_text("Move (W)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "move",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[0]
                        .add_enabled(has_selection, egui::Button::new("S").min_size(btn_size))
                        .on_hover_text("Scale (R)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "scale",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[0]
                        .add_enabled(has_selection, egui::Button::new("Cp").min_size(btn_size))
                        .on_hover_text("Copy (Ctrl+C)")
                        .clicked()
                    {
                        let indices = tab.selection_manager.selected_indices.clone();
                        crate::commands::manipulate::copy::copy_entities(&mut tab.model, &indices);
                    }
                    if cols[0]
                        .add_enabled(has_selection, egui::Button::new("E").min_size(btn_size))
                        .on_hover_text("Erase selected")
                        .clicked()
                    {
                        let indices = tab.selection_manager.selected_indices.clone();
                        for idx in indices.iter().rev() {
                            tab.model.remove_entity(*idx);
                        }
                        tab.selection_manager.selected_indices.clear();
                    }

                    // Column 2
                    if cols[1]
                        .add_enabled(has_selection, egui::Button::new("R").min_size(btn_size))
                        .on_hover_text("Rotate (E)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "rotate",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[1]
                        .add_enabled(has_selection, egui::Button::new("Mi").min_size(btn_size))
                        .on_hover_text("Mirror")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "mirror",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[1]
                        .add_enabled(has_selection, egui::Button::new("Of").min_size(btn_size))
                        .on_hover_text("Offset")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "offset",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[1]
                        .add_enabled(has_selection, egui::Button::new("Tr").min_size(btn_size))
                        .on_hover_text("Trim")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "trim",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                });

                ui.separator();

                // ===== CREATE SECTION =====
                ui.label(
                    egui::RichText::new("Create")
                        .small()
                        .color(egui::Color32::from_gray(150)),
                );
                ui.columns(2, |cols| {
                    if cols[0]
                        .add(egui::Button::new("L").min_size(btn_size))
                        .on_hover_text("Line (L)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "line",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[0]
                        .add(egui::Button::new("◯").min_size(btn_size))
                        .on_hover_text("Circle (C)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "circle",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[0]
                        .add(egui::Button::new("⌒").min_size(btn_size))
                        .on_hover_text("Arc (A)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "arc",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }

                    if cols[1]
                        .add(egui::Button::new("□").min_size(btn_size))
                        .on_hover_text("Rectangle (R)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "rect",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[1]
                        .add(egui::Button::new("⬡").min_size(btn_size))
                        .on_hover_text("Polygon (P)")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "polygon",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[1]
                        .add(egui::Button::new("T").min_size(btn_size))
                        .on_hover_text("Text")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "text",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                });

                ui.separator();

                // ===== ANNOTATE SECTION =====
                ui.label(
                    egui::RichText::new("Annotate")
                        .small()
                        .color(egui::Color32::from_gray(150)),
                );
                ui.columns(2, |cols| {
                    if cols[0]
                        .add(egui::Button::new("Dst").min_size(btn_size))
                        .on_hover_text("Distance")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "distance",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[0]
                        .add(egui::Button::new("Ar").min_size(btn_size))
                        .on_hover_text("Area")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "area",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }

                    if cols[1]
                        .add(egui::Button::new("Dim").min_size(btn_size))
                        .on_hover_text("Dimension")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "measure",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[1]
                        .add(egui::Button::new("Per").min_size(btn_size))
                        .on_hover_text("Perimeter")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "perim",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                });

                ui.separator();

                // ===== STRUCTURE SECTION =====
                ui.label(
                    egui::RichText::new("Structure")
                        .small()
                        .color(egui::Color32::from_rgb(100, 150, 200)),
                );
                ui.columns(2, |cols| {
                    let struct_btn = |text: &str| {
                        egui::Button::new(egui::RichText::new(text).strong())
                            .min_size(btn_size)
                            .fill(egui::Color32::from_rgb(50, 60, 80))
                    };

                    if cols[0]
                        .add(struct_btn("C"))
                        .on_hover_text("Column")
                        .clicked()
                    {
                        tab.executor.start_command(
                            "column",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }
                    if cols[0].add(struct_btn("B")).on_hover_text("Beam").clicked() {
                        tab.executor.start_command(
                            "beam",
                            &mut tab.model,
                            &tab.selection_manager.selected_indices,
                        );
                    }

                    // Future items (disabled)
                    cols[1]
                        .add_enabled(false, struct_btn("F"))
                        .on_hover_text("Flooring (Coming soon)");
                    cols[1]
                        .add_enabled(false, struct_btn("D"))
                        .on_hover_text("Door (Coming soon)");
                });
            });
        });
}

/// Render horizontal bar showing available structural types when a structural command is active
pub fn render_structure_type_bar(ctx: &egui::Context, vm: &mut CadViewModel) {
    // First check if we need to show the bar at all
    let show_bar = if !vm.tabs.is_empty() {
        let tab = &vm.tabs[vm.active_tab_index];
        tab.executor.is_active()
            && (tab.executor.status_message.contains("COLUMN")
                || tab.executor.status_message.contains("BEAM"))
    } else {
        false
    };

    if !show_bar {
        return;
    }

    // Collect data before UI rendering
    let (active_type, dims) = {
        let tab = &vm.tabs[vm.active_tab_index];
        let is_column = tab.executor.status_message.contains("COLUMN");

        if is_column {
            let (w, d) = tab.model.structural_types.get_active_column_dimensions();
            (
                tab.model.structural_types.get_active_column_type(),
                format!("{}x{}", w, d),
            )
        } else {
            let (w, h) = tab.model.structural_types.get_active_beam_dimensions();
            (
                tab.model.structural_types.get_active_beam_type(),
                format!("{}x{}", w, h),
            )
        }
    };

    let mut open_type_manager = false;

    egui::TopBottomPanel::top("structure_type_bar")
        .frame(
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(45, 50, 60))
                .inner_margin(egui::Margin::symmetric(8.0, 4.0)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Active Type:").color(egui::Color32::from_gray(180)));
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(format!("{} ({})", active_type, dims))
                        .color(egui::Color32::LIGHT_GREEN)
                        .strong(),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                if ui.button("📐 Type Manager").clicked() {
                    open_type_manager = true;
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                ui.label(egui::RichText::new("Orientation:").color(egui::Color32::from_gray(180)));
                ui.add_space(8.0);

                if ui
                    .selectable_label(false, "H")
                    .on_hover_text("Horizontal")
                    .clicked()
                {
                    // TODO: Set horizontal
                }
                if ui
                    .selectable_label(false, "V")
                    .on_hover_text("Vertical")
                    .clicked()
                {
                    // TODO: Set vertical
                }
            });
        });

    if open_type_manager {
        vm.structural_type_window.open = true;
    }
}
