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
                    vm.new_project();
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
            });

            ui.menu_button("Actions", |ui| {
                ui.set_min_width(140.0);

                // Shapes section
                ui.label("Shapes");
                ui.separator();
                if ui.button("Line (L)").clicked() {
                    vm.executor
                        .start_command("line", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui.button("Circle (C)").clicked() {
                    vm.executor
                        .start_command("circle", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui.button("Rectangle").clicked() {
                    vm.executor
                        .start_command("rect", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui.button("Arc").clicked() {
                    vm.executor
                        .start_command("arc", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Transform section
                ui.label("Transform");
                ui.separator();
                let has_selection = !vm.selected_indices.is_empty();
                if ui
                    .add_enabled(has_selection, egui::Button::new("Move (W)"))
                    .clicked()
                {
                    vm.executor
                        .start_command("move", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_selection, egui::Button::new("Rotate (E)"))
                    .clicked()
                {
                    vm.executor
                        .start_command("rotate", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_selection, egui::Button::new("Scale (R)"))
                    .clicked()
                {
                    vm.executor
                        .start_command("scale", &mut vm.model, &vm.selected_indices);
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
                    vm.executor
                        .start_command("copy", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui
                    .add_enabled(has_selection, egui::Button::new("Cut (Ctrl+X)"))
                    .clicked()
                {
                    vm.executor
                        .start_command("cut", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Construction section
                ui.label("Construction");
                ui.separator();
                if ui.button("Axis (A)").clicked() {
                    vm.executor
                        .start_command("axis", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui.button("Trim (T)").clicked() {
                    vm.executor
                        .start_command("trim", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }

                ui.add_space(8.0);

                // Annotation section
                ui.label("Annotation");
                ui.separator();
                if ui.button("Text").clicked() {
                    vm.executor
                        .start_command("text", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
                if ui.button("Distance").clicked() {
                    vm.executor
                        .start_command("distance", &mut vm.model, &vm.selected_indices);
                    ui.close_menu();
                }
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
