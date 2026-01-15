use crate::commands::InputModifiers;
use crate::model::Vector2;
use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Handle a click on the canvas (mouse down/up without drag)
    pub fn handle_click(&mut self, pos: Vector2, modifiers: InputModifiers) {
        let effective_pos = self.get_effective_position(pos);

        if self.executor.is_active() {
            // Save state before modifying
            self.save_undo_state();

            self.executor.push_point(
                effective_pos,
                &mut self.model,
                &self.selection_manager.selected_indices,
            );
            self.command_history.push(format!(
                "Point: {:.2}, {:.2}",
                effective_pos.x, effective_pos.y
            ));
        } else {
            // Delegate to SelectionManager
            let msg = self.selection_manager.handle_click_selection(
                pos,
                5.0 / self.viewport.zoom,
                &self.model,
                modifiers.shift,
                modifiers.ctrl,
            );
            self.executor.status_message = msg;
        }
    }

    pub fn handle_drag_start(&mut self, pos: Vector2, modifiers: InputModifiers) {
        // Reset drag state
        self.dragging_label_index = None;
        self.drag_last_pos = None;

        if !self.executor.is_active() {
            // Check for label dragging first
            let tolerance = 5.0 / self.viewport.zoom;
            // Iterate all entities to find Lines with labels
            for (i, entity) in self.model.entities.iter().enumerate().rev() {
                if let crate::model::Entity::Line(line) = entity {
                    if line.hit_test_label(pos, tolerance) {
                        self.dragging_label_index = Some(i);
                        self.drag_last_pos = Some(pos);
                        self.executor.status_message = "Dragging label...".to_string();
                        // Also select the line if not selected
                        if !self.selection_manager.selected_indices.contains(&i) {
                            if !modifiers.shift && !modifiers.ctrl {
                                self.selection_manager.clear();
                            }
                            self.selection_manager.selected_indices.insert(i);
                        }
                        return; // Found a label to drag, skip selection rect
                    }
                }
            }

            if !modifiers.shift && !modifiers.ctrl {
                self.selection_manager.clear();
            }

            self.selection_manager.start_selection_rect(pos);
            self.executor.status_message = "Drag to select...".to_string();
        }
    }

    pub fn handle_drag_update(&mut self, pos: Vector2) {
        if let Some(idx) = self.dragging_label_index {
            if let Some(last_pos) = self.drag_last_pos {
                let delta = pos - last_pos;

                // Update the specific line entity
                if let Some(crate::model::Entity::Line(line)) = self.model.entities.get_mut(idx) {
                    line.label_offset = line.label_offset + delta;
                }

                self.drag_last_pos = Some(pos);
            }
        } else {
            self.selection_manager.update_selection_rect(pos);
        }
    }

    pub fn handle_drag_end(&mut self, _modifiers: InputModifiers) {
        if self.selection_manager.selection_rect_start.is_some() {
            let msg = self.selection_manager.end_selection_rect(&self.model);
            self.executor.status_message = msg;
        }
    }

    /// Delete selected entity
    pub fn delete_selected(&mut self) {
        if !self.selection_manager.is_empty() {
            self.save_undo_state();

            let (msg, count) = self.selection_manager.delete_selected(&mut self.model);
            self.executor.status_message = msg;
            self.command_history
                .push(format!("Deleted {} items", count));
        } else {
            self.executor.status_message = "Nothing selected to delete".to_string();
        }
    }
}
