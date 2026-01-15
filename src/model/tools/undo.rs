use crate::model::Entity;

/// Manages undo/redo history using state snapshots
pub struct UndoManager {
    /// Stack of previous states (most recent at the end)
    undo_stack: Vec<Vec<Entity>>,
    /// Stack of undone states for redo
    redo_stack: Vec<Vec<Entity>>,
    /// Maximum number of undo levels
    max_levels: usize,
}

impl UndoManager {
    pub fn new(max_levels: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_levels,
        }
    }

    /// Save current state before making changes
    pub fn save_state(&mut self, entities: &[Entity]) {
        // Clear redo stack when new action is performed
        self.redo_stack.clear();

        // Save current state
        self.undo_stack.push(entities.to_vec());

        // Limit stack size
        if self.undo_stack.len() > self.max_levels {
            self.undo_stack.remove(0);
        }
    }

    /// Undo: restore previous state and return it
    pub fn undo(&mut self, current_entities: &[Entity]) -> Option<Vec<Entity>> {
        if let Some(previous_state) = self.undo_stack.pop() {
            // Save current state for redo
            self.redo_stack.push(current_entities.to_vec());
            Some(previous_state)
        } else {
            None
        }
    }

    /// Redo: restore undone state
    pub fn redo(&mut self, current_entities: &[Entity]) -> Option<Vec<Entity>> {
        if let Some(redo_state) = self.redo_stack.pop() {
            // Save current state for undo
            self.undo_stack.push(current_entities.to_vec());
            Some(redo_state)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get undo stack size
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
