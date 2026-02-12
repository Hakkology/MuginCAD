use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::properties;

pub fn render_settings_window(ctx: &egui::Context, vm: &mut CadViewModel) {
    let mut open = vm.show_settings_window;

    egui::Window::new("Settings")
        .open(&mut open)
        .min_width(400.0)
        .min_height(300.0)
        .show(ctx, |ui| {
            ui.heading("Application Settings");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // ── Snap Configuration ───────────────────────
                properties::collapsible_section(ui, "Snap Configuration", |ui| {
                    properties::float_range(
                        ui,
                        "Snap Tolerance:",
                        &mut vm.config.snap_config.tolerance,
                        0.5,
                        1.0..=50.0,
                    );
                    properties::toggle(ui, "Snap to Grid", &mut vm.config.snap_config.snap_to_grid);
                    properties::toggle(
                        ui,
                        "Snap to Endpoint",
                        &mut vm.config.snap_config.snap_to_endpoint,
                    );
                    properties::toggle(
                        ui,
                        "Snap to Midpoint",
                        &mut vm.config.snap_config.snap_to_midpoint,
                    );
                    properties::toggle(
                        ui,
                        "Snap to Center",
                        &mut vm.config.snap_config.snap_to_center,
                    );
                    properties::toggle(
                        ui,
                        "Snap to Intersection",
                        &mut vm.config.snap_config.snap_to_intersection,
                    );
                    properties::toggle(ui, "Snap to Axis", &mut vm.config.snap_config.snap_to_axis);
                });

                ui.add_space(10.0);

                // ── Grid Configuration ───────────────────────
                properties::collapsible_section(ui, "Grid Configuration", |ui| {
                    properties::float_range(
                        ui,
                        "Grid Size:",
                        &mut vm.config.grid_config.grid_size,
                        1.0,
                        1.0..=1000.0,
                    );
                    properties::toggle(ui, "Show Grid", &mut vm.config.grid_config.show_grid);
                    properties::color_rgb(ui, "Grid Color:", &mut vm.config.grid_config.grid_color);
                    properties::color_rgb(ui, "Axis Color:", &mut vm.config.grid_config.axis_color);
                });

                ui.add_space(10.0);

                // ── Appearance Configuration ─────────────────
                properties::collapsible_section(ui, "Appearance Configuration", |ui| {
                    properties::color_rgb(
                        ui,
                        "Background Color:",
                        &mut vm.config.appearance_config.background_color,
                    );
                    properties::color_rgb(
                        ui,
                        "Selection Color:",
                        &mut vm.config.appearance_config.selection_color,
                    );
                });

                ui.add_space(10.0);

                // ── GUI Configuration ────────────────────────
                properties::collapsible_section(ui, "GUI Configuration", |ui| {
                    properties::toggle(
                        ui,
                        "Always Show Inspector",
                        &mut vm.config.gui_config.show_inspector_always,
                    );
                    ui.label(
                        egui::RichText::new(
                            "If disabled, inspector only shows when an object is selected.",
                        )
                        .weak()
                        .size(12.0),
                    );
                });
            });
        });

    vm.show_settings_window = open;
}
