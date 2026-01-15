use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Process command input from terminal
    pub fn process_command(&mut self) {
        let input_text = self.command_input.trim().to_string();
        self.command_input.clear();

        if input_text.is_empty() {
            if self.active_tab_mut().executor.is_active() {
                self.active_tab_mut().executor.cancel();
            }
            return;
        }

        // Reset history navigation
        self.history_nav_index = None;

        self.command_history.push(format!("> {}", input_text));

        // Handle special commands
        let clean = input_text.trim().to_lowercase();

        if self.active_tab_mut().pending_delete_confirmation {
            match clean.as_str() {
                "y" | "yes" => {
                    self.delete_selected();
                    self.active_tab_mut().pending_delete_confirmation = false;
                }
                "n" | "no" => {
                    let (tab, history) = self.active_tab_mut_and_history();
                    tab.executor.status_message = "Delete cancelled".to_string();
                    history.push("Cancelled.".to_string());
                    tab.pending_delete_confirmation = false;
                }
                _ => {
                    let tab = self.active_tab_mut();
                    tab.executor.status_message = "Delete cancelled (invalid input)".to_string();
                    tab.pending_delete_confirmation = false;
                }
            }
            return;
        }

        match clean.as_str() {
            "u" | "undo" => {
                self.undo();
                return;
            }
            "redo" => {
                self.redo();
                return;
            }
            "fill" | "shade" => {
                let (tab, history) = self.active_tab_mut_and_history();
                let mode = tab.executor.toggle_filled();
                let mode_str = if mode { "ON" } else { "OFF" };
                tab.executor.status_message = format!("SHADE mode: {}", mode_str);
                history.push(format!("Shade mode is now {}", mode_str));
                return;
            }
            "clear" => {
                self.save_undo_state();
                let (tab, history) = self.active_tab_mut_and_history();
                tab.model.entities.clear();
                history.clear();
                tab.selection_manager.selected_indices.clear();
                tab.executor.cancel();
                return;
            }
            "d" | "delete" => {
                let (tab, history) = self.active_tab_mut_and_history();
                if !tab.selection_manager.selected_indices.is_empty() {
                    tab.pending_delete_confirmation = true;
                    tab.executor.status_message =
                        "Are you sure you want to delete? (Y/N)".to_string();
                    history.push("Are you sure you want to delete? (Y/N)".to_string());
                } else {
                    tab.executor.status_message = "Nothing selected to delete".to_string();
                }
                return;
            }
            _ => {}
        }

        // Save state before command execution
        self.save_undo_state();

        // Process with command executor
        let tab = self.active_tab_mut();
        tab.executor.process_input(
            &input_text,
            &mut tab.model,
            &tab.selection_manager.selected_indices,
        );
    }

    /// Cancel current command (right-click or Escape)
    pub fn cancel_command(&mut self) {
        let tab = self.active_tab_mut();
        tab.executor.cancel();
        if tab.pending_delete_confirmation {
            tab.pending_delete_confirmation = false;
            tab.executor.status_message = "Cancelled".to_string();
        }
        // Also clear selection rect if we were dragging
        tab.selection_manager.selection_rect_start = None;
        tab.selection_manager.selection_rect_current = None;
    }
}
