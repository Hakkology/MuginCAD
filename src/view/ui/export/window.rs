use crate::export::pdf::PdfExporter;
use crate::export::settings::{ExportSettings, ExportSource, PageOrientation, PageSize, ScaleType};
use crate::model::{CadModel, Entity, Vector2};
use eframe::egui;

pub struct ExportWindow {
    pub open: bool,
    pub settings: ExportSettings,
}

impl Default for ExportWindow {
    fn default() -> Self {
        Self {
            open: false,
            settings: ExportSettings::default(),
        }
    }
}

impl ExportWindow {
    pub fn show(&mut self, ctx: &egui::Context, model: &CadModel) {
        if !self.open {
            return;
        }

        let mut close_window = false;

        egui::Window::new("Export PDF")
            .open(&mut self.open)
            .resize(|r| r.fixed_size([800.0, 600.0]))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Left Panel: Preview
                    ui.vertical(|ui| {
                        ui.heading("Preview");
                        egui::Frame::canvas(ui.style()).show(ui, |ui| {
                            ExportWindow::draw_preview(ui, &self.settings, model);
                        });
                    });

                    ui.separator();

                    // Right Panel: Settings
                    ui.vertical(|ui| {
                        ui.heading("Settings");

                        // Page Size
                        ui.group(|ui| {
                            ui.label("Page Size");
                            ui.radio_value(&mut self.settings.page_size, PageSize::A4, "A4");
                            ui.radio_value(&mut self.settings.page_size, PageSize::A3, "A3");
                        });

                        ui.separator();

                        // Orientation
                        ui.group(|ui| {
                            ui.label("Orientation");
                            ui.radio_value(
                                &mut self.settings.orientation,
                                PageOrientation::Portrait,
                                "Portrait",
                            );
                            ui.radio_value(
                                &mut self.settings.orientation,
                                PageOrientation::Landscape,
                                "Landscape",
                            );
                        });

                        ui.separator();

                        // Scale
                        ui.group(|ui| {
                            ui.label("Scale");
                            ui.radio_value(
                                &mut self.settings.scale_type,
                                ScaleType::FitToPage,
                                "Fit to Page",
                            );
                            ui.radio_value(
                                &mut self.settings.scale_type,
                                ScaleType::Standard(50.0),
                                "1:50",
                            );
                            ui.radio_value(
                                &mut self.settings.scale_type,
                                ScaleType::Standard(100.0),
                                "1:100",
                            );
                        });

                        ui.separator();

                        // Source
                        ui.group(|ui| {
                            ui.label("Source");
                            ui.radio_value(
                                &mut self.settings.source,
                                ExportSource::ModelBounds,
                                "All Entities",
                            );

                            let is_viewport =
                                matches!(self.settings.source, ExportSource::Viewport(_, _));
                            // Use a dummy viewport for radio selection if not already viewport
                            let viewport_val = if is_viewport {
                                self.settings.source
                            } else {
                                ExportSource::Viewport(
                                    Vector2::new(0.0, 0.0),
                                    Vector2::new(100.0, 100.0),
                                )
                            };

                            if ui
                                .radio_value(&mut self.settings.source, viewport_val, "Region")
                                .clicked()
                            {
                                // If clicked, try to sync
                            }

                            if matches!(self.settings.source, ExportSource::Viewport(_, _)) {
                                if let Some((min, max)) = model.export_region {
                                    // Auto-update settings from model if we are in Viewport mode
                                    self.settings.source = ExportSource::Viewport(min, max);
                                    ui.label(format!(
                                        "Selected: {:.0},{:.0} - {:.0},{:.0}",
                                        min.x, min.y, max.x, max.y
                                    ));
                                } else {
                                    ui.label("No region selected.");
                                    ui.label("Run 'Actions > Select Export Region' first.");
                                }
                            }
                        });

                        ui.add_space(20.0);

                        // Actions
                        ui.horizontal(|ui| {
                            if ui.button("Export...").clicked() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("PDF", &["pdf"])
                                    .save_file()
                                {
                                    if let Err(e) =
                                        PdfExporter::export_to_file(model, &self.settings, &path)
                                    {
                                        eprintln!("Export failed: {}", e);
                                    } else {
                                        close_window = true;
                                    }
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                close_window = true;
                            }
                        });
                    });
                });
            });

        if close_window {
            self.open = false;
        }
    }

    fn draw_preview(ui: &mut egui::Ui, settings: &ExportSettings, model: &CadModel) {
        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(400.0, 500.0), // Fixed preview area size
            egui::Sense::hover(),
        );

        // Draw background
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, egui::Color32::from_gray(50));

        // Draw Page representation
        let (page_w_mm, page_h_mm) = settings.page_size.dimensions_mm();
        let (w_mm, h_mm) = if matches!(settings.orientation, PageOrientation::Landscape) {
            (page_h_mm, page_w_mm)
        } else {
            (page_w_mm, page_h_mm)
        };

        // Calculate fit scale for preview window
        let preview_scale = (rect.width() / w_mm).min(rect.height() / h_mm) * 0.9;

        let display_w = w_mm * preview_scale;
        let display_h = h_mm * preview_scale;

        let page_rect =
            egui::Rect::from_center_size(rect.center(), egui::vec2(display_w, display_h));

        painter.rect_filled(page_rect, 0.0, egui::Color32::WHITE);

        // Draw Content inside Page Rect
        let (min_bound, max_bound) = match settings.source {
            ExportSource::ModelBounds => {
                let mut min_b = Vector2::new(f32::MAX, f32::MAX);
                let mut max_b = Vector2::new(f32::MIN, f32::MIN);
                let mut has = false;
                for e in &model.entities {
                    has = true;
                    match e {
                        Entity::Line(l) => {
                            min_b.x = min_b.x.min(l.start.x).min(l.end.x);
                            min_b.y = min_b.y.min(l.start.y).min(l.end.y);
                            max_b.x = max_b.x.max(l.start.x).max(l.end.x);
                            max_b.y = max_b.y.max(l.start.y).max(l.end.y);
                        }
                        Entity::Circle(c) => {
                            min_b.x = min_b.x.min(c.center.x - c.radius);
                            min_b.y = min_b.y.min(c.center.y - c.radius);
                            max_b.x = max_b.x.max(c.center.x + c.radius);
                            max_b.y = max_b.y.max(c.center.y + c.radius);
                        }
                        Entity::Arc(a) => {
                            min_b.x = min_b.x.min(a.center.x - a.radius);
                            min_b.y = min_b.y.min(a.center.y - a.radius);
                            max_b.x = max_b.x.max(a.center.x + a.radius);
                            max_b.y = max_b.y.max(a.center.y + a.radius);
                        }
                        Entity::Rectangle(r) => {
                            min_b.x = min_b.x.min(r.min.x);
                            min_b.y = min_b.y.min(r.min.y);
                            max_b.x = max_b.x.max(r.max.x);
                            max_b.y = max_b.y.max(r.max.y);
                        }
                        Entity::Text(t) => {
                            min_b.x = min_b.x.min(t.position.x);
                            min_b.y = min_b.y.min(t.position.y);
                            max_b.x = max_b.x.max(t.position.x);
                            max_b.y = max_b.y.max(t.position.y);
                        }
                    }
                }
                if !has {
                    (Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0))
                } else {
                    // Add small padding to avoid zero-size
                    if (max_b.x - min_b.x).abs() < 1.0 {
                        max_b.x = min_b.x + 1.0;
                    }
                    if (max_b.y - min_b.y).abs() < 1.0 {
                        max_b.y = min_b.y + 1.0;
                    }
                    (min_b, max_b)
                }
            }
            ExportSource::Viewport(min, max) => (min, max),
        };

        let content_width = max_bound.x - min_bound.x;
        let content_height = max_bound.y - min_bound.y;

        let margin_mm = settings.margin_mm;
        let print_w = w_mm - margin_mm * 2.0;
        let print_h = h_mm - margin_mm * 2.0;

        let scale = match settings.scale_type {
            ScaleType::FitToPage => {
                let s_x = print_w / content_width;
                let s_y = print_h / content_height;
                s_x.min(s_y)
            }
            ScaleType::Standard(r) => 1.0 / r,
            ScaleType::Custom(s) => 1.0 / s,
        };

        let pdf_content_w = content_width * scale;
        let pdf_content_h = content_height * scale;

        let pdf_off_x = margin_mm + (print_w - pdf_content_w) / 2.0;
        let pdf_off_y = margin_mm + (print_h - pdf_content_h) / 2.0;

        let transform_point = |p: Vector2| -> egui::Pos2 {
            let rel_x = p.x - min_bound.x;
            let rel_y = p.y - min_bound.y;
            let pdf_x = pdf_off_x + rel_x * scale;
            let pdf_y = pdf_off_y + rel_y * scale;
            let screen_x = page_rect.min.x + pdf_x * preview_scale;
            let screen_y = page_rect.max.y - pdf_y * preview_scale; // Y-up flip
            egui::pos2(screen_x, screen_y)
        };

        if model.entities.is_empty() {
            ui.label("No entities to preview.");
        } else {
            // println!("Preview: {} entities. Bounds: {:?} - {:?}", model.entities.len(), min_bound, max_bound);
        }

        let mut shapes = Vec::new();

        for entity in &model.entities {
            match entity {
                Entity::Line(line) => {
                    let p1 = transform_point(line.start);
                    let p2 = transform_point(line.end);
                    shapes.push(egui::Shape::line_segment(
                        [p1, p2],
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                    ));
                }
                Entity::Circle(circle) => {
                    // Approximate circle with line segments
                    let segments = 32;
                    let mut points = Vec::with_capacity(segments + 1);
                    for i in 0..=segments {
                        let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
                        let x = circle.center.x + circle.radius * angle.cos();
                        let y = circle.center.y + circle.radius * angle.sin();
                        points.push(transform_point(Vector2::new(x, y)));
                    }
                    for i in 0..segments {
                        shapes.push(egui::Shape::line_segment(
                            [points[i], points[i + 1]],
                            egui::Stroke::new(1.0, egui::Color32::BLACK),
                        ));
                    }
                }
                Entity::Arc(arc) => {
                    // Approximate arc with line segments
                    let segments = 24;
                    let mut start_angle = arc.start_angle;
                    let mut end_angle = arc.end_angle;

                    // Handle wrap-around
                    if end_angle < start_angle {
                        end_angle += std::f32::consts::PI * 2.0;
                    }

                    let mut points = Vec::with_capacity(segments + 1);
                    for i in 0..=segments {
                        let t = i as f32 / segments as f32;
                        let angle = start_angle + t * (end_angle - start_angle);
                        let x = arc.center.x + arc.radius * angle.cos();
                        let y = arc.center.y + arc.radius * angle.sin();
                        points.push(transform_point(Vector2::new(x, y)));
                    }
                    for i in 0..segments {
                        shapes.push(egui::Shape::line_segment(
                            [points[i], points[i + 1]],
                            egui::Stroke::new(1.0, egui::Color32::BLACK),
                        ));
                    }
                }
                Entity::Rectangle(rect) => {
                    let p1 = transform_point(rect.min);
                    let p2 = transform_point(Vector2::new(rect.max.x, rect.min.y));
                    let p3 = transform_point(rect.max);
                    let p4 = transform_point(Vector2::new(rect.min.x, rect.max.y));

                    shapes.push(egui::Shape::line_segment(
                        [p1, p2],
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                    ));
                    shapes.push(egui::Shape::line_segment(
                        [p2, p3],
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                    ));
                    shapes.push(egui::Shape::line_segment(
                        [p3, p4],
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                    ));
                    shapes.push(egui::Shape::line_segment(
                        [p4, p1],
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                    ));
                }
                Entity::Text(text) => {
                    let p = transform_point(text.position);
                    painter.text(
                        p,
                        egui::Align2::CENTER_CENTER,
                        &text.text,
                        egui::FontId::proportional(10.0),
                        egui::Color32::BLACK,
                    );
                }
            }
        }
        painter.extend(shapes);
    }
}
