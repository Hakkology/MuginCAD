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

pub use shapes::Geometry;
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

impl Geometry for Shape {
    fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        match self {
            Shape::None => false,
            Shape::Line(s) => s.hit_test(pos, tolerance),
            Shape::Circle(s) => s.hit_test(pos, tolerance),
            Shape::Rectangle(s) => s.hit_test(pos, tolerance),
            Shape::Arc(s) => s.hit_test(pos, tolerance),
            Shape::Text(s) => s.hit_test(pos, tolerance),
        }
    }

    fn center(&self) -> Vector2 {
        match self {
            Shape::None => Vector2::new(0.0, 0.0), // Placeholder, Entity handles this
            Shape::Line(s) => s.center(),
            Shape::Circle(s) => s.center(),
            Shape::Rectangle(s) => s.center(),
            Shape::Arc(s) => s.center(),
            Shape::Text(s) => s.center(),
        }
    }

    fn bounding_box(&self) -> (Vector2, Vector2) {
        match self {
            Shape::None => (
                Vector2::new(f32::MAX, f32::MAX),
                Vector2::new(f32::MIN, f32::MIN),
            ),
            Shape::Line(s) => s.bounding_box(),
            Shape::Circle(s) => s.bounding_box(),
            Shape::Rectangle(s) => s.bounding_box(),
            Shape::Arc(s) => s.bounding_box(),
            Shape::Text(s) => s.bounding_box(),
        }
    }

    fn as_polyline(&self) -> Vec<Vector2> {
        match self {
            Shape::None => Vec::new(),
            Shape::Line(s) => s.as_polyline(),
            Shape::Circle(s) => s.as_polyline(),
            Shape::Rectangle(s) => s.as_polyline(),
            Shape::Arc(s) => s.as_polyline(),
            Shape::Text(s) => s.as_polyline(),
        }
    }

    fn translate(&mut self, delta: Vector2) {
        match self {
            Shape::None => {}
            Shape::Line(s) => s.translate(delta),
            Shape::Circle(s) => s.translate(delta),
            Shape::Rectangle(s) => s.translate(delta),
            Shape::Arc(s) => s.translate(delta),
            Shape::Text(s) => s.translate(delta),
        }
    }

    fn rotate(&mut self, pivot: Vector2, angle: f32) {
        match self {
            Shape::None => {}
            Shape::Line(s) => s.rotate(pivot, angle),
            Shape::Circle(s) => s.rotate(pivot, angle),
            Shape::Rectangle(s) => s.rotate(pivot, angle),
            Shape::Arc(s) => s.rotate(pivot, angle),
            Shape::Text(s) => s.rotate(pivot, angle),
        }
    }

    fn scale(&mut self, base: Vector2, factor: f32) {
        match self {
            Shape::None => {}
            Shape::Line(s) => s.scale(base, factor),
            Shape::Circle(s) => s.scale(base, factor),
            Shape::Rectangle(s) => s.scale(base, factor),
            Shape::Arc(s) => s.scale(base, factor),
            Shape::Text(s) => s.scale(base, factor),
        }
    }

    fn is_closed(&self) -> bool {
        match self {
            Shape::None => false,
            Shape::Line(s) => s.is_closed(),
            Shape::Circle(s) => s.is_closed(),
            Shape::Rectangle(s) => s.is_closed(),
            Shape::Arc(s) => s.is_closed(),
            Shape::Text(s) => s.is_closed(),
        }
    }

    fn is_filled(&self) -> bool {
        match self {
            Shape::None => false,
            Shape::Line(s) => s.is_filled(),
            Shape::Circle(s) => s.is_filled(),
            Shape::Rectangle(s) => s.is_filled(),
            Shape::Arc(s) => s.is_filled(),
            Shape::Text(s) => s.is_filled(),
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
        if self.shape.hit_test(pos, tolerance) {
            return true;
        }
        // Check children
        self.children.iter().any(|c| c.hit_test(pos, tolerance))
    }

    pub fn center(&self) -> Vector2 {
        if matches!(self.shape, Shape::None) {
            let (min, max) = self.bounding_box();
            return Vector2::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0);
        }
        self.shape.center()
    }

    // ── Transforms ──────────────────────────────────────────

    pub fn translate(&mut self, delta: Vector2) {
        self.shape.translate(delta);
        for child in &mut self.children {
            child.translate(delta);
        }
    }

    pub fn rotate(&mut self, pivot: Vector2, angle: f32) {
        self.shape.rotate(pivot, angle);
        for child in &mut self.children {
            child.rotate(pivot, angle);
        }
    }

    pub fn scale(&mut self, base: Vector2, factor: f32) {
        self.shape.scale(base, factor);
        for child in &mut self.children {
            child.scale(base, factor);
        }
    }

    // ── Geometry helpers ────────────────────────────────────

    /// Returns the axis-aligned bounding box as `(min, max)`.
    pub fn bounding_box(&self) -> (Vector2, Vector2) {
        let (mut min_b, mut max_b) = self.shape.bounding_box();

        for child in &self.children {
            let (c_min, c_max) = child.bounding_box();
            min_b.x = min_b.x.min(c_min.x);
            min_b.y = min_b.y.min(c_min.y);
            max_b.x = max_b.x.max(c_max.x);
            max_b.y = max_b.y.max(c_max.y);
        }

        // Handle case where pure container has no children (reset to safe empty box or keep max/min inverted)
        // If min_b > max_b (meaning initialized to MAX/MIN and never updated), we return that.
        // It's up to caller to handle "infinite inverted box" or "default box".
        // The old implementation returned (MAX, MIN) effectively.
        (min_b, max_b)
    }

    /// Convert the entity to a polyline (list of points).
    pub fn as_polyline(&self) -> Vec<Vector2> {
        let mut pts = self.shape.as_polyline();
        for child in &self.children {
            pts.extend(child.as_polyline());
        }
        pts
    }

    /// Whether this entity represents a closed shape.
    pub fn is_closed(&self) -> bool {
        self.shape.is_closed() || !self.children.is_empty()
    }

    /// Whether this entity has a fill.
    pub fn is_filled(&self) -> bool {
        self.shape.is_filled() || self.children.iter().any(|c| c.is_filled())
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
