use crate::model::structure::material::Material;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Holds definitions for structural elements (Materials, Column Types, etc.).
/// These are referenced by ID from actual entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinitions {
    pub materials: HashMap<u64, Material>,
    // Future: column_types: HashMap<u64, ColumnType>,

    // Counter for generic IDs within definitions
    next_id: u64,
}

impl StructureDefinitions {
    pub fn new() -> Self {
        let mut defs = Self {
            materials: HashMap::new(),
            next_id: 1,
        };

        // Add default materials
        defs.add_material(Material::new_concrete(0, "C20/25 Concrete", "C20"));
        defs.add_material(Material::new_steel(0, "S420 Steel", "S420", Some(12.0)));

        defs
    }

    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

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
}
