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
pub mod layer;
pub mod math;
pub mod shapes;
pub mod structure;
pub mod system;
pub mod tools;

pub use math::vector;
pub use system::config;
pub use system::project;
pub use tools::snap;
pub use tools::undo;

use glam::Affine2;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

pub use shapes::Geometry;
pub use shapes::annotation::TextAnnotation;
pub use shapes::arc::Arc;
pub use shapes::circle::Circle;
pub use shapes::line::Line;
pub use shapes::rectangle::Rectangle;
pub use structure::beam::BeamData;
pub use structure::column::ColumnData;
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
    Column(ColumnData),
    Beam(BeamData),
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
            Shape::Column(_) => "Column",
            Shape::Beam(_) => "Beam",
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
            Shape::Column(s) => s.hit_test(pos, tolerance),
            Shape::Beam(s) => s.hit_test(pos, tolerance),
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
            Shape::Column(s) => s.center(),
            Shape::Beam(s) => s.center(),
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
            Shape::Column(s) => s.bounding_box(),
            Shape::Beam(s) => s.bounding_box(),
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
            Shape::Column(s) => s.as_polyline(),
            Shape::Beam(s) => s.as_polyline(),
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
            Shape::Column(s) => s.translate(delta),
            Shape::Beam(s) => s.translate(delta),
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
            Shape::Column(s) => s.rotate(pivot, angle),
            Shape::Beam(s) => s.rotate(pivot, angle),
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
            Shape::Column(s) => s.scale(base, factor),
            Shape::Beam(s) => s.scale(base, factor),
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
            Shape::Column(s) => s.is_closed(),
            Shape::Beam(s) => s.is_closed(),
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
            Shape::Column(s) => s.is_filled(),
            Shape::Beam(s) => s.is_filled(),
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
    pub layer_id: u64,
    pub children: Vec<Entity>,

    /// Local transform relative to parent.
    pub local_transform: Affine2,
    /// Computed world transform.
    pub world_transform: Affine2,
    /// Whether the world transform needs recomputation.
    pub is_dirty: bool,
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
            layer_id: 0,
            children: Vec::new(),
            local_transform: Affine2::IDENTITY,
            world_transform: Affine2::IDENTITY,
            is_dirty: true,
        }
    }

    /// Create an empty container entity.
    pub fn empty(name: impl Into<String>) -> Self {
        Self {
            id: next_id(),
            name: name.into(),
            shape: Shape::None,
            layer_id: 0,
            children: Vec::new(),
            local_transform: Affine2::IDENTITY,
            world_transform: Affine2::IDENTITY,
            is_dirty: true,
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

    pub fn column(data: ColumnData) -> Self {
        Self::new(Shape::Column(data))
    }

    pub fn beam(data: BeamData) -> Self {
        Self::new(Shape::Beam(data))
    }

    // ── Queries ─────────────────────────────────────────────

    pub fn type_name(&self) -> &str {
        self.shape.type_name()
    }

    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        // Convert world pos to local pos
        let local_pos: Vector2 = self
            .world_transform
            .inverse()
            .transform_point2(pos.into())
            .into();

        // Check own shape
        if self.shape.hit_test(local_pos, tolerance) {
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
        let translation = Affine2::from_translation(delta.into());
        self.local_transform = translation * self.local_transform;
        self.set_dirty();
    }

    pub fn rotate(&mut self, pivot: Vector2, angle: f32) {
        // To rotate around a pivot in local space: T(pivot) * R(angle) * T(-pivot)
        let t1 = Affine2::from_translation(pivot.into());
        let r = Affine2::from_angle(angle);
        let t2 = Affine2::from_translation((-Vector2::from(pivot)).into());
        let rot_at_pivot = t1 * r * t2;

        self.local_transform = rot_at_pivot * self.local_transform;
        self.set_dirty();
    }

    pub fn scale(&mut self, base: Vector2, factor: f32) {
        let t1 = Affine2::from_translation(base.into());
        let s = Affine2::from_scale(glam::vec2(factor, factor));
        let t2 = Affine2::from_translation((-Vector2::from(base)).into());
        let scale_at_base = t1 * s * t2;

        self.local_transform = scale_at_base * self.local_transform;
        self.set_dirty();
    }

    // ── Geometry helpers ────────────────────────────────────

    /// Returns the axis-aligned bounding box as `(min, max)`.
    pub fn bounding_box(&self) -> (Vector2, Vector2) {
        let (local_min, local_max) = self.shape.bounding_box();

        let mut world_min = Vector2::new(f32::MAX, f32::MAX);
        let mut world_max = Vector2::new(f32::MIN, f32::MIN);

        // If shape is None, it won't affect the box expansion if we use MAX/MIN
        if !matches!(self.shape, Shape::None) {
            let corners = [
                Vector2::new(local_min.x, local_min.y),
                Vector2::new(local_max.x, local_min.y),
                Vector2::new(local_min.x, local_max.y),
                Vector2::new(local_max.x, local_max.y),
            ];

            for corner in corners {
                let transformed: Vector2 =
                    self.world_transform.transform_point2(corner.into()).into();
                world_min.x = world_min.x.min(transformed.x);
                world_min.y = world_min.y.min(transformed.y);
                world_max.x = world_max.x.max(transformed.x);
                world_max.y = world_max.y.max(transformed.y);
            }
        }

        for child in &self.children {
            let (c_min, c_max) = child.bounding_box();
            world_min.x = world_min.x.min(c_min.x);
            world_min.y = world_min.y.min(c_min.y);
            world_max.x = world_max.x.max(c_max.x);
            world_max.y = world_max.y.max(c_max.y);
        }

        (world_min, world_max)
    }

    /// Convert the entity to a polyline (list of points) in world space.
    pub fn as_polyline(&self) -> Vec<Vector2> {
        let mut pts: Vec<Vector2> = self
            .shape
            .as_polyline()
            .into_iter()
            .map(|p| self.world_transform.transform_point2(p.into()).into())
            .collect();

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
    /// Mark this entity and all its descendants as dirty.
    pub fn set_dirty(&mut self) {
        self.is_dirty = true;
        for child in &mut self.children {
            child.set_dirty();
        }
    }

    /// Recursively update world transforms.
    pub fn update_transforms(&mut self, parent_world: Affine2) {
        if self.is_dirty {
            self.world_transform = parent_world * self.local_transform;
            self.is_dirty = false;
        }

        for child in &mut self.children {
            child.update_transforms(self.world_transform);
        }
    }

    /// Pick an entity ID at the given position (recursive).
    /// Returns the ID of the deepest child that was hit.
    pub fn pick(
        &self,
        pos: Vector2,
        tolerance: f32,
        layer_manager: &crate::model::layer::LayerManager,
    ) -> Option<u64> {
        // Check children first (render order usually means children are on top)
        for child in self.children.iter().rev() {
            if let Some(id) = child.pick(pos, tolerance, layer_manager) {
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

use crate::model::structure::definitions::StructureDefinitions;

pub struct CadModel {
    pub entities: Vec<Entity>,
    pub axis_manager: axis::AxisManager,
    pub definitions: StructureDefinitions,
    pub layer_manager: layer::LayerManager,
    pub export_region: Option<(Vector2, Vector2)>,
}

impl CadModel {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            axis_manager: axis::AxisManager::new(),
            definitions: StructureDefinitions::new(),
            layer_manager: layer::LayerManager::new(),
            export_region: None,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    /// Update all transforms in the hierarchy.
    pub fn update_hierarchy(&mut self) {
        for entity in &mut self.entities {
            entity.update_transforms(Affine2::IDENTITY);
        }
    }

    /// Find the top-most entity ID under the cursor (recursive).
    pub fn pick_entity_id(&self, pos: Vector2, tolerance: f32) -> Option<u64> {
        // Iterate reversely (top-most rendered first)
        for entity in self.entities.iter().rev() {
            if let Some(id) = entity.pick(pos, tolerance, &self.layer_manager) {
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
