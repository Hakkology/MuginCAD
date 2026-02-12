//! Core Data Model
//!
//! This module defines the core data structures of the CAD application.
//! It aims to be pure data and logic, independent of the view or specific UI frameworks.
//!
//! Key components:
//! - `Entity`: A node in the scene hierarchy (id, name, shape, children).
//! - `Shape`: The geometric primitive (Line, Circle, etc.) or `None` for containers.
//! - `CadModel`: The container for all entities in a project.
//! - `AxisManager`: Architectural grid system.
//! - `Vector2`: Basic math primitives.

pub mod axis;
pub mod math;
pub mod shapes;
pub mod system;
pub mod tools;

pub use math::vector;
pub use system::config;
pub use system::project;
pub use tools::snap;
pub use tools::undo;

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

pub use shapes::annotation::TextAnnotation;
pub use shapes::arc::Arc;
pub use shapes::circle::Circle;
pub use shapes::line::Line;
pub use shapes::rectangle::Rectangle;
pub use vector::Vector2;

/// Global atomic counter for unique entity IDs.
static NEXT_ENTITY_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    NEXT_ENTITY_ID.fetch_add(1, Ordering::Relaxed)
}

// ─── Shape ──────────────────────────────────────────────────────

/// The geometric primitive of an entity, or `None` for empty containers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    /// Empty container — has no geometry, only used as a hierarchy parent.
    None,
    Line(Line),
    Circle(Circle),
    Rectangle(Rectangle),
    Arc(Arc),
    Text(TextAnnotation),
}

impl Shape {
    pub fn type_name(&self) -> &'static str {
        match self {
            Shape::None => "Empty",
            Shape::Line(_) => "Line",
            Shape::Circle(_) => "Circle",
            Shape::Rectangle(_) => "Rectangle",
            Shape::Arc(_) => "Arc",
            Shape::Text(_) => "Text",
        }
    }
}

// ─── Entity ─────────────────────────────────────────────────────

/// A node in the scene hierarchy.
///
/// Every entity has a unique `id`, a display `name`, an optional geometric
/// `shape` (may be `Shape::None` for pure containers), and recursive `children`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
    pub name: String,
    pub shape: Shape,
    pub children: Vec<Entity>,
}

impl Entity {
    // ── Constructors ────────────────────────────────────────

    /// Create an entity with the given shape. Name defaults to the shape type.
    pub fn new(shape: Shape) -> Self {
        let name = shape.type_name().to_string();
        Self {
            id: next_id(),
            name,
            shape,
            children: Vec::new(),
        }
    }

    /// Create an empty container entity.
    pub fn empty(name: impl Into<String>) -> Self {
        Self {
            id: next_id(),
            name: name.into(),
            shape: Shape::None,
            children: Vec::new(),
        }
    }

    // ── Factory helpers ─────────────────────────────────────

    pub fn line(start: Vector2, end: Vector2) -> Self {
        Self::new(Shape::Line(Line::new(start, end)))
    }

    pub fn circle(center: Vector2, radius: f32, filled: bool) -> Self {
        Self::new(Shape::Circle(Circle::new(center, radius, filled)))
    }

    pub fn rectangle(min: Vector2, max: Vector2, filled: bool) -> Self {
        Self::new(Shape::Rectangle(Rectangle::new(min, max, filled)))
    }

    pub fn arc(arc: Arc) -> Self {
        Self::new(Shape::Arc(arc))
    }

    pub fn text(annotation: TextAnnotation) -> Self {
        Self::new(Shape::Text(annotation))
    }

    // ── Queries ─────────────────────────────────────────────

    pub fn type_name(&self) -> &str {
        self.shape.type_name()
    }

    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        // Check own shape
        let self_hit = match &self.shape {
            Shape::None => false,
            Shape::Line(line) => line.hit_test(pos, tolerance),
            Shape::Circle(circle) => circle.hit_test(pos, tolerance),
            Shape::Rectangle(rect) => rect.hit_test(pos, tolerance),
            Shape::Arc(arc) => arc.hit_test(pos, tolerance),
            Shape::Text(text) => text.hit_test(pos, tolerance),
        };
        if self_hit {
            return true;
        }
        // Check children
        self.children.iter().any(|c| c.hit_test(pos, tolerance))
    }

    pub fn center(&self) -> Vector2 {
        match &self.shape {
            Shape::Line(line) => Vector2::new(
                (line.start.x + line.end.x) / 2.0,
                (line.start.y + line.end.y) / 2.0,
            ),
            Shape::Circle(circle) => circle.center,
            Shape::Rectangle(rect) => Vector2::new(
                (rect.min.x + rect.max.x) / 2.0,
                (rect.min.y + rect.max.y) / 2.0,
            ),
            Shape::Arc(arc) => arc.center,
            Shape::Text(text) => text.position,
            Shape::None => {
                let (min, max) = self.bounding_box();
                Vector2::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0)
            }
        }
    }

    // ── Transforms ──────────────────────────────────────────

    pub fn translate(&mut self, delta: Vector2) {
        match &mut self.shape {
            Shape::Line(line) => {
                line.start = line.start + delta;
                line.end = line.end + delta;
            }
            Shape::Circle(circle) => {
                circle.center = circle.center + delta;
            }
            Shape::Rectangle(rect) => {
                rect.min = rect.min + delta;
                rect.max = rect.max + delta;
            }
            Shape::Arc(arc) => {
                arc.center = arc.center + delta;
            }
            Shape::Text(text) => {
                text.position = text.position + delta;
                for pt in &mut text.anchor_points {
                    *pt = *pt + delta;
                }
            }
            Shape::None => {}
        }
        for child in &mut self.children {
            child.translate(delta);
        }
    }

    pub fn rotate(&mut self, pivot: Vector2, angle: f32) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let rotate_point = |p: Vector2| -> Vector2 {
            let dx = p.x - pivot.x;
            let dy = p.y - pivot.y;
            Vector2::new(
                pivot.x + dx * cos_a - dy * sin_a,
                pivot.y + dx * sin_a + dy * cos_a,
            )
        };

        match &mut self.shape {
            Shape::Line(line) => {
                line.start = rotate_point(line.start);
                line.end = rotate_point(line.end);
            }
            Shape::Circle(circle) => {
                circle.center = rotate_point(circle.center);
            }
            Shape::Rectangle(rect) => {
                let p1 = rotate_point(rect.min);
                let p2 = rotate_point(Vector2::new(rect.max.x, rect.min.y));
                let p3 = rotate_point(rect.max);
                let p4 = rotate_point(Vector2::new(rect.min.x, rect.max.y));

                rect.min = Vector2::new(
                    p1.x.min(p2.x).min(p3.x).min(p4.x),
                    p1.y.min(p2.y).min(p3.y).min(p4.y),
                );
                rect.max = Vector2::new(
                    p1.x.max(p2.x).max(p3.x).max(p4.x),
                    p1.y.max(p2.y).max(p3.y).max(p4.y),
                );
            }
            Shape::Arc(arc) => {
                arc.center = rotate_point(arc.center);
                arc.start_angle += angle;
                arc.end_angle += angle;
            }
            Shape::Text(text) => {
                text.position = rotate_point(text.position);
                for pt in &mut text.anchor_points {
                    *pt = rotate_point(*pt);
                }
            }
            Shape::None => {}
        }
        for child in &mut self.children {
            child.rotate(pivot, angle);
        }
    }

    pub fn scale(&mut self, base: Vector2, factor: f32) {
        let scale_point = |p: Vector2| -> Vector2 {
            Vector2::new(
                base.x + (p.x - base.x) * factor,
                base.y + (p.y - base.y) * factor,
            )
        };

        match &mut self.shape {
            Shape::Line(line) => {
                line.start = scale_point(line.start);
                line.end = scale_point(line.end);
            }
            Shape::Circle(circle) => {
                circle.center = scale_point(circle.center);
                circle.radius *= factor;
            }
            Shape::Rectangle(rect) => {
                rect.min = scale_point(rect.min);
                rect.max = scale_point(rect.max);
            }
            Shape::Arc(arc) => {
                arc.center = scale_point(arc.center);
                arc.radius *= factor;
            }
            Shape::Text(text) => {
                text.position = scale_point(text.position);
                for pt in &mut text.anchor_points {
                    *pt = scale_point(*pt);
                }
                text.style.font_size *= factor;
            }
            Shape::None => {}
        }
        for child in &mut self.children {
            child.scale(base, factor);
        }
    }

    // ── Geometry helpers ────────────────────────────────────

    /// Returns the axis-aligned bounding box as `(min, max)`.
    pub fn bounding_box(&self) -> (Vector2, Vector2) {
        let shape_bb = match &self.shape {
            Shape::Line(l) => Some((
                Vector2::new(l.start.x.min(l.end.x), l.start.y.min(l.end.y)),
                Vector2::new(l.start.x.max(l.end.x), l.start.y.max(l.end.y)),
            )),
            Shape::Circle(c) => Some((
                Vector2::new(c.center.x - c.radius, c.center.y - c.radius),
                Vector2::new(c.center.x + c.radius, c.center.y + c.radius),
            )),
            Shape::Arc(a) => Some((
                Vector2::new(a.center.x - a.radius, a.center.y - a.radius),
                Vector2::new(a.center.x + a.radius, a.center.y + a.radius),
            )),
            Shape::Rectangle(r) => Some((
                Vector2::new(r.min.x.min(r.max.x), r.min.y.min(r.max.y)),
                Vector2::new(r.min.x.max(r.max.x), r.min.y.max(r.max.y)),
            )),
            Shape::Text(t) => Some((t.position, t.position)),
            Shape::None => None,
        };

        let mut min_b;
        let mut max_b;

        if let Some((s_min, s_max)) = shape_bb {
            min_b = s_min;
            max_b = s_max;
        } else {
            min_b = Vector2::new(f32::MAX, f32::MAX);
            max_b = Vector2::new(f32::MIN, f32::MIN);
        }

        for child in &self.children {
            let (c_min, c_max) = child.bounding_box();
            min_b.x = min_b.x.min(c_min.x);
            min_b.y = min_b.y.min(c_min.y);
            max_b.x = max_b.x.max(c_max.x);
            max_b.y = max_b.y.max(c_max.y);
        }

        (min_b, max_b)
    }

    /// Convert the entity to a polyline (list of points).
    pub fn as_polyline(&self) -> Vec<Vector2> {
        let mut pts = match &self.shape {
            Shape::Line(l) => vec![l.start, l.end],
            Shape::Circle(c) => {
                let segments = 32;
                (0..=segments)
                    .map(|i| {
                        let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
                        Vector2::new(
                            c.center.x + c.radius * angle.cos(),
                            c.center.y + c.radius * angle.sin(),
                        )
                    })
                    .collect()
            }
            Shape::Arc(a) => {
                let segments = 24;
                let start_angle = a.start_angle;
                let mut end_angle = a.end_angle;
                if end_angle < start_angle {
                    end_angle += std::f32::consts::PI * 2.0;
                }
                (0..=segments)
                    .map(|i| {
                        let t = i as f32 / segments as f32;
                        let angle = start_angle + t * (end_angle - start_angle);
                        Vector2::new(
                            a.center.x + a.radius * angle.cos(),
                            a.center.y + a.radius * angle.sin(),
                        )
                    })
                    .collect()
            }
            Shape::Rectangle(r) => {
                vec![
                    r.min,
                    Vector2::new(r.max.x, r.min.y),
                    r.max,
                    Vector2::new(r.min.x, r.max.y),
                    r.min,
                ]
            }
            Shape::Text(t) => vec![t.position],
            Shape::None => Vec::new(),
        };
        for child in &self.children {
            pts.extend(child.as_polyline());
        }
        pts
    }

    /// Whether this entity represents a closed shape.
    pub fn is_closed(&self) -> bool {
        matches!(self.shape, Shape::Circle(_) | Shape::Rectangle(_)) || !self.children.is_empty()
    }

    /// Whether this entity has a fill.
    pub fn is_filled(&self) -> bool {
        match &self.shape {
            Shape::Circle(c) => c.filled,
            Shape::Rectangle(r) => r.filled,
            Shape::Arc(a) => a.filled,
            _ => self.children.iter().any(|c| c.is_filled()),
        }
    }

    // ── Hierarchy helpers ───────────────────────────────────

    /// Find a descendant entity by id (recursive).
    pub fn find_by_id(&self, target_id: u64) -> Option<&Entity> {
        if self.id == target_id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_by_id(target_id) {
                return Some(found);
            }
        }
        None
    }

    /// Find a descendant entity by id (mutable, recursive).
    pub fn find_by_id_mut(&mut self, target_id: u64) -> Option<&mut Entity> {
        if self.id == target_id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_by_id_mut(target_id) {
                return Some(found);
            }
        }
        None
    }
    /// Pick an entity ID at the given position (recursive).
    /// Returns the ID of the deepest child that was hit.
    pub fn pick(&self, pos: Vector2, tolerance: f32) -> Option<u64> {
        // Check children first (render order usually means children are on top)
        for child in self.children.iter().rev() {
            if let Some(id) = child.pick(pos, tolerance) {
                return Some(id);
            }
        }

        // Check self
        if self.hit_test(pos, tolerance) {
            return Some(self.id);
        }

        None
    }
}

// ─── CadModel ───────────────────────────────────────────────────

pub struct CadModel {
    pub entities: Vec<Entity>,
    pub axis_manager: axis::AxisManager,
    pub export_region: Option<(Vector2, Vector2)>,
}

impl CadModel {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            axis_manager: axis::AxisManager::new(),
            export_region: None,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    /// Find the top-most entity ID under the cursor (recursive).
    pub fn pick_entity_id(&self, pos: Vector2, tolerance: f32) -> Option<u64> {
        // Iterate reversely (top-most rendered first)
        for entity in self.entities.iter().rev() {
            if let Some(id) = entity.pick(pos, tolerance) {
                return Some(id);
            }
        }
        None
    }

    /// Find entity by id across the whole tree.
    pub fn find_by_id(&self, id: u64) -> Option<&Entity> {
        for entity in &self.entities {
            if let Some(found) = entity.find_by_id(id) {
                return Some(found);
            }
        }
        None
    }

    /// Find entity by id (mutable) across the whole tree.
    pub fn find_by_id_mut(&mut self, id: u64) -> Option<&mut Entity> {
        for entity in &mut self.entities {
            if let Some(found) = entity.find_by_id_mut(id) {
                return Some(found);
            }
        }
        None
    }

    /// Compute the bounding box of all entities.
    pub fn bounds(&self) -> (Vector2, Vector2) {
        if self.entities.is_empty() {
            return (Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0));
        }

        let mut min_b = Vector2::new(f32::MAX, f32::MAX);
        let mut max_b = Vector2::new(f32::MIN, f32::MIN);

        for entity in &self.entities {
            let (e_min, e_max) = entity.bounding_box();
            min_b.x = min_b.x.min(e_min.x);
            min_b.y = min_b.y.min(e_min.y);
            max_b.x = max_b.x.max(e_max.x);
            max_b.y = max_b.y.max(e_max.y);
        }

        if (max_b.x - min_b.x).abs() < 1.0 {
            max_b.x = min_b.x + 1.0;
        }
        if (max_b.y - min_b.y).abs() < 1.0 {
            max_b.y = min_b.y + 1.0;
        }

        (min_b, max_b)
    }

    /// Remove entities by a set of IDs (recursive).
    /// Returns the number of entities removed.
    pub fn remove_entities_by_ids(&mut self, ids: &std::collections::HashSet<u64>) -> usize {
        let mut count = 0;
        count += Self::remove_recursive(&mut self.entities, ids);
        count
    }

    fn remove_recursive(entities: &mut Vec<Entity>, ids: &std::collections::HashSet<u64>) -> usize {
        let mut count = 0;

        // 1. Remove from current level
        let initial_len = entities.len();
        entities.retain(|e| !ids.contains(&e.id));
        count += initial_len - entities.len();

        // 2. Recurse into remaining children
        for entity in entities.iter_mut() {
            count += Self::remove_recursive(&mut entity.children, ids);
        }

        count
    }

    /// Find the set of selected IDs that do not have an ancestor also selected.
    /// This prevents double-transformations when a parent and child are both selected.
    pub fn get_top_level_selected_ids(
        &self,
        selected_ids: &std::collections::HashSet<u64>,
    ) -> Vec<u64> {
        let mut top_level = Vec::new();
        for entity in &self.entities {
            Self::collect_top_level(entity, selected_ids, &mut top_level);
        }
        top_level
    }

    fn collect_top_level(
        entity: &Entity,
        selected_ids: &std::collections::HashSet<u64>,
        acc: &mut Vec<u64>,
    ) {
        if selected_ids.contains(&entity.id) {
            acc.push(entity.id);
            // Stop recursion: this entity handles itself and all its children
            return;
        }

        for child in &entity.children {
            Self::collect_top_level(child, selected_ids, acc);
        }
    }
}
