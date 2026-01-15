use crate::viewmodel::CadViewModel;
use eframe::egui;

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
                ui.collapsing("Snap Configuration", |ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Snap Tolerance:");
                            ui.add(
                                egui::DragValue::new(&mut vm.config.snap_config.tolerance)
                                    .speed(0.5)
                                    .range(1.0..=50.0),
                            );
                        });

                        ui.checkbox(&mut vm.config.snap_config.snap_to_grid, "Snap to Grid");
                        ui.checkbox(
                            &mut vm.config.snap_config.snap_to_endpoint,
                            "Snap to Endpoint",
                        );
                        ui.checkbox(
                            &mut vm.config.snap_config.snap_to_midpoint,
                            "Snap to Midpoint",
                        );
                        ui.checkbox(&mut vm.config.snap_config.snap_to_center, "Snap to Center");
                        ui.checkbox(
                            &mut vm.config.snap_config.snap_to_intersection,
                            "Snap to Intersection",
                        );
                        ui.checkbox(&mut vm.config.snap_config.snap_to_axis, "Snap to Axis");
                    });
                });

                ui.add_space(10.0);

                ui.collapsing("Grid Configuration", |ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Grid Size:");
                            ui.add(
                                egui::DragValue::new(&mut vm.config.grid_config.grid_size)
                                    .speed(1.0)
                                    .range(1.0..=1000.0),
                            );
                        });
                        ui.checkbox(&mut vm.config.grid_config.show_grid, "Show Grid");

                        ui.horizontal(|ui| {
                            ui.label("Grid Color:");
                            ui.color_edit_button_srgb(&mut vm.config.grid_config.grid_color);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Axis Color:");
                            ui.color_edit_button_srgb(&mut vm.config.grid_config.axis_color);
                        });
                    });
                });

                ui.add_space(10.0);

                ui.collapsing("Appearance Configuration", |ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Background Color:");
                            ui.color_edit_button_srgb(
                                &mut vm.config.appearance_config.background_color,
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Selection Color:");
                            ui.color_edit_button_srgb(
                                &mut vm.config.appearance_config.selection_color,
                            );
                        });
                    });
                });

                ui.add_space(10.0);

                ui.collapsing("GUI Configuration", |ui| {
                    ui.group(|ui| {
                        ui.checkbox(
                            &mut vm.config.gui_config.show_inspector_always,
                            "Always Show Inspector",
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
        });

    vm.show_settings_window = open;
}
