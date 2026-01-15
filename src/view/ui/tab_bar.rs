use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_tab_bar(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.horizontal(|ui| {
        let mut action = None;

        for (i, tab) in vm.tabs.iter().enumerate() {
            let is_active = i == vm.active_tab_index;
            let name = if tab.is_dirty {
                format!("* {}", tab.name)
            } else {
                tab.name.clone()
            };

            ui.group(|ui| {
                ui.style_mut().spacing.item_spacing.x = 2.0;
                // Tab Label / Select
                if ui.selectable_label(is_active, name).clicked() {
                    action = Some(TabAction::Switch(i));
                }

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
            }
        }
    });
}

enum TabAction {
    Switch(usize),
    Close(usize),
    New,
}
