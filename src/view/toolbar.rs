use crate::viewmodel::CadViewModel;
use eframe::egui;

/// Render the left toolbar with icon buttons for commands
pub fn render_toolbar(ctx: &egui::Context, vm: &mut CadViewModel) {
    egui::SidePanel::left("toolbar")
        .resizable(false)
        .default_width(48.0)
        .frame(
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(35, 35, 35))
                .inner_margin(4.0),
        )
        .show(ctx, |ui| {
            let has_selection = !vm.selected_indices.is_empty();
            let btn_size = egui::vec2(36.0, 36.0);

            // Center buttons horizontally
            ui.vertical_centered(|ui| {
                // Transform section
                if ui
                    .add_enabled(has_selection, egui::Button::new("M").min_size(btn_size))
                    .on_hover_text("Move (W)")
                    .clicked()
                {
                    vm.executor
                        .start_command("move", &mut vm.model, &vm.selected_indices);
                }

                if ui
                    .add_enabled(has_selection, egui::Button::new("R").min_size(btn_size))
                    .on_hover_text("Rotate (E)")
                    .clicked()
                {
                    vm.executor
                        .start_command("rotate", &mut vm.model, &vm.selected_indices);
                }

                if ui
                    .add_enabled(has_selection, egui::Button::new("S").min_size(btn_size))
                    .on_hover_text("Scale (R)")
                    .clicked()
                {
                    vm.executor
                        .start_command("scale", &mut vm.model, &vm.selected_indices);
                }

                ui.add_space(4.0);

                // Clipboard section
                if ui
                    .add_enabled(has_selection, egui::Button::new("C").min_size(btn_size))
                    .on_hover_text("Copy (Ctrl+C)")
                    .clicked()
                {
                    vm.executor
                        .start_command("copy", &mut vm.model, &vm.selected_indices);
                }

                if ui
                    .add_enabled(has_selection, egui::Button::new("X").min_size(btn_size))
                    .on_hover_text("Cut (Ctrl+X)")
                    .clicked()
                {
                    vm.executor
                        .start_command("cut", &mut vm.model, &vm.selected_indices);
                }

                ui.separator();
                ui.add_space(4.0);

                // Shapes section
                if ui
                    .add(egui::Button::new("/").min_size(btn_size))
                    .on_hover_text("Line (L)")
                    .clicked()
                {
                    vm.executor
                        .start_command("line", &mut vm.model, &vm.selected_indices);
                }

                if ui
                    .add(egui::Button::new("O").min_size(btn_size))
                    .on_hover_text("Circle (C)")
                    .clicked()
                {
                    vm.executor
                        .start_command("circle", &mut vm.model, &vm.selected_indices);
                }

                if ui
                    .add(egui::Button::new("[]").min_size(btn_size))
                    .on_hover_text("Rectangle")
                    .clicked()
                {
                    vm.executor
                        .start_command("rect", &mut vm.model, &vm.selected_indices);
                }

                if ui
                    .add(egui::Button::new("(").min_size(btn_size))
                    .on_hover_text("Arc")
                    .clicked()
                {
                    vm.executor
                        .start_command("arc", &mut vm.model, &vm.selected_indices);
                }

                ui.separator();
                ui.add_space(4.0);

                // Construction section
                if ui
                    .add(egui::Button::new("+").min_size(btn_size))
                    .on_hover_text("Axis (A)")
                    .clicked()
                {
                    vm.executor
                        .start_command("axis", &mut vm.model, &vm.selected_indices);
                }

                if ui
                    .add(egui::Button::new("T").min_size(btn_size))
                    .on_hover_text("Trim (T)")
                    .clicked()
                {
                    vm.executor
                        .start_command("trim", &mut vm.model, &vm.selected_indices);
                }
            });
        });
}
