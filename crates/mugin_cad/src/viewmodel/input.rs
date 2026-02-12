use crate::commands::InputModifiers;
use crate::model::{Shape, Vector2};
use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Handle a click on the canvas (mouse down/up without drag)
    pub fn handle_click(&mut self, pos: Vector2, modifiers: InputModifiers) {
        let effective_pos = self.get_effective_position(pos);

        // Check if executor is active without holding mutable borrow too long
        let is_active = self.active_tab().executor.is_active();

        if is_active {
            // Save state before modifying
            self.save_undo_state();

            let tab = self.active_tab_mut();
            tab.executor.push_point(
                effective_pos,
                &mut tab.model,
                &tab.selection_manager.selected_indices,
            );
            self.command_history.push(format!(
                "Point: {:.2}, {:.2}",
                effective_pos.x, effective_pos.y
            ));
        } else {
            // Delegate to SelectionManager
            let tab = self.active_tab_mut();
            let msg = tab.selection_manager.handle_click_selection(
                pos,
                5.0 / tab.viewport.zoom,
                &tab.model,
                modifiers.shift,
                modifiers.ctrl,
            );
            tab.executor.status_message = msg;
        }
    }

    pub fn handle_drag_start(&mut self, pos: Vector2, modifiers: InputModifiers) {
        let tab = self.active_tab_mut();
        // Reset drag state
        tab.dragging_label_index = None;
        tab.drag_last_pos = None;

        if !tab.executor.is_active() {
            // Check for label dragging first
            let tolerance = 5.0 / tab.viewport.zoom;
            // Iterate all entities to find Lines with labels
            // We need to find index first to avoid multiple borrow issues inside loop
            let mut label_drag_index = None;

            for (i, entity) in tab.model.entities.iter().enumerate().rev() {
                match &entity.shape {
                    Shape::Line(line) => {
                        if line.hit_test_label(pos, tolerance) {
                            label_drag_index = Some(i);
                            break;
                        }
                    }
                    Shape::Text(text) => {
                        if text.hit_test(pos, tolerance) {
                            label_drag_index = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if let Some(i) = label_drag_index {
                tab.dragging_label_index = Some(i);
                tab.drag_last_pos = Some(pos);
                tab.executor.status_message = "Dragging label...".to_string();

                // Also select the line if not selected
                if !tab.selection_manager.selected_indices.contains(&i) {
                    if !modifiers.shift && !modifiers.ctrl {
                        tab.selection_manager.clear();
                    }
                    tab.selection_manager.selected_indices.insert(i);
                }
                return; // Found a label to drag, skip selection rect
            }

            // Normal selection drag
            if !modifiers.shift && !modifiers.ctrl {
                tab.selection_manager.clear();
            }

            tab.selection_manager.start_selection_rect(pos);
            tab.executor.status_message = "Drag to select...".to_string();
        }
    }

    pub fn handle_drag_update(&mut self, pos: Vector2) {
        let tab = self.active_tab_mut();
        if let Some(idx) = tab.dragging_label_index {
            if let Some(last_pos) = tab.drag_last_pos {
                let delta = pos - last_pos;

                // Update the specific entity
                if let Some(entity) = tab.model.entities.get_mut(idx) {
                    match &mut entity.shape {
                        Shape::Line(line) => {
                            line.label_offset = line.label_offset + delta;
                        }
                        Shape::Text(text) => {
                            text.position = text.position + delta;
                        }
                        _ => {}
                    }
                }

                tab.drag_last_pos = Some(pos);
            }
        } else {
            tab.selection_manager.update_selection_rect(pos);
        }
    }

    pub fn handle_drag_end(&mut self, _modifiers: InputModifiers) {
        let tab = self.active_tab_mut();
        if tab.selection_manager.selection_rect_start.is_some() {
            let msg = tab.selection_manager.end_selection_rect(&tab.model);
            tab.executor.status_message = msg;
        }
    }

    /// Delete selected entity
    pub fn delete_selected(&mut self) {
        if !self.active_tab().selection_manager.is_empty() {
            self.save_undo_state();

            let tab = self.active_tab_mut();
            let (msg, count) = tab.selection_manager.delete_selected(&mut tab.model);
            tab.executor.status_message = msg;
            self.command_history
                .push(format!("Deleted {} items", count));
        } else {
            self.active_tab_mut().executor.status_message =
                "Nothing selected to delete".to_string();
        }
    }
}
