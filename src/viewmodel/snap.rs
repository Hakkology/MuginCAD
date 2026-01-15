use crate::commands::InputModifiers;
use crate::model::Vector2;
use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Update snap point based on mouse position and modifiers
    pub fn update_snap(&mut self, pos: Vector2, modifiers: InputModifiers) {
        if modifiers.ctrl {
            let tab_idx = self.active_tab_index;
            // Split borrow
            let config = &self.config;
            let tab = &mut self.tabs[tab_idx];
            tab.current_snap = tab.snap_system.find_nearest(pos, &tab.model, config);
        } else {
            self.active_tab_mut().current_snap = None;
        }
    }

    /// Get the effective position (snapped if Ctrl is pressed)
    pub fn get_effective_position(&self, pos: Vector2) -> Vector2 {
        if let Some(snap) = &self.active_tab().current_snap {
            snap.position
        } else {
            pos
        }
    }
}
