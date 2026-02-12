//! Hierarchy panel — renders the entity tree in a side panel.
//!
//! Converts the `CadModel` entity list into `TreeNode`s and uses the
//! reusable `mugin_widgets::hierarchy` widget to display them.

use crate::model::{Entity, Shape};
use crate::viewmodel::CadViewModel;
use eframe::egui;
use mugin_widgets::hierarchy::TreeNode;
use std::collections::HashSet;

/// Render the hierarchy panel contents.
pub fn render_hierarchy(ui: &mut egui::Ui, vm: &mut CadViewModel) {
    ui.heading("Hierarchy");
    ui.separator();

    if vm.tabs.is_empty() {
        ui.label("No open tabs.");
        return;
    }

    // 1. Prepare data (ReadOnly block to release borrows)
    let (nodes, selected_ids_set, has_selection) = {
        let tab = &vm.tabs[vm.active_tab_index];
        let entities = &tab.model.entities;

        if entities.is_empty() {
            ui.label(
                egui::RichText::new("No entities")
                    .italics()
                    .color(egui::Color32::GRAY),
            );
            // Even if empty, we want to show toolbar?
            // If empty, let's just make nodes empty.
            (Vec::new(), HashSet::new(), false)
        } else {
            let nodes: Vec<TreeNode> = entities.iter().map(entity_to_node).collect();
            let sel = tab.selection_manager.selected_ids.clone();
            let has_sel = !sel.is_empty();
            (nodes, sel, has_sel)
        }
    };

    // 2. Toolbar (Mutates vm)
    ui.horizontal(|ui| {
        if ui.button("📁 New Group").clicked() {
            let tab = &mut vm.tabs[vm.active_tab_index];
            tab.model.add_entity(Entity::empty("New Group"));
        }

        ui.add_enabled_ui(has_selection, |ui| {
            if ui.button("🗑 Delete").clicked() {
                let tab = &mut vm.tabs[vm.active_tab_index];
                let ids = tab.selection_manager.selected_ids.clone();
                if !ids.is_empty() {
                    tab.model.remove_entities_by_ids(&ids);
                    tab.selection_manager.selected_ids.clear();
                }
            }
        });
    });
    ui.separator();

    // 3. Render tree
    let response = mugin_widgets::hierarchy::tree_view(ui, &nodes, &selected_ids_set);

    // Update hierarchy_renaming flag so terminal doesn't steal focus
    vm.hierarchy_renaming = response.is_renaming;

    // Handle clicks
    if let Some(clicked_id) = response.clicked_id {
        let tab = &mut vm.tabs[vm.active_tab_index];
        let modifiers = ui.input(|i| i.modifiers);
        if modifiers.shift || modifiers.ctrl || modifiers.command {
            if tab.selection_manager.selected_ids.contains(&clicked_id) {
                tab.selection_manager.selected_ids.remove(&clicked_id);
            } else {
                tab.selection_manager.selected_ids.insert(clicked_id);
            }
        } else {
            tab.selection_manager.selected_ids.clear();
            tab.selection_manager.selected_ids.insert(clicked_id);
        }
    }

    // Handle renames — update entity name
    if let Some((renamed_id, new_name)) = response.renamed {
        let tab = &mut vm.tabs[vm.active_tab_index];
        if let Some(entity) = tab.model.entities.iter_mut().find(|e| e.id == renamed_id) {
            entity.name = new_name;
        }
    }

    // Handle reparenting — move entity under another entity (or to root)
    if let Some((dragged_id, target_id)) = response.reparent {
        let tab = &mut vm.tabs[vm.active_tab_index];

        // 1. Remove the dragged entity from wherever it is
        if let Some(removed) = remove_entity_by_id(&mut tab.model.entities, dragged_id) {
            match target_id {
                Some(target) => {
                    // Move under target
                    if let Some(parent) = find_entity_mut(&mut tab.model.entities, target) {
                        parent.children.push(removed);
                    } else {
                        // Target not found, put back at root
                        tab.model.entities.push(removed);
                    }
                }
                None => {
                    // Move to root
                    tab.model.entities.push(removed);
                }
            }

            // Clear selection since indices changed
            // Clear selection since indices changed (actually IDs don't change, but let's clear for safety or keep?)
            // If we keep IDs, the selection follows variables!
            // But if we moved them, they are still same entities.
            // Let's NOT clear selection, so user can keep dragging?
            // Original code cleared it. Let's clear it to be safe.
            tab.selection_manager.selected_ids.clear();
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────

/// Convert an `Entity` into a `TreeNode` for the hierarchy widget.
fn entity_to_node(entity: &Entity) -> TreeNode {
    let icon = shape_icon(&entity.shape);
    let children = entity.children.iter().map(entity_to_node).collect();

    TreeNode {
        id: entity.id,
        label: entity.name.clone(),
        icon,
        children,
    }
}

/// Map a `Shape` variant to a display icon.
fn shape_icon(shape: &Shape) -> &'static str {
    match shape {
        Shape::None => "📁",
        Shape::Line(_) => "📏",
        Shape::Circle(_) => "⭕",
        Shape::Rectangle(_) => "▭",
        Shape::Arc(_) => "◠",
        Shape::Text(_) => "🔤",
    }
}

/// Remove an entity by id from a tree (recursive). Returns the removed entity if found.
fn remove_entity_by_id(entities: &mut Vec<Entity>, id: u64) -> Option<Entity> {
    // Check top-level
    if let Some(pos) = entities.iter().position(|e| e.id == id) {
        return Some(entities.remove(pos));
    }

    // Check children recursively
    for entity in entities.iter_mut() {
        if let Some(removed) = remove_entity_by_id(&mut entity.children, id) {
            return Some(removed);
        }
    }

    None
}

/// Find an entity by id in the tree (mutable, recursive).
fn find_entity_mut(entities: &mut [Entity], id: u64) -> Option<&mut Entity> {
    for entity in entities.iter_mut() {
        if entity.id == id {
            return Some(entity);
        }
        if let Some(found) = find_entity_mut(&mut entity.children, id) {
            return Some(found);
        }
    }
    None
}
