use crate::viewmodel::CadViewModel;
use eframe::egui;

/// Handle all global keyboard shortcuts.
///
/// Called once per frame from the main update loop.
pub fn handle(ctx: &egui::Context, vm: &mut CadViewModel) {
    // Escape — cancel active command
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        vm.cancel_command();
    }

    // End — reset viewport
    if ctx.input(|i| i.key_pressed(egui::Key::End)) {
        if let Some(tab) = vm.tabs.get_mut(vm.active_tab_index) {
            tab.viewport.reset();
        }
    }

    // Delete — delete selected entities
    if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
        vm.delete_selected();
    }

    // Ctrl+C — copy
    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::C)) {
        let tab = vm.active_tab_mut();
        if !tab.selection_manager.selected_indices.is_empty() && !tab.executor.is_active() {
            let indices = tab.selection_manager.selected_indices.clone();
            tab.executor.start_command("copy", &mut tab.model, &indices);
        }
    }

    // Ctrl+X — cut
    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::X)) {
        let tab = vm.active_tab_mut();
        if !tab.selection_manager.selected_indices.is_empty() && !tab.executor.is_active() {
            let indices = tab.selection_manager.selected_indices.clone();
            tab.executor.start_command("cut", &mut tab.model, &indices);
        }
    }
}
