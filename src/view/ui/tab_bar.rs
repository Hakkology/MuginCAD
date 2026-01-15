use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_tab_bar(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.horizontal(|ui| {
        let mut action = None;

        let active_tab_index = vm.active_tab_index;

        for (i, tab) in vm.tabs.iter_mut().enumerate() {
            let is_active = i == active_tab_index;
            let display_name = if tab.is_dirty {
                format!("* {}", tab.name)
            } else {
                tab.name.clone()
            };

            ui.group(|ui| {
                ui.style_mut().spacing.item_spacing.x = 2.0;

                // Tab Label / Select
                let response = ui.selectable_label(is_active, display_name);
                if response.clicked() {
                    action = Some(TabAction::Switch(i));
                }

                // Context menu for renaming
                response.context_menu(|ui| {
                    if ui.button("Rename Project").clicked() {
                        action = Some(TabAction::Rename(i));
                        ui.close_menu();
                    }
                });

                // Close Button
                if ui.small_button("x").clicked() {
                    action = Some(TabAction::Close(i));
                }
            });
        }

        // New Tab Button
        if ui.button("+").clicked() {
            action = Some(TabAction::New);
        }

        // Apply action
        if let Some(act) = action {
            match act {
                TabAction::Switch(i) => {
                    vm.active_tab_index = i;
                }
                TabAction::Close(i) => {
                    vm.close_tab(i);
                }
                TabAction::New => {
                    vm.new_tab();
                }
                TabAction::Rename(i) => {
                    vm.tab_renaming_index = Some(i);
                }
            }
        }
    });

    // Rename Dialog (Popup Window)
    let mut close_rename = false;
    if let Some(idx) = vm.tab_renaming_index {
        if let Some(tab) = vm.tabs.get_mut(idx) {
            let mut open = true;
            egui::Window::new("Rename Project")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -100.0))
                .show(ui.ctx(), |ui| {
                    ui.label("Enter new name:");
                    let response = ui.text_edit_singleline(&mut tab.name);

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        close_rename = true;
                    }

                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        close_rename = true;
                    }

                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            close_rename = true;
                        }
                    });

                    // Auto-focus logic check
                    // response.request_focus() calls only once per frame if conditions met.
                    // Ideally we want to focus ONCE when window opens.
                    // But here we are in immediate mode loop.
                    // If we request focus every frame, cursor stays there. which is fine for simple input.
                    response.request_focus();
                });

            if !open {
                close_rename = true;
            }
        } else {
            // Index invalid
            close_rename = true;
        }
    }

    if close_rename {
        vm.tab_renaming_index = None;
    }
}

enum TabAction {
    Switch(usize),
    Close(usize),
    New,
    Rename(usize),
}
