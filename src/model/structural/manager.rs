use super::types::{BeamType, ColumnType, FloorType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manages structural type templates and instance counters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StructuralTypeManager {
    pub column_types: HashMap<String, ColumnType>,
    pub beam_types: HashMap<String, BeamType>,
    pub floor_types: HashMap<String, FloorType>,

    // Auto-increment counters per type
    #[serde(default)]
    column_counters: HashMap<String, u32>,
    #[serde(default)]
    beam_counters: HashMap<String, u32>,
    #[serde(default)]
    floor_counters: HashMap<String, u32>,
}

impl StructuralTypeManager {
    pub fn new() -> Self {
        Self::default()
    }

    // Column type management
    pub fn add_column_type(&mut self, column_type: ColumnType) {
        self.column_types
            .insert(column_type.id.clone(), column_type);
    }

    pub fn get_column_type(&self, id: &str) -> Option<&ColumnType> {
        self.column_types.get(id)
    }

    pub fn remove_column_type(&mut self, id: &str) -> Option<ColumnType> {
        self.column_counters.remove(id);
        self.column_types.remove(id)
    }

    pub fn next_column_index(&mut self, type_id: &str) -> u32 {
        let counter = self.column_counters.entry(type_id.to_string()).or_insert(0);
        *counter += 1;
        *counter
    }

    // Beam type management
    pub fn add_beam_type(&mut self, beam_type: BeamType) {
        self.beam_types.insert(beam_type.id.clone(), beam_type);
    }

    pub fn get_beam_type(&self, id: &str) -> Option<&BeamType> {
        self.beam_types.get(id)
    }

    pub fn remove_beam_type(&mut self, id: &str) -> Option<BeamType> {
        self.beam_counters.remove(id);
        self.beam_types.remove(id)
    }

    pub fn next_beam_index(&mut self, type_id: &str) -> u32 {
        let counter = self.beam_counters.entry(type_id.to_string()).or_insert(0);
        *counter += 1;
        *counter
    }

    // Floor type management
    pub fn add_floor_type(&mut self, floor_type: FloorType) {
        self.floor_types.insert(floor_type.id.clone(), floor_type);
    }

    pub fn get_floor_type(&self, id: &str) -> Option<&FloorType> {
        self.floor_types.get(id)
    }

    pub fn next_floor_index(&mut self, type_id: &str) -> u32 {
        let counter = self.floor_counters.entry(type_id.to_string()).or_insert(0);
        *counter += 1;
        *counter
    }

    /// Get count of instances for a column type (for display)
    pub fn column_instance_count(&self, type_id: &str) -> u32 {
        *self.column_counters.get(type_id).unwrap_or(&0)
    }

    /// Get count of instances for a beam type
    pub fn beam_instance_count(&self, type_id: &str) -> u32 {
        *self.beam_counters.get(type_id).unwrap_or(&0)
    }
}
