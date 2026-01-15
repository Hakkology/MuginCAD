use serde::{Deserialize, Serialize};

/// Column type template - defines dimensions for a column category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnType {
    pub id: String,     // e.g., "30x50"
    pub width: f32,     // X dimension (mm)
    pub depth: f32,     // Y dimension (mm)
    pub color: [u8; 3], // RGB fill color
}

impl ColumnType {
    pub fn new(id: &str, width: f32, depth: f32) -> Self {
        Self {
            id: id.to_string(),
            width,
            depth,
            color: [128, 128, 128], // Default gray
        }
    }
}

/// Beam type template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeamType {
    pub id: String,  // e.g., "25x40"
    pub width: f32,  // Beam width
    pub height: f32, // Beam height (structural depth)
    pub color: [u8; 3],
}

impl BeamType {
    pub fn new(id: &str, width: f32, height: f32) -> Self {
        Self {
            id: id.to_string(),
            width,
            height,
            color: [100, 100, 100],
        }
    }
}

/// Floor type template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorType {
    pub id: String,     // e.g., "150mm"
    pub thickness: f32, // Slab thickness
    pub color: [u8; 3],
}

impl FloorType {
    pub fn new(id: &str, thickness: f32) -> Self {
        Self {
            id: id.to_string(),
            thickness,
            color: [180, 180, 180],
        }
    }
}
