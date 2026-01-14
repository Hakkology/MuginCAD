use crate::commands::InputModifiers;
use crate::commands::executor::CommandExecutor;
use crate::model::config::AppConfig;
use crate::model::snap::{SnapPoint, SnapSystem};
use crate::model::undo::UndoManager;
use crate::model::{CadModel, Vector2};
use crate::view::viewport::Viewport;

pub struct CadViewModel {
    pub model: CadModel,
    pub command_input: String,
    pub command_history: Vec<String>,
    pub executor: CommandExecutor,
    pub selected_entity_idx: Option<usize>,
    pub snap_system: SnapSystem,
    pub current_snap: Option<SnapPoint>,
    pub undo_manager: UndoManager,
    pub viewport: Viewport,
    pub config: AppConfig,
    pub show_settings_window: bool,
}

impl CadViewModel {
    pub fn new() -> Self {
        Self {
            model: CadModel::new(),
            command_input: String::new(),
            command_history: Vec::new(),
            executor: CommandExecutor::new(),
            selected_entity_idx: None,
            snap_system: SnapSystem::new(),
            current_snap: None,
            undo_manager: UndoManager::new(50), // 50 undo levels
            viewport: Viewport::new(),
            config: AppConfig::default(),
            show_settings_window: false,
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
            self.selected_entity_idx = None;
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
            self.selected_entity_idx = None;
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

        self.command_history.push(format!("> {}", input_text));

        // Handle special commands
        let clean = input_text.trim().to_lowercase();
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
                self.selected_entity_idx = None;
                self.executor.cancel();
                return;
            }
            _ => {}
        }

        // Save state before command execution
        self.save_undo_state();

        // Process with command executor
        self.executor
            .process_input(&input_text, &mut self.model, self.selected_entity_idx);
    }

    /// Handle a click on the canvas
    pub fn handle_click(&mut self, pos: Vector2) {
        let effective_pos = self.get_effective_position(pos);

        if self.executor.is_active() {
            // Save state before modifying
            self.save_undo_state();

            self.executor
                .push_point(effective_pos, &mut self.model, self.selected_entity_idx);
            self.command_history.push(format!(
                "Point: {:.2}, {:.2}",
                effective_pos.x, effective_pos.y
            ));
        } else {
            // Selection mode - no undo needed
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

    /// Create a new empty project
    pub fn new_project(&mut self) {
        self.model.entities.clear();
        self.model.axis_manager.axes.clear();
        self.undo_manager = UndoManager::new(50);
        self.command_history.clear();
        self.selected_entity_idx = None;
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
                    self.selected_entity_idx = None;
                    self.current_snap = None;
                    self.executor.cancel();
                }
            }
        }
    }
}
