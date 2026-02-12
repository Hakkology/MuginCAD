use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_terminal(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.style_mut().visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 200));

        // 1. Settings bar (Bottom-most)
        ui.horizontal(|ui| {
            ui.label("Settings:");
            ui.checkbox(&mut vm.config.snap_config.snap_to_grid, "Snap to Grid");

            if vm.config.snap_config.snap_to_grid {
                ui.label("Grid Size:");
                ui.add(
                    egui::DragValue::new(&mut vm.config.grid_config.grid_size)
                        .speed(1.0)
                        .range(1.0..=1000.0),
                );
            }
        });

        ui.separator();

        // 2. Input Bar
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(vm.status_message())
                    .strong()
                    .color(egui::Color32::LIGHT_BLUE),
            );

            let text_edit = egui::TextEdit::singleline(&mut vm.command_input)
                .desired_width(f32::INFINITY)
                .frame(false)
                .font(egui::TextStyle::Monospace);

            let response = ui.add(text_edit);

            // Only grab focus if no rename or text editing is happening elsewhere.
            // tab_renaming_index covers tab rename, hierarchy_renaming covers hierarchy.
            // For inspector, we simply check if the terminal already lost focus to
            // another widget — if so, don't steal it back this frame.
            // Only grab focus if no rename or text editing is happening elsewhere.
            let text_edit_elsewhere = vm.tab_renaming_index.is_some()
                || vm.hierarchy_renaming
                || vm.inspector_renaming
                || vm.structure_manager_open;

            // Only request focus if we don't have it and nothing else needs it
            if !text_edit_elsewhere {
                if !response.has_focus() && !response.lost_focus() {
                    // Check if we should auto-focus (e.g. not interacting with other widgets)
                    // For now, we keep it aggressive but respect the flags
                    response.request_focus();
                }
            }

            // Handle Arrow keys for history navigation
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                vm.history_up();
                vm.process_command();
            } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                vm.history_down();
            }

            // Handle Enter key
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                vm.process_command();
            }
        });

        ui.separator();

        // 3. Scroll Area (Fills remaining space)
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                // Ensure content is top-down
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.set_width(ui.available_width());
                    for line in &vm.command_history {
                        ui.label(egui::RichText::new(line).monospace());
                    }
                });
            });
    });
}
