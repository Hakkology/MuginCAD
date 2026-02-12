use crate::export::pdf::PdfExporter;
use crate::export::settings::{ExportSettings, ExportSource, PageOrientation, PageSize, ScaleType};
use crate::model::{CadModel, Shape, Vector2};
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
        let (rect, _response) =
            ui.allocate_exact_size(egui::vec2(400.0, 500.0), egui::Sense::hover());

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

        let preview_scale = (rect.width() / w_mm).min(rect.height() / h_mm) * 0.9;
        let display_w = w_mm * preview_scale;
        let display_h = h_mm * preview_scale;

        let page_rect =
            egui::Rect::from_center_size(rect.center(), egui::vec2(display_w, display_h));
        painter.rect_filled(page_rect, 0.0, egui::Color32::WHITE);

        // Calculate bounds
        let (min_bound, max_bound) = match settings.source {
            ExportSource::ModelBounds => model.bounds(),
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
            return;
        }

        // Draw entities using polyline conversion
        let stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
        let mut shapes = Vec::new();

        for entity in &model.entities {
            match &entity.shape {
                Shape::Text(text) => {
                    let p = transform_point(text.position);
                    painter.text(
                        p,
                        egui::Align2::CENTER_CENTER,
                        &text.text,
                        egui::FontId::proportional(10.0),
                        egui::Color32::BLACK,
                    );
                }
                _ => {
                    let polyline = entity.as_polyline();
                    let screen_pts: Vec<egui::Pos2> =
                        polyline.iter().map(|p| transform_point(*p)).collect();

                    for pair in screen_pts.windows(2) {
                        shapes.push(egui::Shape::line_segment([pair[0], pair[1]], stroke));
                    }
                }
            }
        }

        painter.extend(shapes);
    }
}
