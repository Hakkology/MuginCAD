use crate::model::{Circle, Entity, Line, Rectangle, Vector2};
use crate::viewmodel::CadViewModel;
use crate::viewmodel::state::{CommandState, CommandType};

pub fn handle_idle_token(vm: &mut CadViewModel, clean_token: &str) {
    match clean_token {
        "line" | "l" => {
            vm.state = CommandState::WaitingForPoints {
                cmd: CommandType::Line,
                points: Vec::new(),
            };
            vm.status_message = "LINE Specify first point:".to_string();
        }
        "circle" | "c" => {
            vm.state = CommandState::WaitingForPoints {
                cmd: CommandType::Circle,
                points: Vec::new(),
            };
            vm.status_message = "CIRCLE Specify center point:".to_string();
        }
        "rect" | "r" => {
            vm.state = CommandState::WaitingForPoints {
                cmd: CommandType::Rectangle,
                points: Vec::new(),
            };
            vm.status_message = "RECTANGLE Specify first corner:".to_string();
        }
        "fill" | "shade" => {
            vm.filled_mode = !vm.filled_mode;
            let mode = if vm.filled_mode { "ON" } else { "OFF" };
            vm.status_message = format!("SHADE mode: {}", mode);
            vm.command_history
                .push(format!("Shade mode is now {}", mode));
        }
        "clear" => {
            vm.model.entities.clear();
            vm.command_history.clear();
            vm.selected_entity_idx = None;
            vm.status_message = "Command:".to_string();
        }
        _ => {
            vm.status_message = format!("Unknown command \"{}\".", clean_token);
        }
    }
}

pub fn push_point(vm: &mut CadViewModel, pos: Vector2) {
    if let CommandState::WaitingForPoints { cmd, points } = &mut vm.state {
        points.push(pos);
        match cmd {
            CommandType::Line => {
                if points.len() == 2 {
                    let start = points[0];
                    let end = points[1];
                    vm.model.add_entity(Entity::Line(Line::new(start, end)));
                    *points = vec![end];
                    vm.status_message = "Specify next point:".to_string();
                } else {
                    vm.status_message = "Specify next point:".to_string();
                }
                vm.command_history
                    .push(format!("Point: {:.2}, {:.2}", pos.x, pos.y));
            }
            CommandType::Circle => {
                if points.len() == 2 {
                    let center = points[0];
                    let radius = center.dist(points[1]);
                    vm.model.add_entity(Entity::Circle(Circle::new(
                        center,
                        radius,
                        vm.filled_mode,
                    )));
                    vm.state = CommandState::Idle;
                    vm.status_message = "Command:".to_string();
                } else {
                    vm.status_message = "Specify radius point:".to_string();
                    vm.command_history
                        .push(format!("Center: {:.2}, {:.2}", pos.x, pos.y));
                }
            }
            CommandType::Rectangle => {
                if points.len() == 2 {
                    let p1 = points[0];
                    let p2 = points[1];
                    let min = Vector2::new(p1.x.min(p2.x), p1.y.min(p2.y));
                    let max = Vector2::new(p1.x.max(p2.x), p1.y.max(p2.y));
                    vm.model.add_entity(Entity::Rectangle(Rectangle::new(
                        min,
                        max,
                        vm.filled_mode,
                    )));
                    vm.state = CommandState::Idle;
                    vm.status_message = "Command:".to_string();
                } else {
                    vm.status_message = "Specify other corner:".to_string();
                    vm.command_history
                        .push(format!("Corner: {:.2}, {:.2}", pos.x, pos.y));
                }
            }
        }
    }
}

pub fn handle_point_token(
    vm: &mut CadViewModel,
    cmd: CommandType,
    points: &mut Vec<Vector2>,
    clean_token: &str,
) {
    if let Some(pos) = crate::viewmodel::parse_point(clean_token) {
        push_point(vm, pos);
    } else {
        match cmd {
            CommandType::Circle if points.len() == 1 => {
                if let Ok(radius) = clean_token.parse::<f32>() {
                    let center = points[0];
                    vm.model.add_entity(Entity::Circle(Circle::new(
                        center,
                        radius,
                        vm.filled_mode,
                    )));
                    vm.state = CommandState::Idle;
                    vm.status_message = "Command:".to_string();
                } else {
                    vm.status_message = format!("Invalid radius \"{}\".", clean_token);
                }
            }
            _ => {
                vm.status_message = format!("Invalid point \"{}\".", clean_token);
            }
        }
    }
}
