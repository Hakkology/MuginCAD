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
            if vm.tabs.is_empty() {
                ui.label("No Project");
                return;
            }

            let tab = vm.active_tab_mut();
            let has_selection = !tab.selection_manager.selected_indices.is_empty();
            let btn_size = egui::vec2(36.0, 36.0);

            // Center buttons horizontally
            ui.vertical_centered(|ui| {
                // Transform section
                if ui
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

                if ui
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

                if ui
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

                ui.add_space(4.0);

                // Clipboard section
                if ui
                    .add_enabled(has_selection, egui::Button::new("C").min_size(btn_size))
                    .on_hover_text("Copy (Ctrl+C)")
                    .clicked()
                {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("copy", &mut tab.model, &indices);
                }

                if ui
                    .add_enabled(has_selection, egui::Button::new("X").min_size(btn_size))
                    .on_hover_text("Cut (Ctrl+X)")
                    .clicked()
                {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("cut", &mut tab.model, &indices);
                }

                ui.separator();
                ui.add_space(4.0);

                // Shapes section
                if ui
                    .add(egui::Button::new("/").min_size(btn_size))
                    .on_hover_text("Line (L)")
                    .clicked()
                {
                    tab.executor.start_command(
                        "line",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                if ui
                    .add(egui::Button::new("O").min_size(btn_size))
                    .on_hover_text("Circle (C)")
                    .clicked()
                {
                    tab.executor.start_command(
                        "circle",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                if ui
                    .add(egui::Button::new("[]").min_size(btn_size))
                    .on_hover_text("Rectangle")
                    .clicked()
                {
                    tab.executor.start_command(
                        "rect",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                if ui
                    .add(egui::Button::new("(").min_size(btn_size))
                    .on_hover_text("Arc")
                    .clicked()
                {
                    tab.executor.start_command(
                        "arc",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                ui.separator();
                ui.add_space(4.0);

                // Construction section
                if ui
                    .add(egui::Button::new("+").min_size(btn_size))
                    .on_hover_text("Axis (A)")
                    .clicked()
                {
                    tab.executor.start_command(
                        "axis",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                if ui
                    .add(egui::Button::new("T").min_size(btn_size))
                    .on_hover_text("Trim (T)")
                    .clicked()
                {
                    tab.executor.start_command(
                        "trim",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                if ui
                    .add_enabled(has_selection, egui::Button::new("||").min_size(btn_size))
                    .on_hover_text("Offset (O)")
                    .clicked()
                {
                    tab.executor.start_command(
                        "offset",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
            });
        });
}
