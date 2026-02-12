use serde::{Deserialize, Serialize};

/// Categories of structural materials.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MaterialCategory {
    Concrete,
    Steel,
    Timber,
    Other,
}

/// Detailed properties for specific material types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialProperties {
    /// Concrete properties (e.g., C20, C30)
    Concrete {
        class: String,
        // Future: compressive_strength, etc.
    },
    /// Steel properties (e.g., S420, S500)
    Steel {
        grade: String,
        /// If this material represents a specific rebar, it might have a diameter.
        /// Option because generic "Steel" might not have a diameter.
        diameter_mm: Option<f32>,
    },
    /// Generic properties for other materials
    Generic,
}

/// A material definition that can be assigned to structural elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: u64,
    pub name: String,
    pub category: MaterialCategory,
    pub properties: MaterialProperties,
    /// Hatch pattern name for 2D representation (e.g., "ANSI31", "SOLID")
    pub hatch_pattern: String,
    /// Color for rendering (r, g, b, a)
    pub color: (u8, u8, u8, u8),
}

impl Material {
    pub fn new_concrete(id: u64, name: &str, class: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            category: MaterialCategory::Concrete,
            properties: MaterialProperties::Concrete {
                class: class.to_string(),
            },
            hatch_pattern: "ANSI37".to_string(), // Cross-hatch often used for concrete
            color: (128, 128, 128, 255),
        }
    }

    pub fn new_steel(id: u64, name: &str, grade: &str, diameter: Option<f32>) -> Self {
        Self {
            id,
            name: name.to_string(),
            category: MaterialCategory::Steel,
            properties: MaterialProperties::Steel {
                grade: grade.to_string(),
                diameter_mm: diameter,
            },
            hatch_pattern: "ANSI31".to_string(), // Diagonal lines for steel
            color: (70, 130, 180, 255),          // Steel Blue
        }
    }
}
