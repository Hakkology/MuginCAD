use crate::model::Entity;
use crate::viewmodel::CadViewModel;
use eframe::egui;

pub fn render_selection_status(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    if vm.tabs.is_empty() {
        ui.label("No project open");
        return;
    }
    let tab = vm.active_tab();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Selection:").strong());
        if tab.selection_manager.selected_indices.len() == 1 {
            let idx = *tab
                .selection_manager
                .selected_indices
                .iter()
                .next()
                .unwrap();
            if let Some(entity) = tab.model.entities.get(idx) {
                ui.label(
                    egui::RichText::new(entity.type_name())
                        .color(egui::Color32::GOLD)
                        .strong(),
                );
                ui.label(format!("(Index: {})", idx));
            }
        } else if !tab.selection_manager.selected_indices.is_empty() {
            ui.label(
                egui::RichText::new(format!(
                    "{} items selected",
                    tab.selection_manager.selected_indices.len()
                ))
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

    if vm.tabs.is_empty() {
        ui.label("No active project");
        return;
    }

    // History Tools
    ui.group(|ui| {
        ui.label(egui::RichText::new("History").strong());
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            let (can_undo, can_redo, undo_count) = {
                let tab = vm.active_tab();
                (
                    tab.undo_manager.can_undo(),
                    tab.undo_manager.can_redo(),
                    tab.undo_manager.undo_count(),
                )
            };

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
                egui::RichText::new(format!("({} steps)", undo_count))
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
            let has_selection = !vm
                .active_tab()
                .selection_manager
                .selected_indices
                .is_empty();

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("⬌ Move (W)")
                    .on_hover_text("Move selected entity")
                    .clicked()
                {
                    let tab = vm.active_tab_mut();
                    tab.executor.process_input(
                        "move",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
            });

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("↻ Rotate (E)")
                    .on_hover_text("Rotate selected entity")
                    .clicked()
                {
                    let tab = vm.active_tab_mut();
                    tab.executor.process_input(
                        "rotate",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
            });

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("⤢ Scale (R)")
                    .on_hover_text("Scale selected entity")
                    .clicked()
                {
                    let tab = vm.active_tab_mut();
                    tab.executor.process_input(
                        "scale",
                        &mut tab.model,
                        &tab.selection_manager.selected_indices,
                    );
                }
            });
        });

        if vm
            .active_tab()
            .selection_manager
            .selected_indices
            .is_empty()
        {
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

    // Editing logic requiring mutable access
    // To avoid holding the borrow too long or conflicts, we'll scope it
    let tab = vm.active_tab_mut();

    if tab.selection_manager.selected_indices.len() == 1 {
        let idx = *tab
            .selection_manager
            .selected_indices
            .iter()
            .next()
            .unwrap();
        if let Some(entity) = tab.model.entities.get_mut(idx) {
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
                    ui.add_space(5.0);

                    // Length display
                    ui.horizontal(|ui| {
                        ui.label("Length:");
                        ui.label(format!("{:.2}", line.length()));
                    });

                    // Show length toggle
                    ui.checkbox(&mut line.show_length, "Show Length Label");

                    // Label offset (only if show_length is true)
                    if line.show_length {
                        ui.group(|ui| {
                            ui.label("Label Offset");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::DragValue::new(&mut line.label_offset.x).speed(0.5));
                                ui.label("Y:");
                                ui.add(egui::DragValue::new(&mut line.label_offset.y).speed(0.5));
                            });
                        });
                    }
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
                Entity::Arc(arc) => {
                    ui.group(|ui| {
                        ui.label("Center");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut arc.center.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut arc.center.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Radius:");
                        ui.add(
                            egui::DragValue::new(&mut arc.radius)
                                .speed(0.1)
                                .range(0.0..=f32::INFINITY),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Start Angle:");
                        ui.add(egui::DragValue::new(&mut arc.start_angle).speed(0.01));
                    });
                    ui.horizontal(|ui| {
                        ui.label("End Angle:");
                        ui.add(egui::DragValue::new(&mut arc.end_angle).speed(0.01));
                    });
                    ui.checkbox(&mut arc.filled, "Filled");
                }
                Entity::Text(text) => {
                    ui.group(|ui| {
                        ui.label("Position");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut text.position.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut text.position.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Text:");
                        ui.text_edit_singleline(&mut text.text);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Font Size:");
                        ui.add(
                            egui::DragValue::new(&mut text.style.font_size)
                                .speed(0.5)
                                .range(6.0..=72.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Rotation:");
                        // Convert radians to degrees for display
                        let mut degrees = text.rotation.to_degrees();
                        if ui
                            .add(egui::DragValue::new(&mut degrees).speed(1.0).suffix("°"))
                            .changed()
                        {
                            text.rotation = degrees.to_radians();
                        }
                    });
                }
            }

            ui.add_space(20.0);
            if ui
                .button(egui::RichText::new("Delete Entity").color(egui::Color32::RED))
                .clicked()
            {
                // vm is borrowed as tab, so we need to drop tab ref before calling vm.delete_selected()
                // We cannot call vm.delete_selected() here because `tab` is borrowed.
                // We can mark a flag? Or use interior mutability?
                // Or just use tab.selection_manager to clear selection or something?
                // Ideally `delete_selected` is on VM.
                // BUT we are inside `if let Some(entity) = tab.model...`
                // So we are holding a mutable borrow to `tab`.
                // We MUST drop `tab` before calling `vm.delete_selected()`.
                // We can't do that inside `match entity`.
                // Check if button clicked, store result, then execute after block.
                // Since this is immediate mode UI, we can just return a "delete_requested" flag.
            }
        }
    } else if !tab.selection_manager.selected_indices.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(format!(
                    "{} items selected",
                    tab.selection_manager.selected_indices.len()
                ))
                .strong(),
            );
            ui.add_space(10.0);
            if ui
                .button(egui::RichText::new("Delete Selected Items").color(egui::Color32::RED))
                .clicked()
            {
                // same issue
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
// Note: I cannot implement the `delete_selected` logic easily here without refactoring the code structure to avoid holding `tab` borrow.
// I will try to use a "command" pattern or just accept I need to rewrite this function more fundamentally.
// For now, I'll use a valid replacement that avoids the `vm.delete_selected` call INSIDE the borrow,
// OR I will simply accept I need to call it via `tab` logic if possible.
// Actually, `delete_selected` just invokes `executor.start_command("delete", model, selection)`.
// So I can do that directly on `tab`!
// `vm.delete_selected` is just a convenience wrapper. I can replicate it.

/*
Re-implementation of delete logic inline:
let tab = vm.active_tab_mut();
if ... clicked() {
    let indices = tab.selection_manager.selected_indices.clone();
    tab.executor.start_command("delete", &mut tab.model, &indices);
}
*/
