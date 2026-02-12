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

                let response = ui.selectable_label(is_active, display_name);
                if response.clicked() {
                    action = Some(TabAction::Switch(i));
                }

                response.context_menu(|ui| {
                    if ui.button("Rename Project").clicked() {
                        action = Some(TabAction::Rename(i));
                        ui.close_menu();
                    }
                });

                if ui.small_button("x").clicked() {
                    action = Some(TabAction::Close(i));
                }
            });
        }

        if ui.button("+").clicked() {
            action = Some(TabAction::New);
        }

        if let Some(act) = action {
            match act {
                TabAction::Switch(i) => vm.active_tab_index = i,
                TabAction::Close(i) => vm.close_tab(i),
                TabAction::New => vm.new_tab(),
                TabAction::Rename(i) => vm.tab_renaming_index = Some(i),
            }
        }
    });

    // ── Rename Dialog ────────────────────────────────────────
    let mut close_rename = false;
    if let Some(idx) = vm.tab_renaming_index {
        if let Some(tab) = vm.tabs.get_mut(idx) {
            let mut open = true;
            mugin_widgets::window::modal("Rename Project", ui.ctx(), &mut open, |ui| {
                ui.label("Enter new name:");
                let response = ui.text_edit_singleline(&mut tab.name);
                response.request_focus();

                let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                let ok_clicked = ui.button("OK").clicked();

                enter_pressed || ok_clicked
            });

            if !open {
                close_rename = true;
            }
        } else {
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
