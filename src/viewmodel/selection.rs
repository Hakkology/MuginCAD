use crate::model::{CadModel, Entity, Vector2};
use std::collections::HashSet;

#[derive(Default)]
pub struct SelectionManager {
    pub selected_indices: HashSet<usize>,
    pub selection_rect_start: Option<Vector2>,
    pub selection_rect_current: Option<Vector2>,
}

impl SelectionManager {
    pub fn new() -> Self {
        Self {
            selected_indices: HashSet::new(),
            selection_rect_start: None,
            selection_rect_current: None,
        }
    }

    pub fn clear(&mut self) {
        self.selected_indices.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.selected_indices.is_empty()
    }

    pub fn len(&self) -> usize {
        self.selected_indices.len()
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
        let picked_idx = model.pick_entity(pos, tolerance);

        if let Some(idx) = picked_idx {
            if shift || ctrl {
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
            format!("Selected {} items", self.selected_indices.len())
        } else {
            if !shift && !ctrl {
                self.selected_indices.clear();
                "Selection cleared".to_string()
            } else {
                // Maintained selection
                format!("Selected {} items", self.selected_indices.len())
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
            for (i, entity) in model.entities.iter().enumerate() {
                let e_min = match entity {
                    Entity::Line(l) => Vector2::new(l.start.x.min(l.end.x), l.start.y.min(l.end.y)),
                    Entity::Circle(c) => Vector2::new(c.center.x - c.radius, c.center.y - c.radius),
                    Entity::Rectangle(r) => r.min,
                    Entity::Arc(a) => Vector2::new(a.center.x - a.radius, a.center.y - a.radius),
                    Entity::Text(t) => t.position,
                };
                let e_max = match entity {
                    Entity::Line(l) => Vector2::new(l.start.x.max(l.end.x), l.start.y.max(l.end.y)),
                    Entity::Circle(c) => Vector2::new(c.center.x + c.radius, c.center.y + c.radius),
                    Entity::Rectangle(r) => r.max,
                    Entity::Arc(a) => Vector2::new(a.center.x + a.radius, a.center.y + a.radius),
                    Entity::Text(t) => t.position,
                };

                // Check if entity is fully inside selection rect
                if e_min.x >= min.x && e_max.x <= max.x && e_min.y >= min.y && e_max.y <= max.y {
                    self.selected_indices.insert(i);
                }
            }
        }

        self.selection_rect_start = None;
        self.selection_rect_current = None;

        format!("Selected {} items", self.selected_indices.len())
    }

    /// Delete selected entities from the model
    /// Returns status message and the number of items deleted
    pub fn delete_selected(&mut self, model: &mut CadModel) -> (String, usize) {
        if !self.selected_indices.is_empty() {
            // Delete indices in descending order to avoid index shifting problems
            let mut sorted_indices: Vec<usize> = self.selected_indices.iter().cloned().collect();
            sorted_indices.sort_by(|a, b| b.cmp(a));

            for idx in sorted_indices {
                if idx < model.entities.len() {
                    model.entities.remove(idx);
                }
            }

            let count = self.selected_indices.len();
            self.selected_indices.clear();
            (format!("Deleted {} items", count), count)
        } else {
            ("Nothing selected to delete".to_string(), 0)
        }
    }
}
