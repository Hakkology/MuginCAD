use crate::viewmodel::CadViewModel;
use eframe::egui;

/// Render the top menu bar
pub fn render_top_menu(ctx: &egui::Context, vm: &mut CadViewModel) {
    egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
        // Add some padding
        ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 5.0);

        egui::menu::bar(ui, |ui| {
            ui.menu_button("Project", |ui| {
                ui.set_min_width(120.0);
                if ui.button("New").clicked() {
                    vm.new_tab();
                    ui.close_menu();
                }
                if ui.button("Save").clicked() {
                    vm.save_project();
                    ui.close_menu();
                }
                if ui.button("Load").clicked() {
                    vm.load_project();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Export PDF...").clicked() {
                    vm.export_window.open = true;
                    ui.close_menu();
                }
                if ui.button("Select Export Region").clicked() {
                    let tab = vm.active_tab_mut();
                    tab.executor.start_command(
                        "select_region",
                        &mut tab.model,
                        &std::collections::HashSet::new(),
                    );
                    ui.close_menu();
                }
            });

            ui.menu_button("Actions", |ui| {
                ui.set_min_width(140.0);

                if vm.tabs.is_empty() {
                    ui.label("No Project Open");
                    return;
                }

                let tab = vm.active_tab_mut();

                // Shapes section
                ui.label("Shapes");
                ui.separator();
                if ui.button("Line (L)").clicked() {
                    tab.executor.start_command(
                        "line",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui.button("Circle (C)").clicked() {
                    tab.executor.start_command(
                        "circle",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui.button("Rectangle").clicked() {
                    tab.executor.start_command(
                        "rect",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui.button("Arc").clicked() {
                    tab.executor.start_command(
                        "arc",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Transform section
                ui.label("Transform");
                ui.separator();
                let has_selection = !tab.selection_manager.selected_indices.is_empty();
                if ui
                    .add_enabled(has_selection, egui::Button::new("Move (W)"))
                    .clicked()
                {
                    tab.executor.start_command(
                        "move",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_selection, egui::Button::new("Rotate (E)"))
                    .clicked()
                {
                    tab.executor.start_command(
                        "rotate",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_selection, egui::Button::new("Scale (R)"))
                    .clicked()
                {
                    tab.executor.start_command(
                        "scale",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Clipboard section
                ui.label("Clipboard");
                ui.separator();
                if ui
                    .add_enabled(has_selection, egui::Button::new("Copy (Ctrl+C)"))
                    .clicked()
                {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("copy", &mut tab.model, &indices);
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_selection, egui::Button::new("Cut (Ctrl+X)"))
                    .clicked()
                {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("cut", &mut tab.model, &indices);
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Construction section
                ui.label("Construction");
                ui.separator();
                if ui.button("Axis (A)").clicked() {
                    tab.executor.start_command(
                        "axis",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui.button("Trim (T)").clicked() {
                    tab.executor.start_command(
                        "trim",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_selection, egui::Button::new("Offset (O)"))
                    .clicked()
                {
                    tab.executor.start_command(
                        "offset",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Annotation section
                ui.label("Annotation");
                ui.separator();
                if ui.button("Text").clicked() {
                    tab.executor.start_command(
                        "text",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui.button("Distance").clicked() {
                    tab.executor.start_command(
                        "distance",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
            });

            ui.menu_button("Structure", |ui| {
                ui.set_min_width(140.0);

                if vm.tabs.is_empty() {
                    ui.label("No Project Open");
                    return;
                }

                let tab = vm.active_tab_mut();

                // Structural elements section
                ui.label("Elements");
                ui.separator();
                if ui.button("Column (Col)").clicked() {
                    tab.executor.start_command(
                        "column",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }
                if ui.button("Beam (B)").clicked() {
                    tab.executor.start_command(
                        "beam",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Future elements (placeholder)
                ui.label("Openings");
                ui.separator();
                ui.add_enabled(false, egui::Button::new("Door"))
                    .on_hover_text("Coming soon");
                ui.add_enabled(false, egui::Button::new("Window"))
                    .on_hover_text("Coming soon");

                ui.add_space(8.0);

                ui.label("Slabs");
                ui.separator();
                ui.add_enabled(false, egui::Button::new("Flooring"))
                    .on_hover_text("Coming soon");
            });

            ui.menu_button("Tools", |ui| {
                ui.set_min_width(120.0);
                if ui.button("Settings").clicked() {
                    vm.show_settings_window = true;
                    ui.close_menu();
                }
            });
        });
    });
}
