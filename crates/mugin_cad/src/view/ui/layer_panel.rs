use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_layer_panel(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let tab = &mut vm.tabs[vm.active_tab_index];
    let layer_manager = &mut tab.model.layer_manager;
    let next_id = layer_manager.layers.keys().max().unwrap_or(&0) + 1; // Simple next_id logic for UI if needed locally, but manager handles it.

    ui.vertical(|ui| {
        // Toolbar
        ui.horizontal(|ui| {
            if ui.button("➕ New Layer").clicked() {
                layer_manager.add_layer(format!("Layer {}", next_id), egui::Color32::WHITE);
            }
            if ui.button("🗑 Delete").clicked() {
                let active = layer_manager.active_layer_id;
                layer_manager.remove_layer(active);
            }
        });

        ui.separator();

        // Layer List
        let mut layers: Vec<_> = layer_manager.layers.values_mut().collect();
        layers.sort_by_key(|l| l.id);

        let mut next_active = layer_manager.active_layer_id;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for layer in layers {
                ui.horizontal(|ui| {
                    // ... (rest same as before but with focus check)
                    // Active Radio
                    if ui.radio(next_active == layer.id, "").clicked() {
                        next_active = layer.id;
                    }

                    // Visibility Toggle
                    let icon = if layer.is_visible { "👁" } else { "🚫" };
                    if ui.button(icon).clicked() {
                        layer.is_visible = !layer.is_visible;
                    }

                    // Color Swatch
                    ui.color_edit_button_srgba(&mut layer.color);

                    // Name
                    let response = ui.text_edit_singleline(&mut layer.name);
                    if response.has_focus() || response.clicked() {
                        vm.inspector_renaming = true;
                    }
                });
            }
        });

        // Update active layer if changed
        if next_active != layer_manager.active_layer_id {
            layer_manager.set_active_layer(next_active);
        }
    });
}
