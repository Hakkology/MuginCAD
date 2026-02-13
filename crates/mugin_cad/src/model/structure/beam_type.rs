use serde::{Deserialize, Serialize};

/// Defines reinforcement parameters for a specific zone along the beam length.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeamRebarZone {
    /// Diameter of ties in mm.
    pub tie_diameter: f32,
    /// Spacing of ties in cm.
    pub tie_spacing: f32,
}

/// Structural template for a Beam, including reinforcement and material data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeamType {
    pub id: u64,
    pub name: String,

    // --- Geometry ---
    /// Width in cm
    pub width: f32,
    /// Height in cm
    pub height: f32,

    // --- Materials ---
    pub concrete_material_id: u64,
    pub steel_material_id: u64,

    // --- Longitudinal Reinforcement (Boyuna Donatı) ---
    pub top_bar_diameter: f32,
    pub top_bar_count: u32,
    pub bottom_bar_diameter: f32,
    pub bottom_bar_count: u32,
    pub side_bar_diameter: f32,
    pub side_bar_count: u32, // Count per side face

    // --- Transverse Reinforcement (Ties / Etriye) ---
    /// Support Left (Zone A)
    pub zone_left: BeamRebarZone,
    /// Span / Mid (Zone B)
    pub zone_mid: BeamRebarZone,
    /// Support Right (Zone C)
    pub zone_right: BeamRebarZone,

    /// Ratio of beam length for support zones (e.g., 0.25 means L/4)
    pub support_zone_ratio: f32,

    /// Optional override color
    pub color_override: Option<(u8, u8, u8, u8)>,
}

impl BeamType {
    pub fn new(
        id: u64,
        name: impl Into<String>,
        width: f32,
        height: f32,
        concrete_id: u64,
        steel_id: u64,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            width,
            height,
            concrete_material_id: concrete_id,
            steel_material_id: steel_id,
            top_bar_diameter: 14.0,
            top_bar_count: 3,
            bottom_bar_diameter: 14.0,
            bottom_bar_count: 3,
            side_bar_diameter: 10.0,
            side_bar_count: 0,
            zone_left: BeamRebarZone {
                tie_diameter: 8.0,
                tie_spacing: 10.0,
            },
            zone_mid: BeamRebarZone {
                tie_diameter: 8.0,
                tie_spacing: 20.0,
            },
            zone_right: BeamRebarZone {
                tie_diameter: 8.0,
                tie_spacing: 10.0,
            },
            support_zone_ratio: 0.25, // L/4
            color_override: None,
        }
    }
}
