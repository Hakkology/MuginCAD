use crate::commands::{Command, CommandCategory, CommandContext, InputModifiers, PointResult};
use crate::model::{Entity, Vector2};

/// Base structure for manipulation commands (Move, Rotate, Scale, etc.)
/// Stores original entity state and provides common functionality
#[derive(Debug, Clone)]
pub struct ManipulationBase {
    /// Index of the entity being manipulated
    pub entity_idx: Option<usize>,
    /// Original entity state (for undo/cancel)
    pub original_entity: Option<Entity>,
    /// Base point for the manipulation
    pub base_point: Option<Vector2>,
    /// Points collected during manipulation
    pub points: Vec<Vector2>,
}

impl ManipulationBase {
    pub fn new() -> Self {
        Self {
            entity_idx: None,
            original_entity: None,
            base_point: None,
            points: Vec::new(),
        }
    }

    /// Initialize from context - captures entity state
    pub fn init_from_context(&mut self, ctx: &CommandContext) {
        self.entity_idx = ctx.selected_entity_idx;
        if let Some(idx) = ctx.selected_entity_idx {
            if let Some(entity) = ctx.model.entities.get(idx) {
                self.original_entity = Some(entity.clone());
            }
        }
    }

    /// Get the entity center point
    pub fn get_entity_center(&self, model: &crate::model::CadModel) -> Option<Vector2> {
        if let Some(idx) = self.entity_idx {
            if let Some(entity) = model.entities.get(idx) {
                return Some(entity.center());
            }
        }
        None
    }

    /// Restore original entity (for cancel)
    pub fn restore_original(&self, model: &mut crate::model::CadModel) {
        if let (Some(idx), Some(original)) = (self.entity_idx, &self.original_entity) {
            if idx < model.entities.len() {
                model.entities[idx] = original.clone();
            }
        }
    }
}

/// Delegate trait for manipulation commands
/// Implement this to define specific manipulation behavior
pub trait ManipulationDelegate: std::fmt::Debug {
    /// Apply the manipulation to the entity
    fn apply(&self, entity: &mut Entity, from: Vector2, to: Vector2, modifiers: InputModifiers);

    /// Get the prompt for the second point
    fn second_point_prompt(&self) -> String;

    /// Get command name
    fn name(&self) -> &'static str;
}

/// Generic manipulation command that uses a delegate
#[derive(Debug, Clone)]
pub struct ManipulationCommand<D: ManipulationDelegate + Clone> {
    pub base: ManipulationBase,
    pub delegate: D,
}

impl<D: ManipulationDelegate + Clone + 'static> ManipulationCommand<D> {
    pub fn new(delegate: D) -> Self {
        Self {
            base: ManipulationBase::new(),
            delegate,
        }
    }
}

impl<D: ManipulationDelegate + Clone + 'static> Command for ManipulationCommand<D> {
    fn name(&self) -> &'static str {
        self.delegate.name()
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Manipulation
    }

    fn can_execute(&self, ctx: &CommandContext) -> bool {
        ctx.selected_entity_idx.is_some()
    }

    fn initial_prompt(&self) -> String {
        format!("{} Specify base point:", self.delegate.name())
    }

    fn cannot_execute_message(&self) -> String {
        "No entity selected. Select an entity first.".to_string()
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        self.base.init_from_context(ctx);
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        self.base.points.push(pos);

        if self.base.points.len() == 1 {
            self.base.base_point = Some(pos);
            PointResult::NeedMore {
                prompt: self.delegate.second_point_prompt(),
            }
        } else {
            // Apply the manipulation
            let from = self.base.base_point.unwrap();
            let to = pos;

            if let Some(idx) = self.base.entity_idx {
                if let Some(entity) = ctx.model.entities.get_mut(idx) {
                    self.delegate.apply(entity, from, to, ctx.modifiers);
                }
            }

            PointResult::Complete
        }
    }

    fn get_points(&self) -> &[Vector2] {
        &self.base.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
