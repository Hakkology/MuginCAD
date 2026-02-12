use super::settings::{ExportSettings, ExportSource, PageOrientation, ScaleType};
use crate::model::{CadModel, Shape, Vector2};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct PdfExporter;

impl PdfExporter {
    pub fn export_to_file(
        model: &CadModel,
        settings: &ExportSettings,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Setup page size
        let (mut width_mm, mut height_mm) = settings.page_size.dimensions_mm();
        if matches!(settings.orientation, PageOrientation::Landscape) {
            std::mem::swap(&mut width_mm, &mut height_mm);
        }

        let (doc, page1, layer1) = PdfDocument::new(
            "CAD Export",
            Mm(width_mm as f64),
            Mm(height_mm as f64),
            "CAD Layer",
        );
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // 1. Calculate Source Bounds
        let (min_bound, max_bound) = match settings.source {
            ExportSource::ModelBounds => model.bounds(),
            ExportSource::Viewport(min, max) => (min, max),
        };

        let content_width = max_bound.x - min_bound.x;
        let content_height = max_bound.y - min_bound.y;

        // 2. Calculate Scale
        let margin = settings.margin_mm;
        let print_width = width_mm - margin * 2.0;
        let print_height = height_mm - margin * 2.0;

        let scale = match settings.scale_type {
            ScaleType::FitToPage => {
                let scale_x = print_width / content_width;
                let scale_y = print_height / content_height;
                scale_x.min(scale_y)
            }
            ScaleType::Standard(ratio) => 1.0 / ratio,
            ScaleType::Custom(s) => 1.0 / s,
        };

        // Center content
        let scaled_w = content_width * scale;
        let scaled_h = content_height * scale;

        let offset_x = margin + (print_width - scaled_w) / 2.0;
        let offset_y = margin + (print_height - scaled_h) / 2.0;

        // Transform function: CAD (x,y) -> PDF (mm, mm)
        let transform = |p: Vector2| -> (f64, f64) {
            let rel_x = p.x - min_bound.x;
            let rel_y = p.y - min_bound.y;
            let pdf_x = offset_x + rel_x * scale;
            let pdf_y = offset_y + rel_y * scale;
            (pdf_x as f64, pdf_y as f64)
        };

        // 3. Draw Entities
        let outline_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
        current_layer.set_outline_color(outline_color);
        current_layer.set_outline_thickness(0.5);

        for entity in &model.entities {
            match &entity.shape {
                Shape::Text(text) => {
                    let pos = transform(text.position);
                    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
                    current_layer.use_text(text.text.clone(), 10.0, Mm(pos.0), Mm(pos.1), &font);
                }
                _ => {
                    let polyline = entity.as_polyline();
                    let points: Vec<(Point, bool)> = polyline
                        .iter()
                        .map(|p| {
                            let (x, y) = transform(*p);
                            (Point::new(Mm(x), Mm(y)), false)
                        })
                        .collect();

                    let shape = Line {
                        points,
                        is_closed: entity.is_closed(),
                        has_fill: entity.is_filled(),
                        has_stroke: true,
                        is_clipping_path: false,
                    };

                    current_layer.add_shape(shape);
                }
            }
        }

        let mut file = BufWriter::new(File::create(path)?);
        doc.save(&mut file)?;

        Ok(())
    }
}
