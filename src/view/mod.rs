pub mod canvas;
pub mod inspector;
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
        // Dark theme
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(20, 20, 20);
        ctx.set_visuals(visuals);

        // Global shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.view_model.cancel_command();
        }

        // Reset viewport with End key
        if ctx.input(|i| i.key_pressed(egui::Key::End)) {
            self.view_model.viewport.reset();
        }

        // Layout
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
