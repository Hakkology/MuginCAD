use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::toolbar;

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
            let has_sel = !tab.selection_manager.selected_indices.is_empty();

            ui.vertical_centered(|ui| {
                // ── Transform ────────────────────────────────────
                if toolbar::tool_button(ui, "M", "Move (W)", has_sel) {
                    tab.executor.start_command(
                        "move",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "R", "Rotate (E)", has_sel) {
                    tab.executor.start_command(
                        "rotate",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "S", "Scale (R)", has_sel) {
                    tab.executor.start_command(
                        "scale",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                ui.add_space(4.0);

                // ── Clipboard ────────────────────────────────────
                if toolbar::tool_button(ui, "C", "Copy (Ctrl+C)", has_sel) {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("copy", &mut tab.model, &indices);
                }
                if toolbar::tool_button(ui, "X", "Cut (Ctrl+X)", has_sel) {
                    let indices = tab.selection_manager.selected_indices.clone();
                    tab.executor.start_command("cut", &mut tab.model, &indices);
                }

                toolbar::separator(ui);

                // ── Shapes ───────────────────────────────────────
                if toolbar::tool_button(ui, "/", "Line (L)", true) {
                    tab.executor.start_command(
                        "line",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "O", "Circle (C)", true) {
                    tab.executor.start_command(
                        "circle",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "[]", "Rectangle", true) {
                    tab.executor.start_command(
                        "rect",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "(", "Arc", true) {
                    tab.executor.start_command(
                        "arc",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }

                toolbar::separator(ui);

                // ── Construction ─────────────────────────────────
                if toolbar::tool_button(ui, "+", "Axis (A)", true) {
                    tab.executor.start_command(
                        "axis",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "T", "Trim (T)", true) {
                    tab.executor.start_command(
                        "trim",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "||", "Offset (O)", has_sel) {
                    tab.executor.start_command(
                        "offset",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "Txt", "Text Annotation", true) {
                    tab.executor.start_command(
                        "text",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "Dim", "Measure Distance", true) {
                    tab.executor.start_command(
                        "measure",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "Area", "Measure Closed Area", true) {
                    tab.executor.start_command(
                        "area",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
                if toolbar::tool_button(ui, "Perim", "Measure Perimeter", true) {
                    tab.executor.start_command(
                        "perim",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
            });
        });
}
