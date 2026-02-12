//! # Mugin Widgets
//!
//! A reusable egui widget toolkit for CAD and engineering applications.
//!
//! Provides high-level helpers to eliminate boilerplate when building
//! egui-based desktop applications:
//!
//! - **`window`** — Modal dialogs and resizable windows
//! - **`properties`** — Property editors (point, float, angle, color)
//! - **`toolbar`** — Toolbar buttons and menu items
//!
//! ## Quick Start
//!
//! ```rust
//! use mugin_widgets::properties;
//! use mugin_widgets::window;
//!
//! // Edit a 2D point with labeled X/Y drag values
//! properties::point2(ui, "Position", &mut x, &mut y);
//!
//! // Show a centered modal dialog
//! window::modal("Rename", ctx, &mut open, |ui| {
//!     ui.text_edit_singleline(&mut name);
//!     ui.button("OK").clicked()
//! });
//! ```

pub mod properties;
pub mod toolbar;
pub mod window;
