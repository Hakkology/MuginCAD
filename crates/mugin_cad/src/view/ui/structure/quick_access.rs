use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_quick_access(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    egui::Frame::none()
        .inner_margin(egui::Margin::symmetric(20.0, 8.0)) // Add margins
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Columns Section
                render_section(ui, "Columns", |ui| {
                    render_column_selector(ui, vm);
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                // Beams Section
                render_section(ui, "Beams", |ui| {
                    ui.add_enabled(false, egui::Label::new("Coming Soon"));
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                // Slabs Section
                render_section(ui, "Slabs", |ui| {
                    ui.add_enabled(false, egui::Label::new("Coming Soon"));
                });
            });
        });
}

fn render_section<F>(ui: &mut egui::Ui, label: &str, add_contents: F)
where
    F: FnOnce(&mut egui::Ui),
{
    ui.vertical(|ui| {
        // Label with slightly smaller text or different color
        ui.label(
            egui::RichText::new(label)
                .strong()
                .small()
                .color(egui::Color32::from_gray(180)),
        );
        ui.add_space(2.0); // Small gap between label and controls
        ui.horizontal(|ui| {
            add_contents(ui);
        });
    });
}

fn render_column_selector(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    // 1. Gather data (Immutable borrow of vm/tab)
    let (mut active_id, col_types) = {
        let tab = vm.active_tab();
        let mut types: Vec<(u64, String)> = tab
            .model
            .definitions
            .column_types
            .iter()
            .map(|(id, c)| (*id, c.name.clone()))
            .collect();
        types.sort_by(|a, b| a.1.cmp(&b.1));

        (vm.active_column_type_id, types)
    };

    // 2. Determine active ID logic (local calculation)
    if active_id.is_none() {
        if let Some(first) = col_types.first() {
            active_id = Some(first.0);
        }
    }

    // 3. Update VM state if needed (Mutable borrow of VM)
    if vm.active_column_type_id != active_id {
        vm.active_column_type_id = active_id;
    }

    // 4. Determine display name
    let current_name = if let Some(id) = active_id {
        col_types
            .iter()
            .find(|(cid, _)| *cid == id)
            .map(|(_, name)| name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Select Type...".to_string()
    };

    // 5. Render UI
    let combo = egui::ComboBox::from_id_salt("quick_col_type")
        .width(150.0)
        .selected_text(current_name);

    combo.show_ui(ui, |ui| {
        for (id, name) in col_types {
            if ui.selectable_label(active_id == Some(id), name).clicked() {
                vm.active_column_type_id = Some(id);
            }
        }
    });

    if ui.button("Place").clicked() {
        let tab = vm.active_tab_mut();
        if !tab.executor.start_command(
            "place_column",
            &mut tab.model,
            &tab.selection_manager.selected_ids.clone(),
        ) {
            tab.executor.status_message = "Could not start place_column command".to_string();
        }
    }
}
