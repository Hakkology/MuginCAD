use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Process command input from terminal
    pub fn process_command(&mut self) {
        let input_text = self.command_input.trim().to_string();
        self.command_input.clear();

        if input_text.is_empty() {
            if self.executor.is_active() {
                self.executor.cancel();
            }
            return;
        }

        // Reset history navigation
        self.history_nav_index = None;

        self.command_history.push(format!("> {}", input_text));

        // Handle special commands
        let clean = input_text.trim().to_lowercase();

        if self.pending_delete_confirmation {
            match clean.as_str() {
                "y" | "yes" => {
                    self.delete_selected();
                    self.pending_delete_confirmation = false;
                }
                "n" | "no" => {
                    self.executor.status_message = "Delete cancelled".to_string();
                    self.command_history.push("Cancelled.".to_string());
                    self.pending_delete_confirmation = false;
                }
                _ => {
                    self.executor.status_message = "Delete cancelled (invalid input)".to_string();
                    self.pending_delete_confirmation = false;
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
                let mode = self.executor.toggle_filled();
                let mode_str = if mode { "ON" } else { "OFF" };
                self.executor.status_message = format!("SHADE mode: {}", mode_str);
                self.command_history
                    .push(format!("Shade mode is now {}", mode_str));
                return;
            }
            "clear" => {
                self.save_undo_state();
                self.model.entities.clear();
                self.command_history.clear();
                self.selection_manager.selected_indices.clear();
                self.executor.cancel();
                return;
            }
            "d" | "delete" => {
                if !self.selection_manager.selected_indices.is_empty() {
                    self.pending_delete_confirmation = true;
                    self.executor.status_message =
                        "Are you sure you want to delete? (Y/N)".to_string();
                    self.command_history
                        .push("Are you sure you want to delete? (Y/N)".to_string());
                } else {
                    self.executor.status_message = "Nothing selected to delete".to_string();
                }
                return;
            }
            _ => {}
        }

        // Save state before command execution
        self.save_undo_state();

        // Process with command executor
        self.executor.process_input(
            &input_text,
            &mut self.model,
            &self.selection_manager.selected_indices,
        );
    }

    /// Cancel current command (right-click or Escape)
    pub fn cancel_command(&mut self) {
        self.executor.cancel();
        if self.pending_delete_confirmation {
            self.pending_delete_confirmation = false;
            self.executor.status_message = "Cancelled".to_string();
        }
        // Also clear selection rect if we were dragging
        self.selection_manager.selection_rect_start = None;
        self.selection_manager.selection_rect_current = None;
    }
}
