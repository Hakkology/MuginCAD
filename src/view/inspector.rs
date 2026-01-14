use crate::model::Entity;
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_selection_status(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Selection:").strong());
        if vm.selected_indices.len() == 1 {
            let idx = *vm.selected_indices.iter().next().unwrap();
            if let Some(entity) = vm.model.entities.get(idx) {
                ui.label(
                    egui::RichText::new(entity.type_name())
                        .color(egui::Color32::GOLD)
                        .strong(),
                );
                ui.label(format!("(Index: {})", idx));
            }
        } else if !vm.selected_indices.is_empty() {
            ui.label(
                egui::RichText::new(format!("{} items selected", vm.selected_indices.len()))
                    .color(egui::Color32::GOLD)
                    .strong(),
            );
        } else {
            ui.label(egui::RichText::new("None").weak());
        }
    });
}

pub fn render_inspector(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.heading("Inspector");
    ui.separator();
    ui.add_space(10.0);

    // History Tools
    ui.group(|ui| {
        ui.label(egui::RichText::new("History").strong());
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            let can_undo = vm.undo_manager.can_undo();
            let can_redo = vm.undo_manager.can_redo();

            ui.add_enabled_ui(can_undo, |ui| {
                if ui
                    .button("↩ Undo (U)")
                    .on_hover_text("Undo last action")
                    .clicked()
                {
                    vm.undo();
                }
            });

            ui.add_enabled_ui(can_redo, |ui| {
                if ui
                    .button("↪ Redo")
                    .on_hover_text("Redo last undone action")
                    .clicked()
                {
                    vm.redo();
                }
            });

            ui.label(
                egui::RichText::new(format!("({} steps)", vm.undo_manager.undo_count()))
                    .small()
                    .weak(),
            );
        });
    });

    ui.add_space(10.0);

    // Utility Toolbar
    ui.group(|ui| {
        ui.label(egui::RichText::new("Transform Tools").strong());
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            let has_selection = !vm.selected_indices.is_empty();

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("⬌ Move (W)")
                    .on_hover_text("Move selected entity")
                    .clicked()
                {
                    vm.executor
                        .process_input("move", &mut vm.model, &vm.selected_indices);
                }
            });

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("↻ Rotate (E)")
                    .on_hover_text("Rotate selected entity")
                    .clicked()
                {
                    vm.executor
                        .process_input("rotate", &mut vm.model, &vm.selected_indices);
                }
            });

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("⤢ Scale (R)")
                    .on_hover_text("Scale selected entity")
                    .clicked()
                {
                    vm.executor
                        .process_input("scale", &mut vm.model, &vm.selected_indices);
                }
            });
        });

        if vm.selected_indices.is_empty() {
            ui.label(
                egui::RichText::new("Select entities to use transform tools")
                    .small()
                    .weak(),
            );
        }
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    if vm.selected_indices.len() == 1 {
        let idx = *vm.selected_indices.iter().next().unwrap();
        if let Some(entity) = vm.model.entities.get_mut(idx) {
            ui.label(egui::RichText::new(entity.type_name()).size(18.0).strong());
            ui.add_space(5.0);

            match entity {
                Entity::Line(line) => {
                    ui.group(|ui| {
                        ui.label("Start Point");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut line.start.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut line.start.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.label("End Point");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut line.end.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut line.end.y).speed(0.1));
                        });
                    });
                }
                Entity::Circle(circle) => {
                    ui.group(|ui| {
                        ui.label("Center");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut circle.center.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut circle.center.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Radius:");
                        ui.add(
                            egui::DragValue::new(&mut circle.radius)
                                .speed(0.1)
                                .range(0.0..=f32::INFINITY),
                        );
                    });
                    ui.checkbox(&mut circle.filled, "Filled");
                }
                Entity::Rectangle(rect) => {
                    ui.group(|ui| {
                        ui.label("Min Corner");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut rect.min.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut rect.min.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.label("Max Corner");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut rect.max.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut rect.max.y).speed(0.1));
                        });
                    });
                    ui.checkbox(&mut rect.filled, "Filled");
                }
            }

            ui.add_space(20.0);
            if ui
                .button(egui::RichText::new("Delete Entity").color(egui::Color32::RED))
                .clicked()
            {
                vm.delete_selected();
            }
        }
    } else if !vm.selected_indices.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(format!("{} items selected", vm.selected_indices.len()))
                    .strong(),
            );
            ui.add_space(10.0);
            if ui
                .button(egui::RichText::new("Delete Selected Items").color(egui::Color32::RED))
                .clicked()
            {
                vm.delete_selected();
            }
        });
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(egui::RichText::new("No entity selected").weak());
            ui.label(
                egui::RichText::new("Click objects in the viewport to inspect/select them")
                    .small()
                    .weak(),
            );
        });
    }
}
