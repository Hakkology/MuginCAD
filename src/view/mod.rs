use crate::model::Entity;
use crate::model::Vector2;
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
        // Aesthetic setup: Dark theme is already default in egui, but let's ensure it's sleek
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(20, 20, 20);
        ctx.set_visuals(visuals);

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.view_model.state = crate::viewmodel::CommandState::Idle;
            self.view_model.status_message = "Command:".to_string();
            self.view_model.command_input.clear();
        }

        egui::SidePanel::right("inspector")
            .resizable(true)
            .default_width(250.0)
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(25, 25, 25))
                    .inner_margin(10.0),
            )
            .show(ctx, |ui| {
                render_inspector(ui, &mut self.view_model);
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
                render_selection_status(ui, &mut self.view_model);
                ui.separator();
                render_terminal(ui, &mut self.view_model);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 15)))
            .show(ctx, |ui| {
                render_canvas(ui, &mut self.view_model);
            });
    }
}

fn render_selection_status(ui: &mut egui::Ui, vm: &mut CadViewModel) {
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

fn render_inspector(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.heading("Inspector");
    ui.separator();
    ui.add_space(10.0);

    let mut to_delete = None;

    if let Some(idx) = vm.selected_entity_idx {
        if let Some(entity) = vm.model.entities.get_mut(idx) {
            ui.label(egui::RichText::new(entity.type_name()).size(18.0).strong());
            ui.add_space(5.0);

            match entity {
                Entity::Line { start, end } => {
                    ui.group(|ui| {
                        ui.label("Start Point");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut start.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut start.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.label("End Point");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut end.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut end.y).speed(0.1));
                        });
                    });
                }
                Entity::Circle {
                    center,
                    radius,
                    filled,
                } => {
                    ui.group(|ui| {
                        ui.label("Center");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut center.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut center.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Radius:");
                        ui.add(
                            egui::DragValue::new(radius)
                                .speed(0.1)
                                .range(0.0..=f32::INFINITY),
                        );
                    });
                    ui.checkbox(filled, "Filled");
                }
                Entity::Rectangle { min, max, filled } => {
                    ui.group(|ui| {
                        ui.label("Min Corner");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut min.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut min.y).speed(0.1));
                        });
                    });
                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.label("Max Corner");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut max.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut max.y).speed(0.1));
                        });
                    });
                    ui.checkbox(filled, "Filled");
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

fn render_terminal(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.vertical(|ui| {
        ui.style_mut().visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 200));

        let scroll_height = ui.available_height() - 30.0;
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .max_height(scroll_height)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                for line in &vm.command_history {
                    ui.label(egui::RichText::new(line).monospace());
                }
            });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(&vm.status_message)
                    .strong()
                    .color(egui::Color32::LIGHT_BLUE),
            );

            let text_edit = egui::TextEdit::singleline(&mut vm.command_input)
                .desired_width(f32::INFINITY)
                .frame(false)
                .font(egui::TextStyle::Monospace);

            let response = ui.add(text_edit);

            if vm.command_history.is_empty() {
                response.request_focus();
            }

            if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                vm.process_command();
                response.request_focus();
            }
        });
    });
}

fn render_canvas(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

    let rect = response.rect;

    // Convert mouse position to CAD coordinates
    if response.clicked() {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let cad_pos = Vector2::new(
                mouse_pos.x - rect.center().x,
                -(mouse_pos.y - rect.center().y),
            );
            vm.handle_click(cad_pos);
        }
    }

    // Grid (optional but nice for CAD)
    let grid_size = 50.0;
    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 40));

    // Simple coordinate transformation (center of screen is 0,0)
    let to_screen = |pos: Vector2| {
        egui::pos2(
            rect.center().x + pos.x,
            rect.center().y - pos.y, // Y is up in CAD
        )
    };

    // Draw grid
    let mut x = (rect.min.x / grid_size).floor() * grid_size;
    while x < rect.max.x {
        painter.line_segment(
            [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
            stroke,
        );
        x += grid_size;
    }
    let mut y = (rect.min.y / grid_size).floor() * grid_size;
    while y < rect.max.y {
        painter.line_segment(
            [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
            stroke,
        );
        y += grid_size;
    }

    // Axes
    painter.line_segment(
        [
            egui::pos2(rect.min.x, rect.center().y),
            egui::pos2(rect.max.x, rect.center().y),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60)),
    );
    painter.line_segment(
        [
            egui::pos2(rect.center().x, rect.min.y),
            egui::pos2(rect.center().x, rect.max.y),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 60)),
    );

    // Draw entities
    let mut hovered_entity_idx = None;
    if let Some(mouse_pos) = response.hover_pos() {
        let cad_mouse = Vector2::new(
            mouse_pos.x - rect.center().x,
            -(mouse_pos.y - rect.center().y),
        );
        hovered_entity_idx = vm.model.pick_entity(cad_mouse, 5.0);
    }

    for (i, entity) in vm.model.entities.iter().enumerate() {
        let is_selected = Some(i) == vm.selected_entity_idx;
        let is_hovered = Some(i) == hovered_entity_idx;

        let color = if is_selected {
            egui::Color32::GOLD
        } else if is_hovered {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_rgb(0, 255, 255)
        };

        let stroke_width = if is_selected { 2.5 } else { 1.5 };

        match entity {
            Entity::Line { start, end } => {
                painter.line_segment(
                    [to_screen(*start), to_screen(*end)],
                    egui::Stroke::new(stroke_width, color),
                );
            }
            Entity::Circle {
                center,
                radius,
                filled,
            } => {
                let center_screen = to_screen(*center);
                let radius_pixels = *radius;
                if *filled {
                    painter.circle_filled(center_screen, radius_pixels, color.linear_multiply(0.3));
                }
                painter.circle_stroke(
                    center_screen,
                    radius_pixels,
                    egui::Stroke::new(stroke_width, color),
                );
            }
            Entity::Rectangle { min, max, filled } => {
                let rect_screen = egui::Rect::from_min_max(
                    to_screen(Vector2::new(min.x, max.y)),
                    to_screen(Vector2::new(max.x, min.y)),
                );
                if *filled {
                    painter.rect_filled(rect_screen, 0.0, color.linear_multiply(0.3));
                }
                painter.rect_stroke(rect_screen, 0.0, egui::Stroke::new(stroke_width, color));
            }
        }
    }

    // Cursor Crosshairs
    if let Some(mouse_pos) = response.hover_pos() {
        let cross_stroke = egui::Stroke::new(
            0.5,
            egui::Color32::from_rgba_unmultiplied(200, 200, 200, 100),
        );
        // Horizontal
        painter.line_segment(
            [
                egui::pos2(rect.min.x, mouse_pos.y),
                egui::pos2(rect.max.x, mouse_pos.y),
            ],
            cross_stroke,
        );
        // Vertical
        painter.line_segment(
            [
                egui::pos2(mouse_pos.x, rect.min.y),
                egui::pos2(mouse_pos.x, rect.max.y),
            ],
            cross_stroke,
        );

        // Previews
        match vm.state {
            crate::viewmodel::CommandState::WaitingForLineEnd { start } => {
                let end_cad = Vector2::new(
                    mouse_pos.x - rect.center().x,
                    -(mouse_pos.y - rect.center().y),
                );
                painter.line_segment(
                    [to_screen(start), to_screen(end_cad)],
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
                    ),
                );
            }
            crate::viewmodel::CommandState::WaitingForCircleRadius { center } => {
                let current_cad = Vector2::new(
                    mouse_pos.x - rect.center().x,
                    -(mouse_pos.y - rect.center().y),
                );
                let radius = center.dist(current_cad);
                painter.circle_stroke(
                    to_screen(center),
                    radius,
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
                    ),
                );
            }
            crate::viewmodel::CommandState::WaitingForRectEnd { start } => {
                let current_cad = Vector2::new(
                    mouse_pos.x - rect.center().x,
                    -(mouse_pos.y - rect.center().y),
                );
                let min = Vector2::new(start.x.min(current_cad.x), start.y.min(current_cad.y));
                let max = Vector2::new(start.x.max(current_cad.x), start.y.max(current_cad.y));
                let rect_screen = egui::Rect::from_min_max(
                    to_screen(Vector2::new(min.x, max.y)),
                    to_screen(Vector2::new(max.x, min.y)),
                );
                painter.rect_stroke(
                    rect_screen,
                    0.0,
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128),
                    ),
                );
            }
            _ => {}
        }
    }
}
