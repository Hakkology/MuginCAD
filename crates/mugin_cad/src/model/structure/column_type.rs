use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnType {
    pub id: u64,
    pub name: String,

    // --- Geometry ---
    /// Width in cm
    pub width: f32,
    /// Depth in cm
    pub depth: f32,

    // --- Materials ---
    /// Reference to Concrete Material (e.g., C25)
    pub concrete_material_id: u64,
    /// Reference to Steel Material for Rebar (e.g., S420)
    pub rebar_material_id: u64,

    // --- Longitudinal Reinforcement (Boyuna Donatı) ---
    pub long_bar_diameter: f32, // mm (e.g., 14.0)
    pub long_bars_x: u32,       // Count along width face (e.g., 3 means 3 bars on top/bottom)
    pub long_bars_y: u32, // Count along depth face (e.g., 2 means 2 bars on left/right between corners)

    // --- Transverse Reinforcement (Etriye) ---
    pub stirrup_diameter: f32,     // mm (e.g., 8.0)
    pub stirrup_spacing_supp: f32, // cm (Sklaştırma bölgesi)
    pub stirrup_spacing_mid: f32,  // cm (Orta bölge)
    pub has_ties: bool,            // Çiroz var mı?

    /// Optional override color
    pub color_override: Option<(u8, u8, u8, u8)>,
}

impl ColumnType {
    pub fn new(
        id: u64,
        name: impl Into<String>,
        width: f32,
        depth: f32,
        concrete_material_id: u64,
        rebar_material_id: u64,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            width,
            depth,
            concrete_material_id,
            rebar_material_id,
            long_bar_diameter: 14.0,
            long_bars_x: 3,
            long_bars_y: 3,
            stirrup_diameter: 8.0,
            stirrup_spacing_supp: 10.0,
            stirrup_spacing_mid: 20.0,
            has_ties: true,
            color_override: None,
        }
    }
}
