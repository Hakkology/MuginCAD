use super::vector::Vector2;
use crate::model::CadModel;
use crate::model::Entity;
use std::f32::consts::PI;

// Removed DirectedEdge struct as it's no longer used.

/// Finds the smallest closed loop surrounding the given point.
/// Returns a tuple: (List of Entity Indices, List of Polygon Vertices).
pub fn find_closed_region(model: &CadModel, p: Vector2) -> Option<(Vec<usize>, Vec<Vector2>)> {
    // 1. Ray Cast Check to find nearest edge to the Right (+X direction)
    let ray_dir = Vector2::new(1.0, 0.0);
    let mut closest_hit: Option<(usize, Vector2)> = None;
    let mut min_dist = f32::MAX;

    for (i, entity) in model.entities.iter().enumerate() {
        if let Some(pos) = intersect_ray_entity(p, ray_dir, entity) {
            let dist = pos.x - p.x;
            if dist > 0.0001 && dist < min_dist {
                min_dist = dist;
                closest_hit = Some((i, pos));
            }
        }
    }

    let (start_entity_idx, _hit_pos) = closest_hit?;

    // 2. Build Topology Graph
    // Snap tolerance
    let tolerance = 0.01;
    let mut nodes: Vec<Vector2> = Vec::new(); // Unique points
    let mut adj: Vec<Vec<EdgeInfo>> = Vec::new();

    // Helper to intern points
    let mut get_node_idx = |pos: Vector2| -> usize {
        for (i, n) in nodes.iter().enumerate() {
            if n.dist(pos) < tolerance {
                return i;
            }
        }
        nodes.push(pos);
        nodes.len() - 1
    };

    for (i, entity) in model.entities.iter().enumerate() {
        let (start, end) = match entity {
            Entity::Line(l) => (l.start, l.end),
            Entity::Arc(a) => {
                let s = Vector2::new(
                    a.center.x + a.radius * a.start_angle.cos(),
                    a.center.y + a.radius * a.start_angle.sin(),
                );
                let e = Vector2::new(
                    a.center.x + a.radius * a.end_angle.cos(),
                    a.center.y + a.radius * a.end_angle.sin(),
                );
                (s, e)
            }
            _ => continue,
        };

        let u = get_node_idx(start);
        // Ensure adj grows with nodes
        if u >= adj.len() {
            adj.resize_with(u + 1, Vec::new);
        }

        let v = get_node_idx(end);
        if v >= adj.len() {
            adj.resize_with(v + 1, Vec::new);
        }

        // Add edge logic (bidirectional)
        adj[u].push(EdgeInfo {
            entity_idx: i,
            target: v,
            is_arc: matches!(entity, Entity::Arc(_)),
        });
        adj[v].push(EdgeInfo {
            entity_idx: i,
            target: u,
            is_arc: matches!(entity, Entity::Arc(_)),
        });
    }

    // 3. Determine Start Direction
    let start_entity = &model.entities[start_entity_idx];
    let (p_a, p_b) = match start_entity {
        Entity::Line(l) => (l.start, l.end),
        // Basic arc handling: Treat as chord for direction check?
        // For exact check, we need tangent at intersection.
        // Assume Line for MVP logic or approximate.
        Entity::Arc(a) => {
            // Approximation: use start/end chord
            let s = Vector2::new(
                a.center.x + a.radius * a.start_angle.cos(),
                a.center.y + a.radius * a.start_angle.sin(),
            );
            let e = Vector2::new(
                a.center.x + a.radius * a.end_angle.cos(),
                a.center.y + a.radius * a.end_angle.sin(),
            );
            (s, e)
        }
        _ => return None,
    };

    let vec_ab = p_b - p_a;
    let cross = ray_dir.x * vec_ab.y - ray_dir.y * vec_ab.x;

    // Find the node indices for p_a and p_b
    let u_idx = get_node_idx(p_a); // "re-getting" returns existing index
    let v_idx = get_node_idx(p_b);

    // If cross > 0, ray hits "right side" of vector AB, so we must go A->B to be CCW?
    // Ray (1,0). AB (0, 1) [Up]. Cross = 1. Left turn. Correct.
    // We walk along the wall. Wall is on our Left? No, Inside is Left.
    // If Ray came from inside, Ray hits interior wall.
    // Traversing CCW means Left Wall.
    // So we follow vector that is Left of Ray? (0,1) is left of (1,0).
    // So if Cross > 0, we follow AB. Next node is B (v_idx).

    let mut curr_node = if cross > 0.0 { v_idx } else { u_idx };
    let start_node = if cross > 0.0 { u_idx } else { v_idx }; // Make sure we know where we truly started

    let mut path_indices = Vec::new();
    let mut path_vertices = Vec::new();

    path_indices.push(start_entity_idx);
    path_vertices.push(nodes[start_node]);
    // path_vertices.push(nodes[curr_node]); // Will add in loop

    let mut prev_node = start_node;
    let mut prev_entity = start_entity_idx;

    // 4. Traverse Graph
    for _ in 0..1000 {
        // Safety break
        if curr_node == start_node {
            // Closed loop found!
            path_vertices.push(nodes[curr_node]); // Close the polygon visually
            return Some((path_indices, path_vertices));
        }

        path_vertices.push(nodes[curr_node]);

        // Find best next edge
        let incoming_vec = nodes[curr_node] - nodes[prev_node];
        // We want the Left-most turn.
        // Smallest angle relative to incoming_vec (extended).
        // Or Largest angle?
        // Let's use `atan2`.
        // Current Heading = `incoming_vec`.
        // We want to turn Left.
        // Candidates `C`: `vec_NC`.
        // Relative Angle = `Angle(NC) - Angle(Heading)`.
        // Normalize to [-PI, PI].
        // "Left-most" means smallest positive angle? Or closest to PI?
        // We want to "hug the wall".
        // CCW traversal of interior -> Left turns are 90 deg. Right turns are 270 deg.
        // We want to turn LEFT as much as possible?
        // No, we want to stay inside.
        // The wall is on our Left.
        // If we hit a junction, and we turn sharp Left (e.g. 150 deg turn back), we stay close to the previous wall on our left.
        // If we turn Right, we might cross into the open.
        // So we pick the edge with the "most left" direction?
        // i.e. Maximize turn angle to the left?

        let heading_angle = incoming_vec.y.atan2(incoming_vec.x);

        // Wait, "Left" is positive angle in math.
        // We want the turn that is "sharpest left" -> Largest positive angle?
        // Actually, we want the smallest deviation from "Straight back"?
        // Standard algorithm: Select edge with smallest angle > PI?
        // Let's rely on: Sort edges CCW. Pick the next one after the incoming edge (reversed).

        // Incoming edge enters at `incoming_vec`.
        // In the local frame of `curr_node`, the incoming edge is at `heading_angle + PI` (since it came from prev).
        // We want the next edge in CW (Clockwise) or CCW order?
        // If we are inside, and traversing CCW, we turn Left at corners.
        // "Left" relative to incoming path.
        // So we scan CCW from `incoming_angle`. The first edge we hit is the one that turns "most right"?
        // Taking the "rightmost" branch keeps us on the outer boundary of the island?
        // Taking the "leftmost" branch keeps us in the hole?
        // We are outlining a Single region.
        // Rule: "Turn Right" to follow external boundary. "Turn Left" to follow internal?
        // If we are INSIDE a polygon, and walk CCW, the wall is on the RIGHT?
        // Wait. Standard polygon definition (OpenGL): CCW.
        // Vertices 1,2,3.
        // Walking 1->2->3. Center is to the Left.
        // Wall 1-2 is on the Right? No.
        // If I walk 1->2, and Center is Left, then I am looking at 2. My Left hand points to Center. My Right hand points to "Outside".
        // So Wall is on the Right.
        // Correct.
        // So we want to hug the wall on our Right.
        // "Hug Right" means at a junction, take the most Right turn.
        // "Right turn" = smallest angle difference in CW direction.
        // Or largest angle difference in CCW.

        // Let's calculate angle of outgoing edge relative to incoming vector.
        // `rel_angle = edge_angle - heading_angle`.
        // Normalize to `[0, 2PI)`.
        // We want the smallest `rel_angle` (closest to straight/right)?
        // Actually "Right" is negative angle.
        // `edge_angle` is `atan2`.
        // `diff = edge_angle - heading_angle`.
        // normalize to `(-PI, PI]`.
        // "Rightmost" means smallest (most negative) value.
        // "Leftmost" means largest (most positive) value.
        // If we want to hug wall on Right... we take the sharpest Right turn. i.e. Most negative angle?

        // Let's try: minimize `diff` (normalized to ensure we scan correctly).
        // Let's normalize to `[0, 2PI)` where 0 is Heading?
        // Then Right turn is `Warning: 3 quarter turns`. Left turn is `1 quarter turn`.
        // We want "Hard Right".
        // That corresponds to `diff` close to `2PI` (if 0 is straight).
        // Or `diff` close to `0` (if 0 is straight right?) No.

        // Simplest: `rel = (edge_angle - reverse_incoming_angle)`. Normalize to `[0, 2PI)`.
        // `reverse_incoming_angle` is where we came from.
        // We scan `[0, 2PI)` starting from there.
        // To hug Right wall, we take the *first* edge we find in CCW scan?
        // No, that's Leftmost.
        // We take the *last* edge in CCW scan? That's Rightmost.
        // Or scan CW?

        // Let's pick: "Edge with Update Angle closest to -PI (Right U-Turn)".

        // Actually simple Atan2 logic:
        // Compute Atan2 for all edges.
        // Compute `delta = edge_angle - incoming_angle`.
        // Normalize `delta` to `(-PI, PI]`.
        // "Rightmost" turn is the minimum `delta` value.

        let mut best_next_node = None;
        let mut best_edge_idx = 0;
        let mut min_delta = f32::MAX;

        for edge in &adj[curr_node] {
            if edge.target == prev_node && edge.entity_idx == prev_entity {
                continue;
            } // Don't go back immediately if alternatives exist (unless dead end)

            let vec = nodes[edge.target] - nodes[curr_node];
            let angle = vec.y.atan2(vec.x);

            let mut delta = angle - heading_angle;
            // Normalize to (-PI, PI]
            while delta <= -PI {
                delta += 2.0 * PI;
            }
            while delta > PI {
                delta -= 2.0 * PI;
            }

            // We want the "Right-most" turn.
            // Right is negative. Left is positive.
            // Smallest signed delta is the most Right.

            if delta < min_delta {
                min_delta = delta;
                best_next_node = Some(edge.target);
                best_edge_idx = edge.entity_idx;
            }
        }

        if let Some(next) = best_next_node {
            // Correct logic:
            prev_entity = best_edge_idx;
            prev_node = curr_node; // The node we are leaving
            curr_node = next;

            path_indices.push(best_edge_idx);
        } else {
            // Dead end
            return None;
        }
    }

    None
}

struct EdgeInfo {
    entity_idx: usize,
    target: usize,
    #[allow(dead_code)]
    is_arc: bool, // Reserved for potential arc-specific traversal logic
}

fn intersect_ray_entity(origin: Vector2, dir: Vector2, entity: &Entity) -> Option<Vector2> {
    match entity {
        Entity::Line(line) => {
            let p1 = line.start;
            let p2 = line.end;
            let v1 = origin - p1;
            let v2 = p2 - p1;
            let v3 = Vector2::new(-dir.y, dir.x);
            let dot = v2.dot(v3);
            if dot.abs() < 0.00001 {
                return None;
            }
            let t1 = (v2.x * v1.y - v2.y * v1.x) / dot; // Distance along Ray
            let t2 = (dir.x * v1.y - dir.y * v1.x) / dot; // Position on Line (0..1)

            if t1 >= 0.0 && t2 >= 0.0 && t2 <= 1.0 {
                return Some(origin + dir * t1);
            }
            None
        }
        Entity::Arc(arc) => {
            // Check arc intersection
            // Simple approach: Check circle intersection, then angle range.
            // Ray P + t*D.
            // |P + t*D - C|^2 = R^2
            // Let L = P - C.
            // |L + t*D|^2 = R^2
            // L.L + 2t(L.D) + t^2(D.D) = R^2
            // t^2 + 2(L.D)t + (L.L - R^2) = 0 (since D is unit vector? (1,0) yes)

            let l = origin - arc.center;
            let b = 2.0 * l.dot(dir);
            let c = l.dot(l) - arc.radius * arc.radius;
            let det = b * b - 4.0 * c;

            if det < 0.0 {
                return None;
            }
            let sqrt_det = det.sqrt();
            let t1 = (-b - sqrt_det) / 2.0;
            let t2 = (-b + sqrt_det) / 2.0;

            // Find smallest positive t
            let mut candidates = Vec::new();
            if t1 >= 0.0 {
                candidates.push(t1);
            }
            if t2 >= 0.0 {
                candidates.push(t2);
            }
            candidates.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for t in candidates {
                let pos = origin + dir * t;
                // Calculate angle for checking
                let angle = (pos.y - arc.center.y).atan2(pos.x - arc.center.x);

                // Normalize angles to [0, 2PI)
                let normalize = |a: f32| -> f32 {
                    let m = a % (2.0 * PI);
                    if m < 0.0 { m + 2.0 * PI } else { m }
                };

                let start = normalize(arc.start_angle);
                let end = normalize(arc.end_angle);
                let angle_norm = normalize(angle);

                let is_inside = if start < end {
                    angle_norm >= start && angle_norm <= end
                } else {
                    // Span crosses 0/2PI
                    angle_norm >= start || angle_norm <= end
                };

                if is_inside {
                    return Some(pos);
                }
            }
            None
        }
        _ => None,
    }
}

pub fn calculate_polygon_area(points: &[Vector2]) -> f32 {
    let mut area = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        area += points[i].x * points[j].y;
        area -= points[j].x * points[i].y;
    }
    (area / 2.0).abs()
}

pub fn calculate_path_perimeter(points: &[Vector2]) -> f32 {
    let mut perim = 0.0;
    for i in 0..points.len() - 1 {
        perim += points[i].dist(points[i + 1]);
    }
    // Check closure?
    // If indices suggest closed, points should reflect it.
    // Logic above pushes start_node at end.
    perim
}
