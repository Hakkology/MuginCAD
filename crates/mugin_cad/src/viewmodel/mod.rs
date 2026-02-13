//! View Model Layer (MVVM)
//!
//! This module contains the state management and business logic for the application.
//! It acts as a bridge between the data model (`crate::model`) and the view (`crate::view`).
//!
//! Key components:
//! - `CadViewModel`: The main state container.
//! - `ProjectTab`: Manages state for a single open project (tab).
//! - `InputManager`: Handling raw input events.
//! - `CommandManager`: Managing the command execution and undo/redo history.
//!
//! The view model is responsible for:
//! 1. Maintaining the state of the application.
//! 2. Processing user inputs and executing commands.
//! 3. Converting model data into a format suitable for rendering (though direct rendering is handled in `view`).

mod commands;
mod history;
// mod index_helper;
mod input;
mod project;
mod selection;
mod snap;
pub mod tab;

use self::tab::ProjectTab;
use crate::commands::InputModifiers;
use crate::model::config::AppConfig;
use crate::model::{Entity, Vector2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftPanelTab {
    Hierarchy,
    Layers,
}

pub struct PendingLayerChange {
    pub entity_ids: Vec<u64>,
    pub new_layer_id: u64,
}

/// Clipboard for copy/cut/paste operations
#[derive(Default)]
pub struct Clipboard {
    #[allow(dead_code)]
    pub entities: Vec<Entity>,
    #[allow(dead_code)]
    pub base_point: Option<Vector2>,
}

pub struct CadViewModel {
    pub tabs: Vec<ProjectTab>,
    pub active_tab_index: usize,

    // Global State
    pub command_input: String,
    pub command_history: Vec<String>,
    pub history_nav_index: Option<usize>,
    pub config: AppConfig,
    pub show_settings_window: bool,
    pub tab_renaming_index: Option<usize>,
    pub hierarchy_renaming: bool,
    pub inspector_renaming: bool,
    pub materials_manager_open: bool,
    pub column_manager_open: bool,
    pub beam_manager_open: bool,
    pub active_column_type_id: Option<u64>,
    pub active_beam_type_id: Option<u64>,
    pub layer_change_prompt: Option<PendingLayerChange>,
    #[allow(dead_code)]
    pub clipboard: Clipboard,
    pub export_window: crate::view::ui::export::window::ExportWindow,
    pub active_left_panel_tab: LeftPanelTab,
}

impl CadViewModel {
    pub fn new() -> Self {
        let default_tab = ProjectTab::new("Untitled".to_string());

        Self {
            tabs: vec![default_tab],
            active_tab_index: 0,
            command_input: String::new(),
            command_history: Vec::new(),
            history_nav_index: None,
            config: AppConfig::default(),
            show_settings_window: false,
            tab_renaming_index: None,
            hierarchy_renaming: false,
            inspector_renaming: false,
            materials_manager_open: false,
            column_manager_open: false,
            beam_manager_open: false,
            active_column_type_id: None,
            active_beam_type_id: None,
            layer_change_prompt: None,
            clipboard: Clipboard::default(),
            export_window: crate::view::ui::export::window::ExportWindow::default(),
            active_left_panel_tab: LeftPanelTab::Hierarchy,
        }
    }

    pub fn active_tab(&self) -> &ProjectTab {
        &self.tabs[self.active_tab_index]
    }

    pub fn active_tab_mut(&mut self) -> &mut ProjectTab {
        &mut self.tabs[self.active_tab_index]
    }

    pub fn new_tab(&mut self) {
        let name = format!("Untitled {}", self.tabs.len() + 1);
        self.tabs.push(ProjectTab::new(name));
        self.active_tab_index = self.tabs.len() - 1;
    }

    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            // Don't close the last tab, just reset it? Or allow closing app?
            // For now, let's just create a new empty one if we close the last one
            self.tabs.remove(index);
            self.tabs.push(ProjectTab::new("Untitled".to_string()));
            self.active_tab_index = 0;
        } else {
            self.tabs.remove(index);
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            } else if self.active_tab_index > index {
                // If we closed a tab before the active one, shift index
                self.active_tab_index -= 1;
            }
        }
    }

    /// Get status message from active executor
    pub fn status_message(&self) -> &str {
        &self.active_tab().executor.status_message
    }

    /// Get active tab and history mutably simultaneously (to satisfy borrow checker)
    pub fn active_tab_mut_and_history(&mut self) -> (&mut ProjectTab, &mut Vec<String>) {
        (
            &mut self.tabs[self.active_tab_index],
            &mut self.command_history,
        )
    }

    /// Update keyboard modifiers (called from view)
    pub fn set_modifiers(&mut self, modifiers: InputModifiers) {
        self.active_tab_mut().executor.set_modifiers(modifiers);
    }

    pub fn apply_layer_change(&mut self, recursive: bool) {
        if let Some(change) = self.layer_change_prompt.take() {
            let tab = self.active_tab_mut();
            for id in change.entity_ids {
                if let Some(e) = tab.model.find_by_id_mut(id) {
                    if recursive {
                        Self::set_layer_recursive(e, change.new_layer_id);
                    } else {
                        e.layer_id = change.new_layer_id;
                    }
                }
            }
        }
    }

    fn set_layer_recursive(entity: &mut Entity, layer_id: u64) {
        entity.layer_id = layer_id;
        for child in &mut entity.children {
            Self::set_layer_recursive(child, layer_id);
        }
    }

    pub fn cancel_layer_change(&mut self) {
        self.layer_change_prompt = None;
    }
}
