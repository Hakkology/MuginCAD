//! Window and dialog helpers.
//!
//! Simplifies creating modal dialogs, centered popups, and resizable
//! windows that all follow the same boilerplate pattern.

use eframe::egui;

// ─── Modal Dialog ────────────────────────────────────────────────────────

/// Shows a centered modal dialog. Returns `true` when the dialog closes.
///
/// The `body` closure should render the dialog content. Return `true`
/// from the body to signal that the dialog should close (e.g., when
/// the user presses "OK" or hits Enter).
///
/// # Example
///
/// ```rust
/// let closed = mugin_widgets::window::modal("Rename", ctx, &mut is_open, |ui| {
///     ui.label("Enter new name:");
///     let r = ui.text_edit_singleline(&mut name);
///     r.request_focus();
///     ui.button("OK").clicked() // returns true → closes dialog
/// });
/// ```
pub fn modal(
    title: &str,
    ctx: &egui::Context,
    open: &mut bool,
    body: impl FnOnce(&mut egui::Ui) -> bool,
) -> bool {
    if !*open {
        return false;
    }

    let mut should_close = false;

    let mut window_open = true;
    egui::Window::new(title)
        .open(&mut window_open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -100.0))
        .show(ctx, |ui| {
            if body(ui) {
                should_close = true;
            }
        });

    // Window X button was clicked
    if !window_open {
        should_close = true;
    }

    if should_close {
        *open = false;
    }

    should_close
}

// ─── Resizable Dialog ────────────────────────────────────────────────────

/// Shows a resizable dialog window with a fixed minimum size.
///
/// Unlike [`modal`], this window can be resized and moved freely.
/// The `body` closure returns `true` to close the window.
///
/// # Example
///
/// ```rust
/// mugin_widgets::window::dialog("Export PDF", ctx, &mut is_open, [800.0, 600.0], |ui| {
///     // render settings and preview...
///     ui.button("Cancel").clicked()
/// });
/// ```
pub fn dialog(
    title: &str,
    ctx: &egui::Context,
    open: &mut bool,
    size: [f32; 2],
    body: impl FnOnce(&mut egui::Ui) -> bool,
) -> bool {
    if !*open {
        return false;
    }

    let mut should_close = false;

    egui::Window::new(title)
        .open(open)
        .resize(|r| r.fixed_size(size))
        .show(ctx, |ui| {
            if body(ui) {
                should_close = true;
            }
        });

    if should_close {
        *open = false;
    }

    should_close
}

// ─── Confirmation Dialog ─────────────────────────────────────────────────

/// A result from a confirmation dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmResult {
    /// Dialog is still open, no decision made.
    Pending,
    /// User confirmed (clicked "OK" / "Yes").
    Confirmed,
    /// User cancelled (clicked "Cancel" / "No" / closed window).
    Cancelled,
}

/// Shows a confirmation dialog with a message and OK/Cancel buttons.
///
/// # Example
///
/// ```rust
/// match mugin_widgets::window::confirm("Delete?", ctx, &mut open, "Are you sure?") {
///     ConfirmResult::Confirmed => { delete_entity(); }
///     ConfirmResult::Cancelled => { /* nothing */ }
///     ConfirmResult::Pending => { /* still showing */ }
/// }
/// ```
pub fn confirm(title: &str, ctx: &egui::Context, open: &mut bool, message: &str) -> ConfirmResult {
    if !*open {
        return ConfirmResult::Pending;
    }

    let mut result = ConfirmResult::Pending;

    let closed = modal(title, ctx, open, |ui| {
        ui.label(message);
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("OK").clicked() {
                result = ConfirmResult::Confirmed;
            }
            if ui.button("Cancel").clicked() {
                result = ConfirmResult::Cancelled;
            }
        });
        result != ConfirmResult::Pending
    });

    // If window was closed via X button
    if closed && result == ConfirmResult::Pending {
        result = ConfirmResult::Cancelled;
    }

    result
}
