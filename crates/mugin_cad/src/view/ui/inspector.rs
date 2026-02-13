use crate::model::Shape;
use crate::model::shapes::{
    annotation::TextAnnotation, arc::Arc, circle::Circle, line::Line, rectangle::Rectangle,
};
use crate::model::structure::column::ColumnData;
use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::properties;

pub fn render_selection_status(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    if vm.tabs.is_empty() {
        ui.label("No project open");
        return;
    }
    let tab = vm.active_tab();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Selection:").strong());
        if tab.selection_manager.selected_ids.len() == 1 {
            let id = *tab.selection_manager.selected_ids.iter().next().unwrap();
            if let Some(entity) = tab.model.find_by_id(id) {
                ui.label(
                    egui::RichText::new(entity.type_name())
                        .color(egui::Color32::GOLD)
                        .strong(),
                );
                ui.label(format!("(ID: {})", id));
            }
        } else if !tab.selection_manager.selected_ids.is_empty() {
            ui.label(
                egui::RichText::new(format!(
                    "{} items selected",
                    tab.selection_manager.selected_ids.len()
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

    // ── History Tools ────────────────────────────────────────
    properties::section(ui, "History", |ui| {
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

    // ── Transform Tools ──────────────────────────────────────
    properties::section(ui, "Transform Tools", |ui| {
        ui.horizontal(|ui| {
            let has_selection = !vm.active_tab().selection_manager.selected_ids.is_empty();

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("⬌ Move (W)")
                    .on_hover_text("Move selected entity")
                    .clicked()
                {
                    let tab = vm.active_tab_mut();
                    let ids = tab.selection_manager.selected_ids.clone();
                    tab.executor.process_input("move", &mut tab.model, &ids);
                }
            });

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("↻ Rotate (E)")
                    .on_hover_text("Rotate selected entity")
                    .clicked()
                {
                    let tab = vm.active_tab_mut();
                    let ids = tab.selection_manager.selected_ids.clone();
                    tab.executor.process_input("rotate", &mut tab.model, &ids);
                }
            });

            ui.add_enabled_ui(has_selection, |ui| {
                if ui
                    .button("⤢ Scale (R)")
                    .on_hover_text("Scale selected entity")
                    .clicked()
                {
                    let tab = vm.active_tab_mut();
                    let ids = tab.selection_manager.selected_ids.clone();
                    tab.executor.process_input("scale", &mut tab.model, &ids);
                }
            });
        });

        if vm.active_tab().selection_manager.selected_ids.is_empty() {
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

    // ── Entity Inspector ─────────────────────────────────────
    let mut is_renaming = false;
    let mut delete_id = None;
    let mut delete_selection = false;

    {
        let tab = vm.active_tab_mut();

        if tab.selection_manager.selected_ids.len() == 1 {
            let id = *tab.selection_manager.selected_ids.iter().next().unwrap();

            if let Some(entity) = tab.model.find_by_id_mut(id) {
                // Entity name (editable)
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    let response = ui.text_edit_singleline(&mut entity.name);
                    if response.has_focus() || response.clicked() {
                        is_renaming = true;
                    }
                });
                ui.add_space(3.0);
                ui.label(
                    egui::RichText::new(entity.type_name())
                        .size(14.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(5.0);

                match &mut entity.shape {
                    Shape::Line(line) => inspect_line(ui, line),
                    Shape::Circle(circle) => inspect_circle(ui, circle),
                    Shape::Rectangle(rect) => inspect_rectangle(ui, rect),
                    Shape::Arc(arc) => inspect_arc(ui, arc),
                    Shape::Text(text) => inspect_text(ui, text),
                    Shape::Column(col) => inspect_column(ui, col),
                    Shape::None => {}
                }

                if !entity.children.is_empty() {
                    properties::section(ui, "Children", |ui| {
                        ui.label(format!("Children: {}", entity.children.len()));
                        for (i, child) in entity.children.iter().enumerate() {
                            ui.label(format!("  {}. {}", i + 1, child.type_name()));
                        }
                    });
                }

                ui.add_space(20.0);
                if ui
                    .button(egui::RichText::new("Delete Entity").color(egui::Color32::RED))
                    .clicked()
                {
                    delete_id = Some(id);
                }
            }
        } else if !tab.selection_manager.selected_ids.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!(
                        "{} items selected",
                        tab.selection_manager.selected_ids.len()
                    ))
                    .strong(),
                );
                ui.add_space(10.0);
                if ui
                    .button(egui::RichText::new("Delete Selected Items").color(egui::Color32::RED))
                    .clicked()
                {
                    delete_selection = true;
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

    vm.inspector_renaming = is_renaming;

    // Handle deferred deletion
    if let Some(id_to_delete) = delete_id {
        let tab = vm.active_tab_mut();
        tab.model
            .remove_entities_by_ids(&std::collections::HashSet::from([id_to_delete]));
        tab.selection_manager.selected_ids.remove(&id_to_delete);
    }

    if delete_selection {
        let tab = vm.active_tab_mut();
        let ids = tab.selection_manager.selected_ids.clone();
        if !ids.is_empty() {
            tab.model.remove_entities_by_ids(&ids);
            tab.selection_manager.selected_ids.clear();
        }
    }
}

fn inspect_line(ui: &mut egui::Ui, line: &mut Line) {
    properties::point2(ui, "Start Point", &mut line.start.x, &mut line.start.y);
    ui.add_space(5.0);
    properties::point2(ui, "End Point", &mut line.end.x, &mut line.end.y);
    ui.add_space(5.0);

    properties::display_float(ui, "Length:", line.length(), 2);
    properties::toggle(ui, "Show Length Label", &mut line.show_length);

    if line.show_length {
        properties::point2_speed(
            ui,
            "Label Offset",
            &mut line.label_offset.x,
            &mut line.label_offset.y,
            0.5,
        );
    }
}

fn inspect_circle(ui: &mut egui::Ui, circle: &mut Circle) {
    properties::point2(ui, "Center", &mut circle.center.x, &mut circle.center.y);
    ui.add_space(5.0);
    properties::float_range(ui, "Radius:", &mut circle.radius, 0.1, 0.0..=f32::INFINITY);
    properties::toggle(ui, "Filled", &mut circle.filled);
}

fn inspect_rectangle(ui: &mut egui::Ui, rect: &mut Rectangle) {
    properties::point2(ui, "Min Corner", &mut rect.min.x, &mut rect.min.y);
    ui.add_space(5.0);
    properties::point2(ui, "Max Corner", &mut rect.max.x, &mut rect.max.y);
    properties::toggle(ui, "Filled", &mut rect.filled);
}

fn inspect_arc(ui: &mut egui::Ui, arc: &mut Arc) {
    properties::point2(ui, "Center", &mut arc.center.x, &mut arc.center.y);
    ui.add_space(5.0);
    properties::float_range(ui, "Radius:", &mut arc.radius, 0.1, 0.0..=f32::INFINITY);
    properties::float_value(ui, "Start Angle:", &mut arc.start_angle, 0.01);
    properties::float_value(ui, "End Angle:", &mut arc.end_angle, 0.01);
    properties::toggle(ui, "Filled", &mut arc.filled);
}

fn inspect_text(ui: &mut egui::Ui, text: &mut TextAnnotation) {
    properties::point2(ui, "Position", &mut text.position.x, &mut text.position.y);
    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Text:");
        ui.text_edit_singleline(&mut text.text);
    });

    properties::float_range(ui, "Font Size:", &mut text.style.font_size, 0.5, 6.0..=72.0);
    properties::angle_degrees(ui, "Rotation:", &mut text.rotation);
}

fn inspect_column(ui: &mut egui::Ui, col: &mut ColumnData) {
    ui.heading("Column Properties");
    properties::point2(ui, "Center", &mut col.center.x, &mut col.center.y);
    ui.add_space(5.0);

    properties::float_range(ui, "Width:", &mut col.width, 1.0, 1.0..=1000.0);
    properties::float_range(ui, "Height:", &mut col.height, 1.0, 1.0..=1000.0);
    properties::angle_degrees(ui, "Rotation:", &mut col.rotation);

    ui.add_space(5.0);
    ui.horizontal(|ui| {
        ui.label("Label:");
        ui.text_edit_singleline(&mut col.label);
    });

    // Future: Material selection via ComboBox
    ui.label(egui::RichText::new(format!("Type ID: {}", col.column_type_id)).weak());
}
