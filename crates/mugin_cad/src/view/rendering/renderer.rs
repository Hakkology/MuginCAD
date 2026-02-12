use crate::model::Entity;
use crate::view::context::DrawContext;
use crate::view::rendering::renderable::Renderable;

pub fn render_entities(
    ctx: &DrawContext,
    entities: &[Entity],
    selected_ids: &std::collections::HashSet<u64>,
    hovered_entity_id: Option<u64>,
) {
    for entity in entities {
        entity.render_recursive(ctx, selected_ids, hovered_entity_id);
    }
}
