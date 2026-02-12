use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::toolbar;

/// Render the top menu bar
pub fn render_top_menu(ctx: &egui::Context, vm: &mut CadViewModel) {
    egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
        ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 5.0);

        egui::menu::bar(ui, |ui| {
            // ── Project Menu ─────────────────────────────────
            ui.menu_button("Project", |ui| {
                ui.set_min_width(120.0);

                if toolbar::menu_action(ui, "New") {
                    vm.new_tab();
                }
                if toolbar::menu_action(ui, "Save") {
                    vm.save_project();
                }
                if toolbar::menu_action(ui, "Load") {
                    vm.load_project();
                }

                ui.separator();

                if toolbar::menu_action(ui, "Export PDF...") {
                    vm.export_window.open = true;
                }
                if toolbar::menu_action(ui, "Select Export Region") {
                    let tab = vm.active_tab_mut();
                    tab.executor.start_command(
                        "select_region",
                        &mut tab.model,
                        &std::collections::HashSet::new(),
                    );
                }
            });

            // ── Actions Menu ─────────────────────────────────
            ui.menu_button("Actions", |ui| {
                ui.set_min_width(140.0);

                if vm.tabs.is_empty() {
                    ui.label("No Project Open");
                    return;
                }

                let tab = vm.active_tab_mut();
                let has_sel = !tab.selection_manager.selected_indices.is_empty();

                // Shapes
                toolbar::menu_section(ui, "Shapes");
                if toolbar::menu_action(ui, "Line (L)") {
                    tab.executor.start_command(
                        "line",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_action(ui, "Circle (C)") {
                    tab.executor.start_command(
                        "circle",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_action(ui, "Rectangle") {
                    tab.executor.start_command(
                        "rect",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_action(ui, "Arc") {
                    tab.executor.start_command(
                        "arc",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                // Transform
                toolbar::menu_section(ui, "Transform");
                if toolbar::menu_item(ui, "Move (W)", has_sel) {
                    tab.executor.start_command(
                        "move",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_item(ui, "Rotate (E)", has_sel) {
                    tab.executor.start_command(
                        "rotate",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_item(ui, "Scale (R)", has_sel) {
                    tab.executor.start_command(
                        "scale",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                // Clipboard
                toolbar::menu_section(ui, "Clipboard");
                if toolbar::menu_item(ui, "Copy (Ctrl+C)", has_sel) {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("copy", &mut tab.model, &indices);
                }
                if toolbar::menu_item(ui, "Cut (Ctrl+X)", has_sel) {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("cut", &mut tab.model, &indices);
                }

                // Construction
                toolbar::menu_section(ui, "Construction");
                if toolbar::menu_action(ui, "Axis (A)") {
                    tab.executor.start_command(
                        "axis",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_action(ui, "Trim (T)") {
                    tab.executor.start_command(
                        "trim",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_item(ui, "Offset (O)", has_sel) {
                    tab.executor.start_command(
                        "offset",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                // Annotation
                toolbar::menu_section(ui, "Annotation");
                if toolbar::menu_action(ui, "Text") {
                    tab.executor.start_command(
                        "text",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::menu_action(ui, "Distance") {
                    tab.executor.start_command(
                        "distance",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
            });

            // ── Tools Menu ───────────────────────────────────
            ui.menu_button("Tools", |ui| {
                ui.set_min_width(120.0);
                if toolbar::menu_action(ui, "Settings") {
                    vm.show_settings_window = true;
                }
            });
        });
    });
}
