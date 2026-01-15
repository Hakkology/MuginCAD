use crate::commands::executor::CommandExecutor;
use crate::model::CadModel;
use crate::model::Vector2;
use crate::model::snap::{SnapPoint, SnapSystem};
use crate::model::undo::UndoManager;
use crate::view::viewport::Viewport;
use crate::viewmodel::selection::SelectionManager;
use std::path::PathBuf;

pub struct ProjectTab {
    pub name: String,
    pub file_path: Option<PathBuf>,
    pub is_dirty: bool,

    // Project State
    pub model: CadModel,
    pub executor: CommandExecutor,
    pub selection_manager: SelectionManager,
    pub snap_system: SnapSystem,
    pub current_snap: Option<SnapPoint>,
    pub undo_manager: UndoManager,
    pub viewport: Viewport,

    // Interaction State
    pub pending_delete_confirmation: bool,
    pub dragging_label_index: Option<usize>,
    pub drag_last_pos: Option<Vector2>,
}

impl ProjectTab {
    pub fn new(name: String) -> Self {
        Self {
            name,
            file_path: None,
            is_dirty: false,
            model: CadModel::new(),
            executor: CommandExecutor::new(),
            selection_manager: SelectionManager::new(),
            snap_system: SnapSystem::new(),
            current_snap: None,
            undo_manager: UndoManager::new(50),
            viewport: Viewport::new(),
            pending_delete_confirmation: false,
            dragging_label_index: None,
            drag_last_pos: None,
        }
    }
}
