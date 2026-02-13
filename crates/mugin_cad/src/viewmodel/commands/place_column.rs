use crate::commands::{
    Command, CommandCategory, CommandContext, InputModifiers, InputResult, PointResult,
};
use crate::model::structure::column::ColumnData;
use crate::model::structure::column_type::ColumnType;
use crate::model::{Entity, Shape, Vector2};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct CmdPlaceColumn {
    points: Vec<Vector2>,
    /// 0=Center, 1=TL, 2=TR, 3=BR, 4=BL
    current_anchor_index: usize,
    rotation: f32,
    active_column_type_id: Option<u64>,
    /// We cache the column definition for preview rendering
    cached_col_type: Option<ColumnType>,
}

impl CmdPlaceColumn {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            current_anchor_index: 0,
            rotation: 0.0,
            active_column_type_id: None,
            cached_col_type: None,
        }
    }

    pub fn cycle_anchor(&mut self) {
        self.current_anchor_index = (self.current_anchor_index + 1) % 5;
    }

    pub fn rotate_cw(&mut self) {
        self.rotation += std::f32::consts::FRAC_PI_2; // 90 degrees
    }
}

impl Command for CmdPlaceColumn {
    fn name(&self) -> &'static str {
        "Place Column"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    fn on_start(&mut self, ctx: &CommandContext) {
        // Here we attempt to find the active column type.
        // Since we don't have direct access to VM here, we rely on `active_column_type_id`
        // being passed in conceptualy. But currently it's None.
        // We'll search `model.definitions` for *any* type if we have none.

        // Improve: The `CommandExecutor` could modify the command after creation if we cast it.
        // For now, let's just pick the first available one to ensure it works.
        if self.cached_col_type.is_none() {
            if let Some((id, col)) = ctx.model.definitions.column_types.iter().next() {
                self.active_column_type_id = Some(*id);
                self.cached_col_type = Some(col.clone());
            }
        }
    }

    fn initial_prompt(&self) -> String {
        "Specify insertion point (Q: Anchor, E: Rotate):".to_string()
    }

    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult {
        let (type_id, col_type) =
            if let (Some(id), Some(col)) = (self.active_column_type_id, &self.cached_col_type) {
                (id, col)
            } else {
                // Try to fetch again if missing
                if let Some((id, col)) = ctx.model.definitions.column_types.iter().next() {
                    self.active_column_type_id = Some(*id);
                    self.cached_col_type = Some(col.clone());
                    (*id, col)
                } else {
                    return PointResult::NeedMore {
                        prompt: "No column types defined!".to_string(),
                    };
                }
            };

        // Determine Center based on Anchor
        let width = col_type.width;
        let depth = col_type.depth;

        let half_w = width / 2.0;
        let half_h = depth / 2.0;

        let (sin, cos) = self.rotation.sin_cos();

        // Local anchor offset
        let local_anchor = match self.current_anchor_index {
            1 => Vector2::new(-half_w, -half_h), // TL
            2 => Vector2::new(half_w, -half_h),  // TR
            3 => Vector2::new(half_w, half_h),   // BR
            4 => Vector2::new(-half_w, half_h),  // BL
            _ => Vector2::new(0.0, 0.0),         // Center
        };

        let rotated_anchor = Vector2::new(
            local_anchor.x * cos - local_anchor.y * sin,
            local_anchor.x * sin + local_anchor.y * cos,
        );

        let final_center = pos - rotated_anchor;

        let mut col_data =
            ColumnData::new(final_center, width, depth, type_id, col_type.name.clone());
        col_data.rotation = self.rotation;

        ctx.model.add_entity(Entity::column(col_data));

        PointResult::Complete
    }

    fn constrain_point(
        &self,
        pos: Vector2,
        _last_point: Option<Vector2>,
        _modifiers: InputModifiers,
    ) -> Vector2 {
        pos
    }

    fn process_input(&mut self, input: &str, _ctx: &mut CommandContext) -> InputResult {
        InputResult::Continue
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
        _points: &[Vector2],
        cursor_pos_cad: Vector2,
    ) {
        if let Some(col_type) = &self.cached_col_type {
            // Calculate ghost position
            let width = col_type.width;
            let depth = col_type.depth;
            let half_w = width / 2.0;
            let half_h = depth / 2.0;

            let (sin, cos) = self.rotation.sin_cos();

            let local_anchor = match self.current_anchor_index {
                1 => Vector2::new(-half_w, -half_h),
                2 => Vector2::new(half_w, -half_h),
                3 => Vector2::new(half_w, half_h),
                4 => Vector2::new(-half_w, half_h),
                _ => Vector2::new(0.0, 0.0),
            };

            let rotated_anchor = Vector2::new(
                local_anchor.x * cos - local_anchor.y * sin,
                local_anchor.x * sin + local_anchor.y * cos,
            );

            let center_cad = cursor_pos_cad - rotated_anchor;
            let center_screen = ctx.to_screen(center_cad);

            // Use shared renderer
            // Zoom implies scale.
            // Scale = pixels per unit. DrawContext handles this internally for transforms,
            // but `draw_column` asks for scale explicitly.
            // ctx.zoom is "pixels per unit" I think?
            // Let's check `DrawContext`.
            let scale = ctx.zoom;

            crate::view::rendering::structure::draw_column(
                ctx.painter,
                center_screen,
                self.rotation, // Screen rotation?
                scale,
                col_type,
                0.5, // Alpha for ghost
            );
        }
    }
}
