use crate::model::Entity;
use crate::view::context::DrawContext;
use crate::view::rendering::renderable::Renderable;

pub fn render_entities(
    ctx: &DrawContext,
    entities: &[Entity],
    selected_indices: &std::collections::HashSet<usize>,
    hovered_entity_idx: Option<usize>,
) {
    for (i, entity) in entities.iter().enumerate() {
        let is_selected = selected_indices.contains(&i);
        let is_hovered = Some(i) == hovered_entity_idx;
        entity.render(ctx, is_selected, is_hovered);
    }
}
