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

        // Reset per-frame focus flags
        self.view_model.inspector_renaming = false;
        self.view_model.hierarchy_renaming = false;

        // Render Settings Window if open
        if self.view_model.show_settings_window {
            settings::render_settings_window(ctx, &mut self.view_model);
        }

        // Render Materials Manager
        if self.view_model.materials_manager_open {
            ui::structure::materials::render_materials_manager(ctx, &mut self.view_model);
        }

        // Render Column Manager
        if self.view_model.column_manager_open {
            ui::structure::columns::render_column_manager(ctx, &mut self.view_model);
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

        // Quick Access Toolbar (Structure)
        egui::TopBottomPanel::top("quick_access").show(ctx, |ui| {
            ui::structure::quick_access::render_quick_access(ui, &mut self.view_model);
        });

        // Global shortcuts
        shortcuts::handle(ctx, &mut self.view_model);

        // Left Toolbar Panel
        toolbar::render_toolbar(ctx, &mut self.view_model);

        // Left Panel (Hierarchy & Layers)
        panel::left_panel("side_panel", 250.0, ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.view_model.active_left_panel_tab,
                    crate::viewmodel::LeftPanelTab::Hierarchy,
                    "Hierarchy",
                );
                ui.selectable_value(
                    &mut self.view_model.active_left_panel_tab,
                    crate::viewmodel::LeftPanelTab::Layers,
                    "Layers",
                );
            });
            ui.separator();

            match self.view_model.active_left_panel_tab {
                crate::viewmodel::LeftPanelTab::Hierarchy => {
                    hierarchy::render_hierarchy(ui, &mut self.view_model);
                }
                crate::viewmodel::LeftPanelTab::Layers => {
                    ui::layer_panel::render_layer_panel(ui, &mut self.view_model);
                }
            }
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

        // Layer Change Prompt Modal
        if self.view_model.layer_change_prompt.is_some() {
            egui::Window::new("Layer Change")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("Apply layer change to all children entities as well?");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Yes (Recursive)").clicked() {
                            self.view_model.apply_layer_change(true);
                        }
                        if ui.button("No (Only Parent)").clicked() {
                            self.view_model.apply_layer_change(false);
                        }
                        if ui.button("Cancel").clicked() {
                            self.view_model.cancel_layer_change();
                        }
                    });
                });
        }
    }
}
