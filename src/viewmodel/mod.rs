use crate::commands::InputModifiers;
use crate::commands::executor::CommandExecutor;
use crate::model::config::AppConfig;
use crate::model::snap::{SnapPoint, SnapSystem};
use crate::model::undo::UndoManager;
use crate::model::{CadModel, Vector2};
use crate::view::viewport::Viewport;
use std::collections::HashSet;

pub struct CadViewModel {
    pub model: CadModel,
    pub command_input: String,
    pub command_history: Vec<String>,
    pub history_nav_index: Option<usize>,
    pub executor: CommandExecutor,
    pub selected_indices: HashSet<usize>,
    pub selection_rect_start: Option<Vector2>,
    pub selection_rect_current: Option<Vector2>,
    pub snap_system: SnapSystem,
    pub current_snap: Option<SnapPoint>,
    pub undo_manager: UndoManager,
    pub viewport: Viewport,
    pub config: AppConfig,
    pub show_settings_window: bool,
    pub pending_delete_confirmation: bool,
}

impl CadViewModel {
    pub fn new() -> Self {
        Self {
            model: CadModel::new(),
            command_input: String::new(),
            command_history: Vec::new(),
            history_nav_index: None,
            executor: CommandExecutor::new(),
            selected_indices: HashSet::new(),
            selection_rect_start: None,
            selection_rect_current: None,
            snap_system: SnapSystem::new(),
            current_snap: None,
            undo_manager: UndoManager::new(50), // 50 undo levels
            viewport: Viewport::new(),
            config: AppConfig::default(),
            show_settings_window: false,
            pending_delete_confirmation: false,
        }
    }

    /// Get status message from executor
    pub fn status_message(&self) -> &str {
        &self.executor.status_message
    }

    /// Update keyboard modifiers (called from view)
    pub fn set_modifiers(&mut self, modifiers: InputModifiers) {
        self.executor.set_modifiers(modifiers);
    }

    /// Update snap point based on mouse position and modifiers
    pub fn update_snap(&mut self, pos: Vector2, modifiers: InputModifiers) {
        if modifiers.ctrl {
            self.current_snap = self
                .snap_system
                .find_nearest(pos, &self.model, &self.config);
        } else {
            self.current_snap = None;
        }
    }

    /// Get the effective position (snapped if Ctrl is pressed)
    pub fn get_effective_position(&self, pos: Vector2) -> Vector2 {
        if let Some(snap) = &self.current_snap {
            snap.position
        } else {
            pos
        }
    }

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
                self.selected_indices.clear();
                self.executor.cancel();
                return;
            }
            "d" | "delete" => {
                if !self.selected_indices.is_empty() {
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
        self.executor
            .process_input(&input_text, &mut self.model, &self.selected_indices);
    }

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
            // Only start selection box if we are not clicking on an entity (or if we are on empty space)
            // But usually drag starts on empty space.
            // For now, let's always allow drag start if no command active.
            // If we click on an entity and drag, maybe we should move it? (Future feature)
            // For now, assume drag is always selection box if not a command.

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
            // Since we don't have a spatial index yet, we iterate all
            for (i, entity) in self.model.entities.iter().enumerate() {
                // Check if entity is inside or intersects rect
                // For simplified logic: check if bounding box of entity intersects selection rect
                // Or simplified: check if center or key points are inside.
                // Let's implement full containment for now as per plan.

                let e_min = match entity {
                    crate::model::Entity::Line(l) => {
                        Vector2::new(l.start.x.min(l.end.x), l.start.y.min(l.end.y))
                    }
                    crate::model::Entity::Circle(c) => {
                        Vector2::new(c.center.x - c.radius, c.center.y - c.radius)
                    }
                    crate::model::Entity::Rectangle(r) => r.min,
                };
                let e_max = match entity {
                    crate::model::Entity::Line(l) => {
                        Vector2::new(l.start.x.max(l.end.x), l.start.y.max(l.end.y))
                    }
                    crate::model::Entity::Circle(c) => {
                        Vector2::new(c.center.x + c.radius, c.center.y + c.radius)
                    }
                    crate::model::Entity::Rectangle(r) => r.max,
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

    /// Cancel current command (right-click or Escape)
    pub fn cancel_command(&mut self) {
        self.executor.cancel();
        if self.pending_delete_confirmation {
            self.pending_delete_confirmation = false;
            self.executor.status_message = "Cancelled".to_string();
        }
        // Also clear selection rect if we were dragging
        self.selection_rect_start = None;
        self.selection_rect_current = None;
    }

    /// Create a new empty project
    pub fn new_project(&mut self) {
        self.model.entities.clear();
        self.model.axis_manager.axes.clear();
        self.undo_manager = UndoManager::new(50);
        self.command_history.clear();
        self.selected_indices.clear();
        self.current_snap = None;
        self.config = AppConfig::default();
        self.executor.cancel();
    }

    /// Save project to a file
    pub fn save_project(&self) {
        if let Some(mut path) = rfd::FileDialog::new()
            .add_filter("OliveCAD Project", &["oliv"])
            .save_file()
        {
            // Ensure extension is present
            if path.extension().and_then(|ext| ext.to_str()) != Some("oliv") {
                path.set_extension("oliv");
            }

            let project_data = crate::model::project::ProjectData::new(
                self.model.entities.clone(),
                self.model.axis_manager.axes.clone(),
                self.config.clone(),
            );

            if let Ok(json) = serde_json::to_string_pretty(&project_data) {
                let _ = std::fs::write(path, json);
            }
        }
    }

    /// Load project from a file
    pub fn load_project(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("OliveCAD Project", &["oliv"])
            .pick_file()
        {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(project_data) =
                    serde_json::from_str::<crate::model::project::ProjectData>(&content)
                {
                    self.model.entities = project_data.entities;
                    self.model.axis_manager.axes = project_data.axes;
                    self.config = project_data.config;

                    // Reset transient state
                    self.undo_manager = UndoManager::new(50);
                    self.command_history.clear();
                    self.selected_indices.clear();
                    self.current_snap = None;
                    self.executor.cancel();
                }
            }
        }
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
