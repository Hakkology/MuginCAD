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
                egui::RichText::new(&vm.status_message)
                    .strong()
                    .color(egui::Color32::LIGHT_BLUE),
            );

            let text_edit = egui::TextEdit::singleline(&mut vm.command_input)
                .desired_width(f32::INFINITY)
                .frame(false)
                .font(egui::TextStyle::Monospace);

            let response = ui.add(text_edit);

            if vm.command_history.is_empty() {
                response.request_focus();
            }

            if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                vm.process_command();
                response.request_focus();
            }
        });
    });
}
