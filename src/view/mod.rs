pub mod canvas;
pub mod rendering;
pub mod ui;
pub mod viewport;

// Re-export for convenience if needed, or update call sites.
// For now, let's keep them accessible via the new paths or re-export.
// Ideally usage should be `view::ui::inspector`.
// But to break less code initially, we can re-export.
pub use ui::inspector;
pub use ui::settings;
pub use ui::terminal;
pub use ui::toolbar;
pub use ui::topmenu;

pub use rendering::context;
pub use rendering::preview;
pub use rendering::renderer;

use crate::viewmodel::CadViewModel;
use eframe::egui;

pub struct CadApp {
    pub view_model: CadViewModel,
}

impl CadApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            view_model: CadViewModel::new(),
        }
    }
}

impl eframe::App for CadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark theme from config
        let mut visuals = egui::Visuals::dark();
        let bg_color = self.view_model.config.appearance_config.background_color;
        visuals.widgets.noninteractive.bg_fill =
            egui::Color32::from_rgb(bg_color[0], bg_color[1], bg_color[2]);
        ctx.set_visuals(visuals);

        // Render Settings Window if open
        if self.view_model.show_settings_window {
            settings::render_settings_window(ctx, &mut self.view_model);
        }

        // Top Menu
        topmenu::render_top_menu(ctx, &mut self.view_model);

        // Global shortcuts

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.view_model.cancel_command();
        }

        // Reset viewport with End key
        if ctx.input(|i| i.key_pressed(egui::Key::End)) {
            self.view_model.viewport.reset();
        }

        // Delete selected entity with Delete key
        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
            self.view_model.delete_selected();
        }

        // Copy with Ctrl+C
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::C)) {
            if !self.view_model.selected_indices.is_empty() && !self.view_model.executor.is_active()
            {
                self.view_model.executor.start_command(
                    "copy",
                    &mut self.view_model.model,
                    &self.view_model.selected_indices,
                );
            }
        }

        // Cut with Ctrl+X
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::X)) {
            if !self.view_model.selected_indices.is_empty() && !self.view_model.executor.is_active()
            {
                self.view_model.executor.start_command(
                    "cut",
                    &mut self.view_model.model,
                    &self.view_model.selected_indices,
                );
            }
        }

        // Left Toolbar Panel
        toolbar::render_toolbar(ctx, &mut self.view_model);

        // Inspector Panel Logic

        let show_inspector = self.view_model.config.gui_config.show_inspector_always
            || !self.view_model.selected_indices.is_empty();

        if show_inspector {
            egui::SidePanel::right("inspector")
                .resizable(true)
                .default_width(250.0)
                .frame(
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(25, 25, 25))
                        .inner_margin(10.0),
                )
                .show(ctx, |ui| {
                    inspector::render_inspector(ui, &mut self.view_model);
                });
        }

        egui::TopBottomPanel::bottom("terminal")
            .resizable(true)
            .default_height(200.0)
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30, 30, 30))
                    .inner_margin(5.0),
            )
            .show(ctx, |ui| {
                inspector::render_selection_status(ui, &mut self.view_model);
                ui.separator();
                terminal::render_terminal(ui, &mut self.view_model);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 15)))
            .show(ctx, |ui| {
                canvas::render_canvas(ui, &mut self.view_model);
            });
    }
}
