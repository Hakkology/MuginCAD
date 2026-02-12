//! Toolbar and menu button helpers.
//!
//! Reduces repeated toolbar/menu button patterns to one-line calls.
//! These are generic — they work with any egui-based application,
//! not just CAD software.

use eframe::egui;

// ─── Toolbar Buttons ─────────────────────────────────────────────────────

/// Renders a square toolbar button with a tooltip.
///
/// Returns `true` if clicked. The button is enabled/disabled via `enabled`.
///
/// # Example
///
/// ```rust
/// if mugin_widgets::toolbar::icon_button(ui, "M", "Move (W)", true, [36.0, 36.0]) {
///     start_move_command();
/// }
/// ```
pub fn icon_button(
    ui: &mut egui::Ui,
    label: &str,
    tooltip: &str,
    enabled: bool,
    size: [f32; 2],
) -> bool {
    ui.add_enabled(
        enabled,
        egui::Button::new(label).min_size(egui::vec2(size[0], size[1])),
    )
    .on_hover_text(tooltip)
    .clicked()
}

/// Renders a toolbar button with default 36×36 size.
pub fn tool_button(ui: &mut egui::Ui, label: &str, tooltip: &str, enabled: bool) -> bool {
    icon_button(ui, label, tooltip, enabled, [36.0, 36.0])
}

// ─── Menu Items ──────────────────────────────────────────────────────────

/// Renders a menu item button. Returns `true` if clicked.
///
/// When clicked, also closes the parent menu automatically.
///
/// # Example
///
/// ```rust
/// if mugin_widgets::toolbar::menu_item(ui, "Line (L)", true) {
///     start_line_command();
/// }
/// ```
pub fn menu_item(ui: &mut egui::Ui, label: &str, enabled: bool) -> bool {
    let clicked = ui.add_enabled(enabled, egui::Button::new(label)).clicked();
    if clicked {
        ui.close_menu();
    }
    clicked
}

/// Renders a simple menu button (always enabled). Returns `true` if clicked.
pub fn menu_action(ui: &mut egui::Ui, label: &str) -> bool {
    menu_item(ui, label, true)
}

// ─── Toolbar Layout Helpers ──────────────────────────────────────────────

/// Adds a visual separator with spacing in a toolbar.
pub fn separator(ui: &mut egui::Ui) {
    ui.separator();
    ui.add_space(4.0);
}

/// Adds a labeled section header in a menu.
pub fn menu_section(ui: &mut egui::Ui, title: &str) {
    ui.add_space(8.0);
    ui.label(title);
    ui.separator();
}
