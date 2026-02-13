use eframe::egui::Color32;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A logical layer that groups entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: u64,
    pub name: String,
    pub color: Color32,
    pub is_visible: bool,
}

impl Layer {
    pub fn new(id: u64, name: String, color: Color32) -> Self {
        Self {
            id,
            name,
            color,
            is_visible: true,
        }
    }
}

/// Manages the collection of layers.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerManager {
    pub layers: HashMap<u64, Layer>,
    pub active_layer_id: u64,
    next_id: u64,
}

impl LayerManager {
    pub fn new() -> Self {
        let mut layers = HashMap::new();
        // Create default layer (ID 0)
        let default_layer = Layer::new(0, "Default".to_string(), Color32::WHITE);
        layers.insert(0, default_layer);

        Self {
            layers,
            active_layer_id: 0,
            next_id: 1,
        }
    }

    pub fn add_layer(&mut self, name: String, color: Color32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let layer = Layer::new(id, name, color);
        self.layers.insert(id, layer);
        id
    }

    pub fn remove_layer(&mut self, id: u64) {
        if id == 0 {
            return; // Cannot remove default layer
        }
        if self.active_layer_id == id {
            self.active_layer_id = 0; // Reset active to default
        }
        self.layers.remove(&id);
    }

    pub fn get_layer(&self, id: u64) -> Option<&Layer> {
        self.layers.get(&id)
    }

    pub fn set_active_layer(&mut self, id: u64) {
        if self.layers.contains_key(&id) {
            self.active_layer_id = id;
        }
    }

    /// Get sorted list of layers (by ID for now, or alphabetical)
    pub fn get_sorted_layers(&self) -> Vec<&Layer> {
        let mut list: Vec<&Layer> = self.layers.values().collect();
        list.sort_by_key(|l| l.id);
        list
    }
}
