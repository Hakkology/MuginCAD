use crate::model::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    A3,
    Custom(f32, f32), // Width, Height in mm
}

impl PageSize {
    pub fn dimensions_mm(&self) -> (f32, f32) {
        match self {
            PageSize::A4 => (210.0, 297.0),
            PageSize::A3 => (297.0, 420.0),
            PageSize::Custom(w, h) => (*w, *h),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ScaleType {
    FitToPage,
    Standard(f32), // 1:x (e.g., 50.0 for 1:50)
    Custom(f32),   // 1:x
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExportSource {
    ModelBounds,
    Viewport(Vector2, Vector2), // Min, Max
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub page_size: PageSize,
    pub orientation: PageOrientation,
    pub scale_type: ScaleType,
    pub source: ExportSource,
    pub margin_mm: f32, // Margin
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            orientation: PageOrientation::Landscape,
            scale_type: ScaleType::FitToPage,
            source: ExportSource::ModelBounds,
            margin_mm: 10.0,
        }
    }
}
