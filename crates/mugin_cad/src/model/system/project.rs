use crate::model::Entity;
use crate::model::axis::Axis;
use crate::model::config::AppConfig;
use crate::model::structure::definitions::StructureDefinitions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectData {
    pub version: String,
    pub entities: Vec<Entity>,
    pub axes: Vec<Axis>,
    pub config: AppConfig,
    pub definitions: StructureDefinitions,
}

impl ProjectData {
    pub fn new(
        entities: Vec<Entity>,
        axes: Vec<Axis>,
        config: AppConfig,
        definitions: StructureDefinitions,
    ) -> Self {
        Self {
            version: "1.0.0".to_string(),
            entities,
            axes,
            config,
            definitions,
        }
    }
}
