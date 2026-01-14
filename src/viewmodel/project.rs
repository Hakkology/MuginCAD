use crate::model::config::AppConfig;
use crate::model::undo::UndoManager;
use crate::viewmodel::CadViewModel;

impl CadViewModel {
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
}
