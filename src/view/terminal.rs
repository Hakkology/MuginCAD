use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_terminal(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.vertical(|ui| {
        ui.style_mut().visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 200));

        let scroll_height = ui.available_height() - 30.0;
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .max_height(scroll_height)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                for line in &vm.command_history {
                    ui.label(egui::RichText::new(line).monospace());
                }
            });

        ui.separator();

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

            // Always keep focus on terminal
            if !response.has_focus() {
                response.request_focus();
            }

            // Handle Enter key
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                vm.process_command();
            }
        });

        ui.separator();

        // Settings bar
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
    });
}
