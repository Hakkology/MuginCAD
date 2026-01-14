pub mod canvas;
pub mod inspector;
pub mod settings;
pub mod terminal;
pub mod viewport;

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
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            // Add some padding
            ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 5.0);

            egui::menu::bar(ui, |ui| {
                ui.menu_button("Project", |ui| {
                    ui.set_min_width(120.0);
                    if ui.button("New").clicked() {
                        // TODO: Implement New Project
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        // TODO: Implement Save
                        ui.close_menu();
                    }
                    if ui.button("Load").clicked() {
                        // TODO: Implement Load
                        ui.close_menu();
                    }
                });

                ui.menu_button("Tools", |ui| {
                    ui.set_min_width(120.0);
                    if ui.button("Settings").clicked() {
                        self.view_model.show_settings_window = true;
                        ui.close_menu();
                    }
                });
            });
        });

        // Global shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.view_model.cancel_command();
        }

        // Reset viewport with End key
        if ctx.input(|i| i.key_pressed(egui::Key::End)) {
            self.view_model.viewport.reset();
        }

        // Inspector Panel Logic
        let show_inspector = self.view_model.config.gui_config.show_inspector_always
            || self.view_model.selected_entity_idx.is_some();

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
            .default_height(150.0)
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
