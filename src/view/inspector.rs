use crate::model::Entity;
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_selection_status(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Selection:").strong());
        if let Some(idx) = vm.selected_entity_idx {
            if let Some(entity) = vm.model.entities.get(idx) {
                ui.label(
                    egui::RichText::new(entity.type_name())
                        .color(egui::Color32::GOLD)
                        .strong(),
                );
                ui.label(format!("(Index: {})", idx));
            }
        } else {
            ui.label(egui::RichText::new("None").weak());
        }
    });
}

pub fn render_inspector(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.heading("Inspector");
    ui.separator();
    ui.add_space(10.0);

    let mut to_delete = None;

    if let Some(idx) = vm.selected_entity_idx {
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
                to_delete = Some(idx);
            }
        }
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(egui::RichText::new("No entity selected").weak());
            ui.label(
                egui::RichText::new("Click an object in the viewport to inspect it")
                    .small()
                    .weak(),
            );
        });
    }

    if let Some(idx) = to_delete {
        vm.model.entities.remove(idx);
        vm.selected_entity_idx = None;
    }
}
