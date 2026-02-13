use crate::viewmodel::CadViewModel;
use eframe::egui;

/// Handle all global keyboard shortcuts.
///
/// Called once per frame from the main update loop.
pub fn handle(ctx: &egui::Context, vm: &mut CadViewModel) {
    // If we are renaming or any text edit has focus, skip global shortcuts
    if vm.tab_renaming_index.is_some()
        || vm.hierarchy_renaming
        || vm.inspector_renaming
        || ctx.memory(|m| m.focused().is_some())
    {
        return;
    }

    // Escape — cancel active command, clear selection, or clear input
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        if !vm.command_input.is_empty() {
            vm.command_input.clear();
        } else if vm.active_tab().executor.is_active() {
            vm.cancel_command();
        } else if !vm.active_tab().selection_manager.selected_ids.is_empty() {
            vm.active_tab_mut().selection_manager.selected_ids.clear();
        }
    }

    // End — reset viewport
    if ctx.input(|i| i.key_pressed(egui::Key::End)) {
        if let Some(tab) = vm.tabs.get_mut(vm.active_tab_index) {
            tab.viewport.reset();
        }
    }

    // Delete — delete selected entities
    if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
        if !vm.active_tab().selection_manager.selected_ids.is_empty() {
            vm.command_input = "delete".to_string();
            vm.process_command();
        }
    }

    // Ctrl+C — copy
    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::C)) {
        let tab = vm.active_tab_mut();
        if !tab.selection_manager.selected_ids.is_empty() && !tab.executor.is_active() {
            let ids = tab.selection_manager.selected_ids.clone();
            tab.executor.start_command("copy", &mut tab.model, &ids);
        }
    }

    // Ctrl+X — cut
    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::X)) {
        let tab = vm.active_tab_mut();
        if !tab.selection_manager.selected_ids.is_empty() && !tab.executor.is_active() {
            let ids = tab.selection_manager.selected_ids.clone();
            tab.executor.start_command("cut", &mut tab.model, &ids);
        }
    }
}
