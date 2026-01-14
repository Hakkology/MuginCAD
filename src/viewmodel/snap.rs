use crate::commands::InputModifiers;
use crate::model::Vector2;
use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Update snap point based on mouse position and modifiers
    pub fn update_snap(&mut self, pos: Vector2, modifiers: InputModifiers) {
        if modifiers.ctrl {
            self.current_snap = self
                .snap_system
                .find_nearest(pos, &self.model, &self.config);
        } else {
            self.current_snap = None;
        }
    }

    /// Get the effective position (snapped if Ctrl is pressed)
    pub fn get_effective_position(&self, pos: Vector2) -> Vector2 {
        if let Some(snap) = &self.current_snap {
            snap.position
        } else {
            pos
        }
    }
}
