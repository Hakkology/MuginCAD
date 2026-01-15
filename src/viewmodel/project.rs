use crate::model::undo::UndoManager;
use crate::viewmodel::CadViewModel;

impl CadViewModel {
    /// Save project to a file
    pub fn save_project(&mut self) {
        let tab_idx = self.active_tab_index;
        let default_name = format!("{}.oliv", self.tabs[tab_idx].name);
        let tab = &mut self.tabs[tab_idx];

        if let Some(mut path) = rfd::FileDialog::new()
            .add_filter("OliveCAD Project", &["oliv"])
            .set_file_name(&default_name)
            .save_file()
        {
            // Ensure extension is present
            if path.extension().and_then(|ext| ext.to_str()) != Some("oliv") {
                path.set_extension("oliv");
            }

            // We need to access self.config. self.tabs is borrowed by 'tab'.
            // self.config is disjoint, so this is valid.
            let project_data = crate::model::project::ProjectData::new(
                tab.model.entities.clone(),
                tab.model.axis_manager.axes.clone(),
                self.config.clone(),
            );

            if let Ok(json) = serde_json::to_string_pretty(&project_data) {
                if std::fs::write(&path, json).is_ok() {
                    tab.file_path = Some(path.clone());
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        tab.name = name.to_string();
                    }
                    tab.is_dirty = false;
                    self.command_history
                        .push(format!("Saved project to {:?}", path));
                }
            }
        }
    }

    /// Load project from a file
    pub fn load_project(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("OliveCAD Project", &["oliv"])
            .pick_file()
        {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(project_data) =
                    serde_json::from_str::<crate::model::project::ProjectData>(&content)
                {
                    // If current tab is active, we check its state
                    let should_new_tab = {
                        let tab = &self.tabs[self.active_tab_index];
                        !tab.model.entities.is_empty() || tab.is_dirty || tab.file_path.is_some()
                    };

                    if should_new_tab {
                        self.new_tab();
                    }

                    // Re-borrow active tab
                    let tab_idx = self.active_tab_index;
                    let tab = &mut self.tabs[tab_idx];

                    tab.model.entities = project_data.entities;
                    tab.model.axis_manager.axes = project_data.axes;
                    self.config = project_data.config;

                    // Reset transient state
                    tab.undo_manager = UndoManager::new(50);
                    tab.selection_manager.selected_indices.clear();
                    tab.current_snap = None;
                    tab.executor.cancel();

                    tab.file_path = Some(path.clone());
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        tab.name = name.to_string();
                    }
                    tab.is_dirty = false;

                    self.command_history
                        .push(format!("Loaded project from {:?}", path));
                }
            }
        }
    }
}
