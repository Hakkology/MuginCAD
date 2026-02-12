//! Reusable tree-view widget for hierarchy panels.
//!
//! Supports collapsible nodes, single-click selection,
//! double-click inline rename, and drag-and-drop reparenting.

use eframe::egui;
use std::collections::HashSet;

// ─── Data ────────────────────────────────────────────────────────────────

/// A single node in the tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub id: u64,
    pub label: String,
    pub icon: &'static str,
    pub children: Vec<TreeNode>,
}

/// Response from the tree view widget.
#[derive(Default)]
pub struct TreeResponse {
    pub clicked_id: Option<u64>,
    pub renamed: Option<(u64, String)>,
    /// (dragged_ids, target_id) — None target = move to root.
    pub reparent: Option<(Vec<u64>, Option<u64>)>,
    pub is_renaming: bool,
}

/// Persistent key for the node ids being dragged.
/// Payload: (ids: Vec<u64>, label: String)
const DRAG_KEY: &str = "tree_drag_ids";

// ─── Widget ──────────────────────────────────────────────────────────────

pub fn tree_view(
    ui: &mut egui::Ui,
    nodes: &[TreeNode],
    selected_ids: &HashSet<u64>,
) -> TreeResponse {
    let mut response = TreeResponse::default();

    for node in nodes {
        render_node(ui, node, selected_ids, &mut response, 0);
    }

    // Retrieve drag state
    let drag_state = ui.data(|d| d.get_temp::<(Vec<u64>, String)>(egui::Id::new(DRAG_KEY)));
    let drag_ids = drag_state.as_ref().map(|(ids, _)| ids.clone());

    // Root-level drop zone
    if drag_ids.is_some() {
        let (_, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 14.0));
        let hovered = ui.rect_contains_pointer(rect);

        if hovered {
            ui.painter().rect_filled(
                rect,
                2.0,
                egui::Color32::from_rgba_premultiplied(100, 200, 100, 80),
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "⬇ Move to Root",
                egui::FontId::proportional(10.0),
                egui::Color32::from_rgb(150, 255, 150),
            );
        }

        if hovered && ui.input(|i| i.pointer.any_released()) {
            if let Some(ids) = drag_ids {
                response.reparent = Some((ids, None));
            }
        }
    }

    // Ghost visual
    if let Some((_, label)) = drag_state {
        if let Some(pointer_pos) = ui.ctx().pointer_hover_pos() {
            let painter = ui.ctx().layer_painter(egui::LayerId::new(
                egui::Order::Tooltip,
                egui::Id::new("drag_ghost"),
            ));

            let text = format!("Move: {}", label);
            // Draw text with background
            let galley = painter.layout_no_wrap(
                text,
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );

            let rect = egui::Rect::from_min_size(
                pointer_pos + egui::vec2(10.0, 10.0),
                galley.size() + egui::vec2(10.0, 5.0),
            );

            painter.rect_filled(rect, 3.0, egui::Color32::from_black_alpha(200));
            painter.galley(
                rect.min + egui::vec2(5.0, 2.5),
                galley,
                egui::Color32::BLACK,
            );
        }
    }

    // Clear drag state on pointer release
    if ui.input(|i| i.pointer.any_released()) {
        ui.data_mut(|d| d.remove_temp::<(Vec<u64>, String)>(egui::Id::new(DRAG_KEY)));
    }

    response
}

// ─── Internal ────────────────────────────────────────────────────────────

fn render_node(
    ui: &mut egui::Ui,
    node: &TreeNode,
    selected_ids: &HashSet<u64>,
    response: &mut TreeResponse,
    depth: usize,
) {
    let is_selected = selected_ids.contains(&node.id);
    let has_children = !node.children.is_empty();
    let indent = depth as f32 * 16.0;

    // Rename state
    let rename_key = ui.make_persistent_id(format!("ren_{}", node.id));
    let mut renaming: bool = ui.data(|d| d.get_temp(rename_key)).unwrap_or(false);
    if renaming {
        response.is_renaming = true;
    }

    // Collapse state
    let col_id = ui.make_persistent_id(format!("col_{}", node.id));
    let is_open = if has_children {
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), col_id, true)
            .is_open()
    } else {
        false
    };

    // Drag state
    let drag_key = egui::Id::new(DRAG_KEY);
    let drag_state = ui.data(|d| d.get_temp::<(Vec<u64>, String)>(drag_key));
    let drag_ids = drag_state.as_ref().map(|(ids, _)| ids.clone()); // Check if ANY of drag_ids contains node.id

    // Check if this node is being dragged
    let is_me_dragged = if let Some(ids) = &drag_ids {
        ids.contains(&node.id)
    } else {
        false
    };

    // ── Row ──────────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.add_space(indent);

        // Selection highlight
        if is_selected && !is_me_dragged {
            let r = ui.available_rect_before_wrap();
            ui.painter().rect_filled(
                r,
                2.0,
                egui::Color32::from_rgba_premultiplied(60, 120, 200, 80),
            );
        }

        // Drop target highlight
        // Only valid if I am NOT one of the dragged items
        let is_drop_target = drag_ids.is_some()
            && !is_me_dragged
            && ui.rect_contains_pointer(ui.available_rect_before_wrap());

        if is_drop_target {
            let r = ui.available_rect_before_wrap();
            ui.painter().rect_stroke(
                r,
                2.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 220, 100)),
            );
            // Extra visual feedback for drop
            ui.painter().rect_filled(
                r,
                2.0,
                egui::Color32::from_rgba_premultiplied(100, 220, 100, 40),
            );
        }

        // Dim if being dragged
        if is_me_dragged {
            ui.set_opacity(0.4);
        }

        // Collapse arrow
        if has_children {
            let arrow = if is_open { "▼" } else { "▶" };
            if ui
                .add(
                    egui::Label::new(
                        egui::RichText::new(arrow)
                            .size(10.0)
                            .color(egui::Color32::GRAY),
                    )
                    .sense(egui::Sense::click()),
                )
                .clicked()
            {
                let mut st = egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    col_id,
                    true,
                );
                st.toggle(ui);
                st.store(ui.ctx());
            }
        } else {
            ui.label(
                egui::RichText::new("•")
                    .size(10.0)
                    .color(egui::Color32::DARK_GRAY),
            );
        }

        // Icon
        ui.label(node.icon);

        // Label / rename
        if renaming {
            let buf_key = ui.make_persistent_id(format!("ren_buf_{}", node.id));
            let mut buf: String = ui
                .data(|d| d.get_temp(buf_key))
                .unwrap_or_else(|| node.label.clone());

            let te = ui.add(
                egui::TextEdit::singleline(&mut buf)
                    .desired_width(120.0)
                    .font(egui::TextStyle::Body),
            );
            te.request_focus();

            if te.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                renaming = false;
                if !buf.is_empty() {
                    response.renamed = Some((node.id, buf.clone()));
                }
            }
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                renaming = false;
            }
            ui.data_mut(|d| d.insert_temp(buf_key, buf));
        } else {
            let text = if is_selected {
                egui::RichText::new(&node.label)
                    .color(egui::Color32::WHITE)
                    .strong()
            } else {
                egui::RichText::new(&node.label).color(egui::Color32::LIGHT_GRAY)
            };

            let r = ui.add(egui::Label::new(text).sense(egui::Sense::click_and_drag()));

            if r.clicked() {
                // Determine modifier state
                // This logic is actually handled in the parent, but we return clicked_id.
                // However, we need to know if we are starting a drag of the SELECTION or just this item.
                response.clicked_id = Some(node.id);
            }
            if r.double_clicked() {
                renaming = true;
                let buf_key = ui.make_persistent_id(format!("ren_buf_{}", node.id));
                ui.data_mut(|d| d.insert_temp(buf_key, node.label.clone()));
            }

            // Start drag
            if r.drag_started() {
                // Logic:
                // 1. If 'node.id' is in 'selected_ids', then we drag ALL selected ids.
                // 2. If 'node.id' is NOT in 'selected_ids', we drag JUST 'node.id'.

                let (ids_to_drag, label) = if selected_ids.contains(&node.id) {
                    let list: Vec<u64> = selected_ids.iter().cloned().collect();
                    // Sort or ensure order? Iter order is random for HashSet.
                    // Ideally we'd respect visual order, but that's hard here.
                    // Just collecting is fine for now.
                    // Ensure node.id is present (it is by definition).

                    let label = if list.len() > 1 {
                        format!("{} items", list.len())
                    } else {
                        node.label.clone()
                    };
                    (list, label)
                } else {
                    (vec![node.id], node.label.clone())
                };

                ui.data_mut(|d| d.insert_temp(drag_key, (ids_to_drag, label)));
            }
        }

        // Drop on this node
        if is_drop_target && ui.input(|i| i.pointer.any_released()) {
            if let Some(dids) = drag_ids {
                // Don't drop 'node' onto itself
                if !dids.contains(&node.id) {
                    response.reparent = Some((dids, Some(node.id)));
                    ui.data_mut(|d| d.remove_temp::<(Vec<u64>, String)>(drag_key));
                }
            }
        }
    });

    ui.data_mut(|d| d.insert_temp(rename_key, renaming));

    // Children
    if has_children && is_open {
        for child in &node.children {
            render_node(ui, child, selected_ids, response, depth + 1);
        }
    }
}
