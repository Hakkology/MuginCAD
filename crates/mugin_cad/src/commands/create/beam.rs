use crate::commands::preview;
use crate::commands::{Command, CommandCategory, CommandContext, PointResult};
use crate::model::structure::beam::BeamData;
use crate::model::structure::beam_type::BeamType;
use crate::model::{Entity, Vector2};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct BeamCommand {
    points: Vec<Vector2>,
    /// 0=Center, 1=Top, 2=Bottom
    current_anchor_index: usize,
    active_beam_type_id: Option<u64>,
    cached_beam_type: Option<BeamType>,
}

impl BeamCommand {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            current_anchor_index: 0,
            active_beam_type_id: None,
            cached_beam_type: None,
        }
    }

    pub fn cycle_anchor(&mut self) {
        self.current_anchor_index = (self.current_anchor_index + 1) % 3;
    }

    pub fn rotate_cw(&mut self) {
        // For beams, rotation could swap ends, but let's just flip anchor side if not center
        if self.current_anchor_index == 1 {
            self.current_anchor_index = 2; // Top to Bottom
        } else if self.current_anchor_index == 2 {
            self.current_anchor_index = 1; // Bottom to Top
        }
    }
}

impl Command for BeamCommand {
    fn name(&self) -> &'static str {
        "Place Beam"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        // Use the active beam type from context if available
        if let Some(id) = ctx.active_beam_type_id {
            if let Some(beam_type) = ctx.model.definitions.beam_types.get(&id) {
                self.active_beam_type_id = Some(id);
                self.cached_beam_type = Some(beam_type.clone());
                return;
            }
        }

        // Fallback to first available if none selected or selection invalid
        if let Some((id, beam_type)) = ctx.model.definitions.beam_types.iter().next() {
            self.active_beam_type_id = Some(*id);
            self.cached_beam_type = Some(beam_type.clone());
        }
    }

    fn initial_prompt(&self) -> String {
        "Specify beam start point (Q: Anchor, E: Flip):".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        // Apply ortho constraint if shift is pressed
        let constrained_pos = if let Some(&last) = self.points.last() {
            self.constrain_point(pos, Some(last), ctx.modifiers)
        } else {
            pos
        };

        self.points.push(constrained_pos);

        if self.points.len() == 1 {
            return PointResult::NeedMore {
                prompt: "Specify beam end point:".to_string(),
            };
        }

        if self.points.len() >= 2 {
            let start = self.points[self.points.len() - 2];
            let end = self.points[self.points.len() - 1];

            let type_id = self.active_beam_type_id.unwrap_or(0);
            let name = self
                .cached_beam_type
                .as_ref()
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "Beam".to_string());

            let anchor = match self.current_anchor_index {
                1 => crate::model::structure::beam::BeamAnchor::Top,
                2 => crate::model::structure::beam::BeamAnchor::Bottom,
                _ => crate::model::structure::beam::BeamAnchor::Center,
            };

            let beam_data = BeamData::new(start, end, type_id, name, anchor);
            ctx.model.add_entity(Entity::beam(beam_data));

            // Chain placement like lines
            return PointResult::NeedMore {
                prompt: "Specify next beam point:".to_string(),
            };
        }

        PointResult::Complete
    }

    fn get_points(&self) -> &[Vector2] {
        &self.points
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }

    fn draw_preview(
        &self,
        ctx: &crate::view::rendering::context::DrawContext,
        points: &[Vector2],
        cursor_pos_cad: Vector2,
    ) {
        if let Some(&last_point) = points.last() {
            // Simple line preview first
            preview::draw_line_to_cursor(ctx, last_point, cursor_pos_cad);

            // If we have a cached beam type, we can draw a thicker preview
            if let Some(beam_type) = &self.cached_beam_type {
                let start_screen = ctx.to_screen(last_point);
                let end_screen = ctx.to_screen(cursor_pos_cad);
                let anchor = match self.current_anchor_index {
                    1 => crate::model::structure::beam::BeamAnchor::Top,
                    2 => crate::model::structure::beam::BeamAnchor::Bottom,
                    _ => crate::model::structure::beam::BeamAnchor::Center,
                };
                crate::view::rendering::structure::draw_beam(
                    ctx.painter,
                    start_screen,
                    end_screen,
                    ctx.zoom,
                    beam_type,
                    0.5, // 50% opacity for preview
                    anchor,
                );
            }
        }
    }
}
