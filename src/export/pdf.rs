use super::settings::{ExportSettings, ExportSource, PageOrientation, ScaleType};
use crate::model::{CadModel, Entity, Vector2};
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
            ExportSource::ModelBounds => {
                let mut min_b = Vector2::new(f32::MAX, f32::MAX);
                let mut max_b = Vector2::new(f32::MIN, f32::MIN);
                let mut has_entities = false;

                for entity in &model.entities {
                    match entity {
                        Entity::Line(l) => {
                            min_b.x = min_b.x.min(l.start.x).min(l.end.x);
                            min_b.y = min_b.y.min(l.start.y).min(l.end.y);
                            max_b.x = max_b.x.max(l.start.x).max(l.end.x);
                            max_b.y = max_b.y.max(l.start.y).max(l.end.y);
                            has_entities = true;
                        }
                        Entity::Circle(c) => {
                            min_b.x = min_b.x.min(c.center.x - c.radius);
                            min_b.y = min_b.y.min(c.center.y - c.radius);
                            max_b.x = max_b.x.max(c.center.x + c.radius);
                            max_b.y = max_b.y.max(c.center.y + c.radius);
                            has_entities = true;
                        }
                        Entity::Arc(a) => {
                            min_b.x = min_b.x.min(a.center.x - a.radius);
                            min_b.y = min_b.y.min(a.center.y - a.radius);
                            max_b.x = max_b.x.max(a.center.x + a.radius);
                            max_b.y = max_b.y.max(a.center.y + a.radius);
                            has_entities = true;
                        }
                        _ => {}
                    }
                }

                if !has_entities {
                    (Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0))
                } else {
                    (min_b, max_b)
                }
            }
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
        for entity in &model.entities {
            match entity {
                Entity::Line(line) => {
                    let start = transform(line.start);
                    let end = transform(line.end);

                    let points = vec![
                        (Point::new(Mm(start.0), Mm(start.1)), false),
                        (Point::new(Mm(end.0), Mm(end.1)), false),
                    ];

                    let line_surf = Line {
                        points,
                        is_closed: false,
                        has_fill: false,
                        has_stroke: true,
                        is_clipping_path: false,
                    };

                    let outline_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
                    current_layer.set_outline_color(outline_color);
                    current_layer.set_outline_thickness(0.5);
                    current_layer.add_shape(line_surf);
                }
                Entity::Circle(circle) => {
                    // Approximate circle with line segments
                    let segments = 32;
                    let mut points = Vec::with_capacity(segments + 1);
                    for i in 0..=segments {
                        let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
                        let x = circle.center.x + circle.radius * angle.cos();
                        let y = circle.center.y + circle.radius * angle.sin();
                        let pt = transform(Vector2::new(x, y));
                        points.push((Point::new(Mm(pt.0), Mm(pt.1)), false));
                    }

                    let circle_shape = Line {
                        points,
                        is_closed: true,
                        has_fill: circle.filled,
                        has_stroke: true,
                        is_clipping_path: false,
                    };

                    let outline_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
                    current_layer.set_outline_color(outline_color);
                    current_layer.set_outline_thickness(0.5);
                    current_layer.add_shape(circle_shape);
                }
                Entity::Arc(arc) => {
                    // Approximate arc with line segments
                    let segments = 24;
                    let start_angle = arc.start_angle;
                    let mut end_angle = arc.end_angle;

                    if end_angle < start_angle {
                        end_angle += std::f32::consts::PI * 2.0;
                    }

                    let mut points = Vec::with_capacity(segments + 1);
                    for i in 0..=segments {
                        let t = i as f32 / segments as f32;
                        let angle = start_angle + t * (end_angle - start_angle);
                        let x = arc.center.x + arc.radius * angle.cos();
                        let y = arc.center.y + arc.radius * angle.sin();
                        let pt = transform(Vector2::new(x, y));
                        points.push((Point::new(Mm(pt.0), Mm(pt.1)), false));
                    }

                    let arc_shape = Line {
                        points,
                        is_closed: false,
                        has_fill: false,
                        has_stroke: true,
                        is_clipping_path: false,
                    };

                    let outline_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
                    current_layer.set_outline_color(outline_color);
                    current_layer.set_outline_thickness(0.5);
                    current_layer.add_shape(arc_shape);
                }
                Entity::Rectangle(rect) => {
                    let p1 = transform(rect.min);
                    let p2 = transform(Vector2::new(rect.max.x, rect.min.y));
                    let p3 = transform(rect.max);
                    let p4 = transform(Vector2::new(rect.min.x, rect.max.y));

                    let points = vec![
                        (Point::new(Mm(p1.0), Mm(p1.1)), false),
                        (Point::new(Mm(p2.0), Mm(p2.1)), false),
                        (Point::new(Mm(p3.0), Mm(p3.1)), false),
                        (Point::new(Mm(p4.0), Mm(p4.1)), false),
                        (Point::new(Mm(p1.0), Mm(p1.1)), false), // Close
                    ];

                    let rect_shape = Line {
                        points,
                        is_closed: true,
                        has_fill: rect.filled,
                        has_stroke: true,
                        is_clipping_path: false,
                    };

                    let outline_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
                    current_layer.set_outline_color(outline_color);
                    current_layer.set_outline_thickness(0.5);
                    current_layer.add_shape(rect_shape);
                }
                Entity::Text(text) => {
                    let pos = transform(text.position);
                    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
                    current_layer.use_text(text.text.clone(), 10.0, Mm(pos.0), Mm(pos.1), &font);
                }
                // Structural elements - render as filled rectangles
                Entity::Column(col) => {
                    let w = 50.0; // Default width
                    let h = 50.0; // Default depth
                    let corners = col.get_corners(w, h);

                    let mut points = Vec::with_capacity(5);
                    for c in &corners {
                        let pt = transform(*c);
                        points.push((Point::new(Mm(pt.0), Mm(pt.1)), false));
                    }
                    points.push(points[0].clone()); // Close

                    let shape = Line {
                        points,
                        is_closed: true,
                        has_fill: true,
                        has_stroke: true,
                        is_clipping_path: false,
                    };

                    let outline_color = Color::Rgb(Rgb::new(0.5, 0.5, 0.5, None));
                    current_layer.set_outline_color(outline_color);
                    current_layer.set_outline_thickness(0.5);
                    current_layer.add_shape(shape);
                }
                Entity::Beam(beam) => {
                    let width = 30.0;
                    let corners = beam.get_corners(width);

                    let mut points = Vec::with_capacity(5);
                    for c in &corners {
                        let pt = transform(*c);
                        points.push((Point::new(Mm(pt.0), Mm(pt.1)), false));
                    }
                    points.push(points[0].clone());

                    let shape = Line {
                        points,
                        is_closed: true,
                        has_fill: true,
                        has_stroke: true,
                        is_clipping_path: false,
                    };

                    let outline_color = Color::Rgb(Rgb::new(0.4, 0.4, 0.4, None));
                    current_layer.set_outline_color(outline_color);
                    current_layer.set_outline_thickness(0.5);
                    current_layer.add_shape(shape);
                }
                Entity::Flooring(floor) => {
                    if floor.boundary_points.len() >= 3 {
                        let mut points = Vec::with_capacity(floor.boundary_points.len() + 1);
                        for p in &floor.boundary_points {
                            let pt = transform(*p);
                            points.push((Point::new(Mm(pt.0), Mm(pt.1)), false));
                        }
                        points.push(points[0].clone());

                        let shape = Line {
                            points,
                            is_closed: true,
                            has_fill: true,
                            has_stroke: true,
                            is_clipping_path: false,
                        };

                        let outline_color = Color::Rgb(Rgb::new(0.7, 0.7, 0.7, None));
                        current_layer.set_outline_color(outline_color);
                        current_layer.set_outline_thickness(0.3);
                        current_layer.add_shape(shape);
                    }
                }
                Entity::Door(_) | Entity::Window(_) => {
                    // TODO: Implement door/window symbols in PDF
                }
            }
        }

        let mut file = BufWriter::new(File::create(path)?);
        doc.save(&mut file)?;

        Ok(())
    }
}
