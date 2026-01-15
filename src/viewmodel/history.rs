use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Save current state for undo
    pub fn save_undo_state(&mut self) {
        let tab = self.active_tab_mut();
        tab.undo_manager.save_state(&tab.model.entities);
    }

    /// Perform undo
    pub fn undo(&mut self) -> bool {
        let (tab, history) = self.active_tab_mut_and_history();
        if let Some(previous_state) = tab.undo_manager.undo(&tab.model.entities) {
            tab.model.entities = previous_state;
            tab.selection_manager.selected_indices.clear();
            history.push("Undo".to_string());
            tab.executor.status_message = "Undo".to_string();
            true
        } else {
            tab.executor.status_message = "Nothing to undo".to_string();
            false
        }
    }

    /// Perform redo
    pub fn redo(&mut self) -> bool {
        let (tab, history) = self.active_tab_mut_and_history();
        if let Some(redo_state) = tab.undo_manager.redo(&tab.model.entities) {
            tab.model.entities = redo_state;
            tab.selection_manager.selected_indices.clear();
            history.push("Redo".to_string());
            tab.executor.status_message = "Redo".to_string();
            true
        } else {
            tab.executor.status_message = "Nothing to redo".to_string();
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
