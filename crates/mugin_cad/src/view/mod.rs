pub mod canvas;
pub mod rendering;
pub mod shortcuts;
pub mod ui;
pub mod viewport;

pub use ui::hierarchy;
pub use ui::inspector;
pub use ui::settings;
pub use ui::tab_bar;
pub use ui::terminal;
pub use ui::toolbar;
pub use ui::topmenu;

pub use rendering::context;

use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::panel;

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

        // Render Structure Manager if open
        if self.view_model.structure_manager_open {
            ui::structure::manager::render_structure_manager(ctx, &mut self.view_model);
        }

        // Render Export Window if open
        {
            let CadViewModel {
                export_window,
                tabs,
                active_tab_index,
                ..
            } = &mut self.view_model;
            if *active_tab_index < tabs.len() {
                let model = &tabs[*active_tab_index].model;
                export_window.show(ctx, model);
            }
        }

        // Top Menu
        topmenu::render_top_menu(ctx, &mut self.view_model);

        // Tab Bar
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            tab_bar::render_tab_bar(ui, &mut self.view_model);
        });

        // Global shortcuts
        shortcuts::handle(ctx, &mut self.view_model);

        // Left Toolbar Panel
        toolbar::render_toolbar(ctx, &mut self.view_model);

        // Hierarchy Panel (left side, after toolbar)
        panel::left_panel("hierarchy", 200.0, ctx, |ui| {
            hierarchy::render_hierarchy(ui, &mut self.view_model);
        });

        // Inspector Panel Logic
        let show_inspector = self.view_model.config.gui_config.show_inspector_always
            || !self
                .view_model
                .active_tab()
                .selection_manager
                .selected_ids
                .is_empty();

        if show_inspector {
            panel::right_panel("inspector", 250.0, ctx, |ui| {
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
