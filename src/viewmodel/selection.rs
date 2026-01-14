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

            self.executor
                .push_point(effective_pos, &mut self.model, &self.selected_indices);
            self.command_history.push(format!(
                "Point: {:.2}, {:.2}",
                effective_pos.x, effective_pos.y
            ));
        } else {
            // Selection mode - single click selection
            let picked_idx = self.model.pick_entity(pos, 5.0 / self.viewport.zoom);

            if let Some(idx) = picked_idx {
                if modifiers.shift || modifiers.ctrl {
                    // Toggle selection
                    if self.selected_indices.contains(&idx) {
                        self.selected_indices.remove(&idx);
                    } else {
                        self.selected_indices.insert(idx);
                    }
                } else {
                    // Single selection
                    self.selected_indices.clear();
                    self.selected_indices.insert(idx);
                }
                self.executor.status_message =
                    format!("Selected {} items", self.selected_indices.len());
            } else {
                if !modifiers.shift && !modifiers.ctrl {
                    self.selected_indices.clear();
                    self.executor.status_message = "Selection cleared".to_string();
                }
            }
        }
    }

    pub fn handle_drag_start(&mut self, pos: Vector2, modifiers: InputModifiers) {
        if !self.executor.is_active() {
            if !modifiers.shift && !modifiers.ctrl {
                self.selected_indices.clear();
            }

            self.selection_rect_start = Some(pos);
            self.selection_rect_current = Some(pos);
            self.executor.status_message = "Drag to select...".to_string();
        }
    }

    pub fn handle_drag_update(&mut self, pos: Vector2) {
        if self.selection_rect_start.is_some() {
            self.selection_rect_current = Some(pos);
        }
    }

    pub fn handle_drag_end(&mut self, _modifiers: InputModifiers) {
        if let (Some(start), Some(end)) = (self.selection_rect_start, self.selection_rect_current) {
            // Calculate rect
            let min = Vector2::new(start.x.min(end.x), start.y.min(end.y));
            let max = Vector2::new(start.x.max(end.x), start.y.max(end.y));

            // Find entities in rect
            for (i, entity) in self.model.entities.iter().enumerate() {
                let e_min = match entity {
                    crate::model::Entity::Line(l) => {
                        Vector2::new(l.start.x.min(l.end.x), l.start.y.min(l.end.y))
                    }
                    crate::model::Entity::Circle(c) => {
                        Vector2::new(c.center.x - c.radius, c.center.y - c.radius)
                    }
                    crate::model::Entity::Rectangle(r) => r.min,
                    crate::model::Entity::Arc(a) => {
                        Vector2::new(a.center.x - a.radius, a.center.y - a.radius)
                    }
                };
                let e_max = match entity {
                    crate::model::Entity::Line(l) => {
                        Vector2::new(l.start.x.max(l.end.x), l.start.y.max(l.end.y))
                    }
                    crate::model::Entity::Circle(c) => {
                        Vector2::new(c.center.x + c.radius, c.center.y + c.radius)
                    }
                    crate::model::Entity::Rectangle(r) => r.max,
                    crate::model::Entity::Arc(a) => {
                        Vector2::new(a.center.x + a.radius, a.center.y + a.radius)
                    }
                };

                // Check if entity is fully inside selection rect
                if e_min.x >= min.x && e_max.x <= max.x && e_min.y >= min.y && e_max.y <= max.y {
                    self.selected_indices.insert(i);
                }
            }

            self.executor.status_message =
                format!("Selected {} items", self.selected_indices.len());
        }

        self.selection_rect_start = None;
        self.selection_rect_current = None;
    }

    /// Delete selected entity
    pub fn delete_selected(&mut self) {
        if !self.selected_indices.is_empty() {
            self.save_undo_state();

            // Delete indices in descending order to avoid index shifting problems
            let mut sorted_indices: Vec<usize> = self.selected_indices.iter().cloned().collect();
            sorted_indices.sort_by(|a, b| b.cmp(a));

            for idx in sorted_indices {
                if idx < self.model.entities.len() {
                    self.model.entities.remove(idx);
                }
            }

            let count = self.selected_indices.len();
            self.selected_indices.clear();
            self.executor.status_message = format!("Deleted {} items", count);
            self.command_history
                .push(format!("Deleted {} items", count));
        } else {
            self.executor.status_message = "Nothing selected to delete".to_string();
        }
    }
}
