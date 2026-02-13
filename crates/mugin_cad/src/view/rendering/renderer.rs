use crate::model::Entity;
use crate::model::structure::definitions::StructureDefinitions;
use crate::view::rendering::context::DrawContext;

pub fn render_entities(
    ctx: &DrawContext,
    definitions: &StructureDefinitions,
    entities: &[Entity],
    selected_ids: &std::collections::HashSet<u64>,
    hovered_entity_id: Option<u64>,
    layer_manager: &crate::model::layer::LayerManager,
) {
    for entity in entities {
        entity.render_recursive(
            ctx,
            definitions,
            selected_ids,
            hovered_entity_id,
            layer_manager,
        );
    }
}
