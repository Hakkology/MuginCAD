//! Property editor widgets for inspector panels and settings windows.
//!
//! Each widget renders a labeled, grouped editor inside an `egui::Ui`.
//! They are designed for property panels where you need to edit
//! coordinates, lengths, angles, colors, and boolean flags.

use eframe::egui;

// ─── Point Editors ───────────────────────────────────────────────────────

/// Renders a labeled X/Y coordinate editor group.
///
/// ```text
/// ┌ Position ──────────┐
/// │ X: [123.4]  Y: [56.7] │
/// └────────────────────┘
/// ```
pub fn point2(ui: &mut egui::Ui, label: &str, x: &mut f32, y: &mut f32) {
    ui.group(|ui| {
        ui.label(label);
        ui.horizontal(|ui| {
            ui.label("X:");
            ui.add(egui::DragValue::new(x).speed(0.1));
            ui.label("Y:");
            ui.add(egui::DragValue::new(y).speed(0.1));
        });
    });
}

/// Same as [`point2`] but with a custom drag speed.
pub fn point2_speed(ui: &mut egui::Ui, label: &str, x: &mut f32, y: &mut f32, speed: f64) {
    ui.group(|ui| {
        ui.label(label);
        ui.horizontal(|ui| {
            ui.label("X:");
            ui.add(egui::DragValue::new(x).speed(speed));
            ui.label("Y:");
            ui.add(egui::DragValue::new(y).speed(speed));
        });
    });
}

// ─── Scalar Editors ──────────────────────────────────────────────────────

/// Renders a labeled drag-value for a single float.
///
/// ```text
/// Radius: [25.0]
/// ```
pub fn float_value(ui: &mut egui::Ui, label: &str, value: &mut f32, speed: f64) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value).speed(speed));
    });
}

/// Renders a labeled drag-value with a min..max range.
///
/// Useful for constrained properties like radius (0..∞) or font size (6..72).
pub fn float_range(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    speed: f64,
    range: std::ops::RangeInclusive<f32>,
) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(value).speed(speed).range(range));
    });
}

// ─── Angle Editor ────────────────────────────────────────────────────────

/// Renders an angle editor that displays degrees but stores radians.
///
/// ```text
/// Rotation: [45.0°]
/// ```
pub fn angle_degrees(ui: &mut egui::Ui, label: &str, radians: &mut f32) {
    ui.horizontal(|ui| {
        ui.label(label);
        let mut degrees = radians.to_degrees();
        if ui
            .add(egui::DragValue::new(&mut degrees).speed(1.0).suffix("°"))
            .changed()
        {
            *radians = degrees.to_radians();
        }
    });
}

// ─── Color Editor ────────────────────────────────────────────────────────

/// Renders a labeled color picker for an `[u8; 3]` RGB color.
///
/// ```text
/// Background Color: [■]
/// ```
pub fn color_rgb(ui: &mut egui::Ui, label: &str, color: &mut [u8; 3]) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.color_edit_button_srgb(color);
    });
}

// ─── Section / Group ─────────────────────────────────────────────────────

/// Renders a titled section with a bold label, spacing, and a body closure.
///
/// ```text
/// ┌ History ──────────────┐
/// │  (your content here)  │
/// └───────────────────────┘
/// ```
pub fn section(ui: &mut egui::Ui, title: &str, body: impl FnOnce(&mut egui::Ui)) {
    ui.group(|ui| {
        ui.label(egui::RichText::new(title).strong());
        ui.separator();
        ui.add_space(5.0);
        body(ui);
    });
}

/// Renders a collapsible section — useful for settings panels.
pub fn collapsible_section(ui: &mut egui::Ui, title: &str, body: impl FnOnce(&mut egui::Ui)) {
    ui.collapsing(title, |ui| {
        ui.group(|ui| {
            body(ui);
        });
    });
}

// ─── Text Input ─────────────────────────────────────────────────────────

/// Renders a labeled text input field.
/// Returns the `egui::Response` to allow for focus tracking.
pub fn text_input(ui: &mut egui::Ui, label: &str, value: &mut String) -> egui::Response {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(value)
    })
    .inner
}

// ─── Checkbox ────────────────────────────────────────────────────────────

/// Renders a labeled checkbox.
pub fn toggle(ui: &mut egui::Ui, label: &str, value: &mut bool) {
    ui.checkbox(value, label);
}

// ─── Read-only Display ───────────────────────────────────────────────────

/// Renders a labeled read-only value (e.g., computed length).
///
/// ```text
/// Length: 142.85
/// ```
pub fn display_value(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label(value);
    });
}

/// Renders a labeled read-only float with a given precision.
pub fn display_float(ui: &mut egui::Ui, label: &str, value: f32, decimals: usize) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label(format!("{:.prec$}", value, prec = decimals));
    });
}
