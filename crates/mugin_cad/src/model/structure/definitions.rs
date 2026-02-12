use crate::model::structure::column_type::ColumnType;
use crate::model::structure::material::Material;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Holds definitions for structural elements (Materials, Column Types, etc.).
/// These are referenced by ID from actual entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinitions {
    pub materials: HashMap<u64, Material>,
    pub column_types: HashMap<u64, ColumnType>,

    // Counter for generic IDs within definitions
    next_id: u64,
}

impl StructureDefinitions {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            column_types: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // --- Materials ---

    pub fn add_material(&mut self, mut material: Material) -> u64 {
        if material.id == 0 {
            material.id = self.next_id();
        }
        let id = material.id;
        self.materials.insert(id, material);
        id
    }

    pub fn get_material(&self, id: u64) -> Option<&Material> {
        self.materials.get(&id)
    }

    pub fn get_material_mut(&mut self, id: u64) -> Option<&mut Material> {
        self.materials.get_mut(&id)
    }

    pub fn remove_material(&mut self, id: u64) {
        self.materials.remove(&id);
    }

    // --- Column Types ---

    pub fn add_column_type(&mut self, mut col_type: ColumnType) -> u64 {
        if col_type.id == 0 {
            col_type.id = self.next_id();
        }
        let id = col_type.id;
        self.column_types.insert(id, col_type);
        id
    }

    pub fn get_column_type(&self, id: u64) -> Option<&ColumnType> {
        self.column_types.get(&id)
    }

    pub fn get_column_type_mut(&mut self, id: u64) -> Option<&mut ColumnType> {
        self.column_types.get_mut(&id)
    }

    pub fn remove_column_type(&mut self, id: u64) {
        self.column_types.remove(&id);
    }
}
