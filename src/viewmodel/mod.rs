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

/// Clipboard for copy/cut/paste operations
#[derive(Default)]
pub struct Clipboard {
    pub entities: Vec<Entity>,
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
    pub clipboard: Clipboard,
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
            clipboard: Clipboard::default(),
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
}
