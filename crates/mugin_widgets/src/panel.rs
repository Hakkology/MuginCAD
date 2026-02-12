//! Panel helpers for consistent UI layout.

use eframe::egui;

/// Styling configuration for panels
const PANEL_FILL: egui::Color32 = egui::Color32::from_rgb(25, 25, 25);
const PANEL_MARGIN: f32 = 8.0;

/// Renders a standardized left side panel.
///
/// # Arguments
/// * `id` - Unique ID for the panel
/// * `default_width` - Default width of the panel
/// * `ctx` - Egui context
/// * `body` - Closure to render panel content
pub fn left_panel(
    id: impl Into<egui::Id>,
    default_width: f32,
    ctx: &egui::Context,
    body: impl FnOnce(&mut egui::Ui),
) {
    egui::SidePanel::left(id)
        .resizable(true)
        .default_width(default_width)
        .frame(
            egui::Frame::none()
                .fill(PANEL_FILL)
                .inner_margin(PANEL_MARGIN),
        )
        .show(ctx, body);
}

/// Renders a standardized right side panel.
///
/// # Arguments
/// * `id` - Unique ID for the panel
/// * `default_width` - Default width of the panel
/// * `ctx` - Egui context
/// * `body` - Closure to render panel content
pub fn right_panel(
    id: impl Into<egui::Id>,
    default_width: f32,
    ctx: &egui::Context,
    body: impl FnOnce(&mut egui::Ui),
) {
    egui::SidePanel::right(id)
        .resizable(true)
        .default_width(default_width)
        .frame(
            egui::Frame::none()
                .fill(PANEL_FILL)
                .inner_margin(PANEL_MARGIN),
        )
        .show(ctx, body);
}
