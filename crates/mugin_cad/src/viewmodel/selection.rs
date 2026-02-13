use crate::model::{CadModel, Vector2};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct SelectionManager {
    pub selected_ids: HashSet<u64>,
    pub selection_rect_start: Option<Vector2>,
    pub selection_rect_current: Option<Vector2>,
    pub last_interacted_id: Option<u64>,
}

impl SelectionManager {
    pub fn new() -> Self {
        Self {
            selected_ids: HashSet::new(),
            selection_rect_start: None,
            selection_rect_current: None,
            last_interacted_id: None,
        }
    }

    pub fn clear(&mut self) {
        self.selected_ids.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.selected_ids.is_empty()
    }

    /// Handle click selection logic
    /// Returns a status message string
    pub fn handle_click_selection(
        &mut self,
        pos: Vector2,
        tolerance: f32,
        model: &CadModel,
        shift: bool,
        ctrl: bool,
    ) -> String {
        // Selection mode - single click selection
        let picked_id = model.pick_entity_id(pos, tolerance);

        if let Some(id) = picked_id {
            if shift || ctrl {
                // Toggle selection
                if self.selected_ids.contains(&id) {
                    self.selected_ids.remove(&id);
                } else {
                    self.selected_ids.insert(id);
                    self.last_interacted_id = Some(id);
                }
            } else {
                // Single selection
                self.selected_ids.clear();
                self.selected_ids.insert(id);
                self.last_interacted_id = Some(id);
            }
            format!("Selected {} items", self.selected_ids.len())
        } else {
            if !shift && !ctrl {
                self.selected_ids.clear();
                "Selection cleared".to_string()
            } else {
                // Maintained selection
                format!("Selected {} items", self.selected_ids.len())
            }
        }
    }

    pub fn start_selection_rect(&mut self, pos: Vector2) {
        self.selection_rect_start = Some(pos);
        self.selection_rect_current = Some(pos);
    }

    pub fn update_selection_rect(&mut self, pos: Vector2) {
        if self.selection_rect_start.is_some() {
            self.selection_rect_current = Some(pos);
        }
    }

    /// End selection rect dragging and select entities inside
    /// Returns status message
    pub fn end_selection_rect(&mut self, model: &CadModel) -> String {
        if let (Some(start), Some(end)) = (self.selection_rect_start, self.selection_rect_current) {
            // Calculate rect
            let min = Vector2::new(start.x.min(end.x), start.y.min(end.y));
            let max = Vector2::new(start.x.max(end.x), start.y.max(end.y));

            // Find entities in rect
            for entity in &model.entities {
                let (e_min, e_max) = entity.bounding_box();

                // Check if entity is fully inside selection rect
                if e_min.x >= min.x && e_max.x <= max.x && e_min.y >= min.y && e_max.y <= max.y {
                    self.selected_ids.insert(entity.id);
                }
            }
        }

        self.selection_rect_start = None;
        self.selection_rect_current = None;

        format!("Selected {} items", self.selected_ids.len())
    }

    /// Delete selected entities from the model
    /// Returns status message and the number of items deleted
    pub fn delete_selected(&mut self, model: &mut CadModel) -> (String, usize) {
        if !self.selected_ids.is_empty() {
            let count = model.remove_entities_by_ids(&self.selected_ids);
            self.selected_ids.clear();
            (format!("Deleted {} items", count), count)
        } else {
            ("Nothing selected to delete".to_string(), 0)
        }
    }
}
