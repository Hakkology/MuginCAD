mod commands;
mod history;
mod project;
mod selection;
mod snap;

use crate::commands::InputModifiers;
use crate::commands::executor::CommandExecutor;
use crate::model::config::AppConfig;
use crate::model::snap::{SnapPoint, SnapSystem};
use crate::model::undo::UndoManager;
use crate::model::{CadModel, Entity, Vector2};
use crate::view::viewport::Viewport;
use std::collections::HashSet;

/// Clipboard for copy/cut/paste operations
#[derive(Default)]
pub struct Clipboard {
    pub entities: Vec<Entity>,
    pub base_point: Option<Vector2>,
}

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
    pub clipboard: Clipboard,
    pub dragging_label_index: Option<usize>,
    pub drag_last_pos: Option<Vector2>,
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
            clipboard: Clipboard::default(),
            dragging_label_index: None,
            drag_last_pos: None,
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
}
