use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Save current state for undo
    pub fn save_undo_state(&mut self) {
        self.undo_manager.save_state(&self.model.entities);
    }

    /// Perform undo
    pub fn undo(&mut self) -> bool {
        if let Some(previous_state) = self.undo_manager.undo(&self.model.entities) {
            self.model.entities = previous_state;
            self.selected_indices.clear();
            self.command_history.push("Undo".to_string());
            self.executor.status_message = "Undo".to_string();
            true
        } else {
            self.executor.status_message = "Nothing to undo".to_string();
            false
        }
    }

    /// Perform redo
    pub fn redo(&mut self) -> bool {
        if let Some(redo_state) = self.undo_manager.redo(&self.model.entities) {
            self.model.entities = redo_state;
            self.selected_indices.clear();
            self.command_history.push("Redo".to_string());
            self.executor.status_message = "Redo".to_string();
            true
        } else {
            self.executor.status_message = "Nothing to redo".to_string();
            false
        }
    }

    /// Navigate history up (older commands)
    pub fn history_up(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        let start_index = self.history_nav_index.unwrap_or(self.command_history.len());

        // Find previous user command (starts with "> ")
        for i in (0..start_index).rev() {
            if self.command_history[i].starts_with("> ") {
                self.history_nav_index = Some(i);
                // remove "> " prefix
                self.command_input = self.command_history[i][2..].to_string();
                return;
            }
        }
    }

    /// Navigate history down (newer commands)
    pub fn history_down(&mut self) {
        if let Some(current_index) = self.history_nav_index {
            // Find next user command
            for i in (current_index + 1)..self.command_history.len() {
                if self.command_history[i].starts_with("> ") {
                    self.history_nav_index = Some(i);
                    self.command_input = self.command_history[i][2..].to_string();
                    return;
                }
            }

            // If no newer command found, clear input (we are back at "now")
            self.history_nav_index = None;
            self.command_input.clear();
        }
    }
}
