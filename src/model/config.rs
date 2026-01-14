use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Snap configuration
    pub snap_config: SnapConfig,
    /// Grid configuration
    pub grid_config: GridConfig,
    /// Appearance configuration
    pub appearance_config: AppearanceConfig,
    /// GUI configuration
    pub gui_config: GuiConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            snap_config: SnapConfig::default(),
            grid_config: GridConfig::default(),
            appearance_config: AppearanceConfig::default(),
            gui_config: GuiConfig::default(),
        }
    }
}

// ... existing structs ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiConfig {
    /// Always show inspector panel regardless of selection
    pub show_inspector_always: bool,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            show_inspector_always: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapConfig {
    /// Snap tolerance in CAD units
    pub tolerance: f32,
    /// Enable snapping to grid
    pub snap_to_grid: bool,
    /// Enable snapping to endpoints
    pub snap_to_endpoint: bool,
    /// Enable snapping to midpoint
    pub snap_to_midpoint: bool,
    /// Enable snapping to center
    pub snap_to_center: bool,
    /// Enable snapping to intersection
    pub snap_to_intersection: bool,
    /// Enable snapping to axis lines
    pub snap_to_axis: bool,
}

impl Default for SnapConfig {
    fn default() -> Self {
        Self {
            tolerance: 15.0,
            snap_to_grid: false,
            snap_to_endpoint: true,
            snap_to_midpoint: true,
            snap_to_center: true,
            snap_to_intersection: true,
            snap_to_axis: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    /// Grid spacing size
    pub grid_size: f32,
    /// Show grid on canvas
    pub show_grid: bool,
    /// Grid color (RGB)
    pub grid_color: [u8; 3],
    /// Axis color (RGB)
    pub axis_color: [u8; 3],
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            grid_size: 50.0,
            show_grid: true,
            grid_color: [40, 40, 40],
            axis_color: [80, 80, 80],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    /// Background color (RGB)
    pub background_color: [u8; 3],
    /// Selection color (RGB)
    pub selection_color: [u8; 3],
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            background_color: [15, 15, 15],
            selection_color: [255, 215, 0], // Gold
        }
    }
}
