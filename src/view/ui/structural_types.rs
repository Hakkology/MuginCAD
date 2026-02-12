use crate::model::structural::manager::StructuralTypeManager;
use crate::model::structural::types::{BeamType, ColumnType, FloorType};
use eframe::egui;

/// Tab selection for the type manager window
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeManagerTab {
    Columns,
    Beams,
    Floors,
}

/// Window state for creating/editing a column type
#[derive(Default, Clone)]
struct ColumnTypeEditor {
    id: String,
    width: f32,
    depth: f32,
    editing: bool,
}

/// Window state for creating/editing a beam type
#[derive(Default, Clone)]
struct BeamTypeEditor {
    id: String,
    width: f32,
    height: f32,
    editing: bool,
}

/// Window state for creating/editing a floor type
#[derive(Default, Clone)]
struct FloorTypeEditor {
    id: String,
    thickness: f32,
    editing: bool,
}

/// Dialog window for managing structural types
pub struct StructuralTypeWindow {
    pub open: bool,
    tab: TypeManagerTab,
    column_editor: ColumnTypeEditor,
    beam_editor: BeamTypeEditor,
    floor_editor: FloorTypeEditor,
}

impl Default for StructuralTypeWindow {
    fn default() -> Self {
        Self {
            open: false,
            tab: TypeManagerTab::Columns,
            column_editor: ColumnTypeEditor {
                id: "C1".to_string(),
                width: 50.0,
                depth: 50.0,
                editing: false,
            },
            beam_editor: BeamTypeEditor {
                id: "B1".to_string(),
                width: 25.0,
                height: 40.0,
                editing: false,
            },
            floor_editor: FloorTypeEditor {
                id: "F1".to_string(),
                thickness: 20.0,
                editing: false,
            },
        }
    }
}

impl StructuralTypeWindow {
    /// Shows the window
    pub fn show(&mut self, ctx: &egui::Context, type_manager: &mut StructuralTypeManager) {
        if !self.open {
            return;
        }

        let mut open = self.open;
        let mut tab = self.tab;

        egui::Window::new("📐 Structural Type Manager")
            .open(&mut open)
            .resizable(true)
            .default_size([550.0, 420.0])
            .show(ctx, |ui| {
                // Tab bar
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut tab, TypeManagerTab::Columns, "🏛 Columns");
                    ui.selectable_value(&mut tab, TypeManagerTab::Beams, "📏 Beams");
                    ui.selectable_value(&mut tab, TypeManagerTab::Floors, "⬛ Floors");
                });

                ui.separator();

                match tab {
                    TypeManagerTab::Columns => {
                        Self::show_columns_tab_inner(ui, &mut self.column_editor, type_manager);
                    }
                    TypeManagerTab::Beams => {
                        Self::show_beams_tab_inner(ui, &mut self.beam_editor, type_manager);
                    }
                    TypeManagerTab::Floors => {
                        Self::show_floors_tab_inner(ui, &mut self.floor_editor, type_manager);
                    }
                }
            });

        self.open = open;
        self.tab = tab;
    }

    fn show_columns_tab_inner(
        ui: &mut egui::Ui,
        editor: &mut ColumnTypeEditor,
        type_manager: &mut StructuralTypeManager,
    ) {
        ui.horizontal(|ui| {
            // Left: List
            ui.vertical(|ui| {
                ui.set_min_width(220.0);
                ui.heading("Column Types");

                // Show active type
                if let Some(ref active_id) = type_manager.active_column_type_id {
                    ui.label(
                        egui::RichText::new(format!("Active: {}", active_id))
                            .color(egui::Color32::LIGHT_GREEN),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("Active: (default 50x50)").color(egui::Color32::GRAY),
                    );
                }
                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        let types: Vec<(String, f32, f32)> = type_manager
                            .column_types
                            .iter()
                            .map(|(id, ct)| (id.clone(), ct.width, ct.depth))
                            .collect();

                        let active_id = type_manager.active_column_type_id.clone();
                        let mut delete_id: Option<String> = None;
                        let mut set_active_id: Option<String> = None;

                        for (id, width, depth) in types {
                            let is_active = active_id.as_ref() == Some(&id);
                            ui.horizontal(|ui| {
                                let label = if is_active {
                                    egui::RichText::new(format!("✓ {} ({}x{})", id, width, depth))
                                        .color(egui::Color32::LIGHT_GREEN)
                                } else {
                                    egui::RichText::new(format!("  {} ({}x{})", id, width, depth))
                                };

                                if ui.selectable_label(is_active, label).clicked() {
                                    set_active_id = Some(id.clone());
                                }

                                if ui.small_button("✏").on_hover_text("Edit").clicked() {
                                    editor.id = id.clone();
                                    editor.width = width;
                                    editor.depth = depth;
                                    editor.editing = true;
                                }
                                if ui.small_button("🗑").on_hover_text("Delete").clicked() {
                                    delete_id = Some(id.clone());
                                }
                            });
                        }

                        if let Some(id) = set_active_id {
                            type_manager.active_column_type_id = Some(id);
                        }
                        if let Some(id) = delete_id {
                            type_manager.remove_column_type(&id);
                        }
                    });
            });

            ui.separator();

            // Right: Editor
            ui.vertical(|ui| {
                ui.heading(if editor.editing {
                    "Edit Column Type"
                } else {
                    "New Column Type"
                });

                ui.horizontal(|ui| {
                    ui.label("ID:");
                    ui.text_edit_singleline(&mut editor.id);
                });

                ui.horizontal(|ui| {
                    ui.label("Width (cm):");
                    ui.add(
                        egui::DragValue::new(&mut editor.width)
                            .range(10.0..=200.0)
                            .speed(1.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Depth (cm):");
                    ui.add(
                        egui::DragValue::new(&mut editor.depth)
                            .range(10.0..=200.0)
                            .speed(1.0),
                    );
                });

                ui.add_space(10.0);

                // Preview
                ui.group(|ui| {
                    ui.label("Preview:");
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(100.0, 100.0), egui::Sense::hover());
                    let scale = 80.0 / editor.width.max(editor.depth);
                    let w = editor.width * scale;
                    let d = editor.depth * scale;
                    let center = rect.center();
                    ui.painter().rect_filled(
                        egui::Rect::from_center_size(center, egui::vec2(w, d)),
                        0.0,
                        egui::Color32::from_rgb(100, 100, 120),
                    );
                    ui.painter().rect_stroke(
                        egui::Rect::from_center_size(center, egui::vec2(w, d)),
                        0.0,
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Save & Use").clicked() {
                        println!(
                            "DEBUG: Save & Use clicked! ID={}, Width={}, Depth={}",
                            editor.id, editor.width, editor.depth
                        );
                        let new_type = ColumnType {
                            id: editor.id.clone(),
                            width: editor.width,
                            depth: editor.depth,
                            color: [128, 128, 128],
                        };
                        type_manager.add_column_type(new_type);
                        type_manager.active_column_type_id = Some(editor.id.clone());
                        println!(
                            "DEBUG: Active column set to: {:?}",
                            type_manager.active_column_type_id
                        );
                        editor.editing = false;
                    }
                    if ui.button("🔄 New").clicked() {
                        editor.id = format!("C{}", type_manager.column_types.len() + 1);
                        editor.width = 50.0;
                        editor.depth = 50.0;
                        editor.editing = false;
                    }
                });
            });
        });
    }

    fn show_beams_tab_inner(
        ui: &mut egui::Ui,
        editor: &mut BeamTypeEditor,
        type_manager: &mut StructuralTypeManager,
    ) {
        ui.horizontal(|ui| {
            // Left: List
            ui.vertical(|ui| {
                ui.set_min_width(220.0);
                ui.heading("Beam Types");

                if let Some(ref active_id) = type_manager.active_beam_type_id {
                    ui.label(
                        egui::RichText::new(format!("Active: {}", active_id))
                            .color(egui::Color32::LIGHT_GREEN),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("Active: (default 25x40)").color(egui::Color32::GRAY),
                    );
                }
                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        let types: Vec<(String, f32, f32)> = type_manager
                            .beam_types
                            .iter()
                            .map(|(id, bt)| (id.clone(), bt.width, bt.height))
                            .collect();

                        let active_id = type_manager.active_beam_type_id.clone();
                        let mut delete_id: Option<String> = None;
                        let mut set_active_id: Option<String> = None;

                        for (id, width, height) in types {
                            let is_active = active_id.as_ref() == Some(&id);
                            ui.horizontal(|ui| {
                                let label = if is_active {
                                    egui::RichText::new(format!("✓ {} ({}x{})", id, width, height))
                                        .color(egui::Color32::LIGHT_GREEN)
                                } else {
                                    egui::RichText::new(format!("  {} ({}x{})", id, width, height))
                                };

                                if ui.selectable_label(is_active, label).clicked() {
                                    set_active_id = Some(id.clone());
                                }

                                if ui.small_button("✏").on_hover_text("Edit").clicked() {
                                    editor.id = id.clone();
                                    editor.width = width;
                                    editor.height = height;
                                    editor.editing = true;
                                }
                                if ui.small_button("🗑").on_hover_text("Delete").clicked() {
                                    delete_id = Some(id.clone());
                                }
                            });
                        }

                        if let Some(id) = set_active_id {
                            type_manager.active_beam_type_id = Some(id);
                        }
                        if let Some(id) = delete_id {
                            type_manager.remove_beam_type(&id);
                        }
                    });
            });

            ui.separator();

            // Right: Editor
            ui.vertical(|ui| {
                ui.heading(if editor.editing {
                    "Edit Beam Type"
                } else {
                    "New Beam Type"
                });

                ui.horizontal(|ui| {
                    ui.label("ID:");
                    ui.text_edit_singleline(&mut editor.id);
                });

                ui.horizontal(|ui| {
                    ui.label("Width (cm):");
                    ui.add(
                        egui::DragValue::new(&mut editor.width)
                            .range(10.0..=100.0)
                            .speed(1.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Height (cm):");
                    ui.add(
                        egui::DragValue::new(&mut editor.height)
                            .range(10.0..=150.0)
                            .speed(1.0),
                    );
                });

                ui.add_space(10.0);

                // Preview
                ui.group(|ui| {
                    ui.label("Cross-section:");
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(100.0, 100.0), egui::Sense::hover());
                    let scale = 80.0 / editor.width.max(editor.height);
                    let w = editor.width * scale;
                    let h = editor.height * scale;
                    let center = rect.center();
                    ui.painter().rect_filled(
                        egui::Rect::from_center_size(center, egui::vec2(w, h)),
                        0.0,
                        egui::Color32::from_rgb(80, 80, 100),
                    );
                    ui.painter().rect_stroke(
                        egui::Rect::from_center_size(center, egui::vec2(w, h)),
                        0.0,
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Save & Use").clicked() {
                        let new_type = BeamType {
                            id: editor.id.clone(),
                            width: editor.width,
                            height: editor.height,
                            color: [100, 100, 100],
                        };
                        type_manager.add_beam_type(new_type);
                        type_manager.active_beam_type_id = Some(editor.id.clone());
                        editor.editing = false;
                    }
                    if ui.button("🔄 New").clicked() {
                        editor.id = format!("B{}", type_manager.beam_types.len() + 1);
                        editor.width = 25.0;
                        editor.height = 40.0;
                        editor.editing = false;
                    }
                });
            });
        });
    }

    fn show_floors_tab_inner(
        ui: &mut egui::Ui,
        editor: &mut FloorTypeEditor,
        type_manager: &mut StructuralTypeManager,
    ) {
        ui.horizontal(|ui| {
            // Left: List
            ui.vertical(|ui| {
                ui.set_min_width(200.0);
                ui.heading("Floor Types");

                egui::ScrollArea::vertical()
                    .max_height(250.0)
                    .show(ui, |ui| {
                        let types: Vec<(String, f32)> = type_manager
                            .floor_types
                            .iter()
                            .map(|(id, ft)| (id.clone(), ft.thickness))
                            .collect();

                        let mut delete_id: Option<String> = None;

                        for (id, thickness) in types {
                            ui.horizontal(|ui| {
                                ui.label(format!("{} ({}cm)", id, thickness));
                                if ui.small_button("✏").on_hover_text("Edit").clicked() {
                                    editor.id = id.clone();
                                    editor.thickness = thickness;
                                    editor.editing = true;
                                }
                                if ui.small_button("🗑").on_hover_text("Delete").clicked() {
                                    delete_id = Some(id.clone());
                                }
                            });
                        }

                        if let Some(id) = delete_id {
                            type_manager.floor_types.remove(&id);
                        }
                    });
            });

            ui.separator();

            // Right: Editor
            ui.vertical(|ui| {
                ui.heading(if editor.editing {
                    "Edit Floor Type"
                } else {
                    "New Floor Type"
                });

                ui.horizontal(|ui| {
                    ui.label("ID:");
                    ui.text_edit_singleline(&mut editor.id);
                });

                ui.horizontal(|ui| {
                    ui.label("Thickness (cm):");
                    ui.add(
                        egui::DragValue::new(&mut editor.thickness)
                            .range(5.0..=50.0)
                            .speed(0.5),
                    );
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Save").clicked() {
                        let new_type = FloorType {
                            id: editor.id.clone(),
                            thickness: editor.thickness,
                            color: [180, 180, 180],
                        };
                        type_manager.add_floor_type(new_type);
                        editor.editing = false;
                    }
                    if ui.button("🔄 New").clicked() {
                        editor.id = format!("F{}", type_manager.floor_types.len() + 1);
                        editor.thickness = 20.0;
                        editor.editing = false;
                    }
                });
            });
        });
    }
}
