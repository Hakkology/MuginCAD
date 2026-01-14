use crate::commands::InputModifiers;
use crate::commands::executor::CommandExecutor;
use crate::model::snap::{SnapPoint, SnapSystem};
use crate::model::{CadModel, Vector2};

pub struct CadViewModel {
    pub model: CadModel,
    pub command_input: String,
    pub command_history: Vec<String>,
    pub executor: CommandExecutor,
    pub selected_entity_idx: Option<usize>,
    pub snap_system: SnapSystem,
    pub current_snap: Option<SnapPoint>,
}

impl CadViewModel {
    pub fn new() -> Self {
        Self {
            model: CadModel::new(),
            command_input: String::new(),
            command_history: Vec::new(),
            executor: CommandExecutor::new(),
            selected_entity_idx: None,
            snap_system: SnapSystem::new(15.0), // 15 pixel tolerance
            current_snap: None,
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
            self.current_snap = self.snap_system.find_nearest(pos, &self.model);
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

    /// Process command input from terminal
    pub fn process_command(&mut self) {
        let input_text = self.command_input.trim().to_string();
        self.command_input.clear();

        if input_text.is_empty() {
            // Empty input cancels current command
            if self.executor.is_active() {
                self.executor.cancel();
            }
            return;
        }

        self.command_history.push(format!("> {}", input_text));

        // Handle special commands
        let clean = input_text.trim().to_lowercase();
        match clean.as_str() {
            "fill" | "shade" => {
                let mode = self.executor.toggle_filled();
                let mode_str = if mode { "ON" } else { "OFF" };
                self.executor.status_message = format!("SHADE mode: {}", mode_str);
                self.command_history
                    .push(format!("Shade mode is now {}", mode_str));
                return;
            }
            "clear" => {
                self.model.entities.clear();
                self.command_history.clear();
                self.selected_entity_idx = None;
                self.executor.cancel();
                return;
            }
            _ => {}
        }

        // Process with command executor
        self.executor
            .process_input(&input_text, &mut self.model, self.selected_entity_idx);
    }

    /// Handle a click on the canvas
    pub fn handle_click(&mut self, pos: Vector2) {
        // Use snapped position if available
        let effective_pos = self.get_effective_position(pos);

        if self.executor.is_active() {
            self.executor
                .push_point(effective_pos, &mut self.model, self.selected_entity_idx);
            self.command_history.push(format!(
                "Point: {:.2}, {:.2}",
                effective_pos.x, effective_pos.y
            ));
        } else {
            // Selection mode
            self.selected_entity_idx = self.model.pick_entity(pos, 5.0);
            if let Some(idx) = self.selected_entity_idx {
                let entity = &self.model.entities[idx];
                self.executor.status_message = format!("Selected: {}", entity.type_name());
            } else {
                self.executor.status_message = "Command:".to_string();
            }
        }
    }

    /// Cancel current command (right-click or Escape)
    pub fn cancel_command(&mut self) {
        self.executor.cancel();
    }
}
