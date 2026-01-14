use crate::model::{CadModel, Entity, Vector2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandState {
    Idle,
    WaitingForLineStart,
    WaitingForLineEnd { start: Vector2 },
    WaitingForCircleCenter,
    WaitingForCircleRadius { center: Vector2 },
    WaitingForRectStart,
    WaitingForRectEnd { start: Vector2 },
}

pub struct CadViewModel {
    pub model: CadModel,
    pub command_input: String,
    pub command_history: Vec<String>,
    pub state: CommandState,
    pub status_message: String,
    pub filled_mode: bool,
    pub selected_entity_idx: Option<usize>,
}

impl CadViewModel {
    pub fn new() -> Self {
        Self {
            model: CadModel::new(),
            command_input: String::new(),
            command_history: Vec::new(),
            state: CommandState::Idle,
            status_message: "Command:".to_string(),
            filled_mode: false,
            selected_entity_idx: None,
        }
    }

    pub fn process_command(&mut self) {
        let input_text = self.command_input.clone();
        if input_text.trim().is_empty() {
            if self.state != CommandState::Idle {
                self.state = CommandState::Idle;
                self.status_message = "Command:".to_string();
            }
            self.command_input.clear();
            return;
        }

        self.command_history.push(format!("> {}", input_text));
        self.command_input.clear();

        // Split by whitespace but keep coordinates like "0,0" together
        let tokens: Vec<&str> = input_text.split_whitespace().collect();

        for token in tokens {
            self.handle_token(token);
        }
    }

    fn handle_token(&mut self, token: &str) {
        let clean_token = token.trim().to_lowercase();

        match self.state {
            CommandState::Idle => match clean_token.as_str() {
                "line" | "l" => {
                    self.state = CommandState::WaitingForLineStart;
                    self.status_message = "LINE Specify first point:".to_string();
                }
                "circle" | "c" => {
                    self.state = CommandState::WaitingForCircleCenter;
                    self.status_message = "CIRCLE Specify center point:".to_string();
                }
                "rect" | "r" => {
                    self.state = CommandState::WaitingForRectStart;
                    self.status_message = "RECTANGLE Specify first corner:".to_string();
                }
                "fill" | "shade" => {
                    self.filled_mode = !self.filled_mode;
                    let mode = if self.filled_mode { "ON" } else { "OFF" };
                    self.status_message = format!("SHADE mode: {}", mode);
                    self.command_history
                        .push(format!("Shade mode is now {}", mode));
                }
                "clear" => {
                    self.model.entities.clear();
                    self.command_history.clear();
                    self.selected_entity_idx = None;
                    self.status_message = "Command:".to_string();
                }
                _ => {
                    self.status_message = format!("Unknown command \"{}\".", clean_token);
                }
            },
            CommandState::WaitingForLineStart => {
                if let Some(pos) = parse_point(&clean_token) {
                    self.state = CommandState::WaitingForLineEnd { start: pos };
                    self.status_message = "Specify next point:".to_string();
                } else {
                    self.status_message = format!("Invalid point \"{}\". Try 'x,y'", clean_token);
                }
            }
            CommandState::WaitingForLineEnd { start } => {
                if let Some(end) = parse_point(&clean_token) {
                    self.model.add_entity(Entity::Line { start, end });
                    self.state = CommandState::WaitingForLineEnd { start: end };
                    self.status_message = "Specify next point:".to_string();
                } else {
                    self.status_message = format!("Invalid point \"{}\". Try 'x,y'", clean_token);
                }
            }
            CommandState::WaitingForCircleCenter => {
                if let Some(pos) = parse_point(&clean_token) {
                    self.state = CommandState::WaitingForCircleRadius { center: pos };
                    self.status_message = "Specify radius:".to_string();
                } else {
                    self.status_message = format!("Invalid center \"{}\".", clean_token);
                }
            }
            CommandState::WaitingForCircleRadius { center } => {
                if let Ok(radius) = clean_token.parse::<f32>() {
                    self.model.add_entity(Entity::Circle {
                        center,
                        radius,
                        filled: self.filled_mode,
                    });
                    self.state = CommandState::Idle;
                    self.status_message = "Command:".to_string();
                } else if let Some(pos) = parse_point(&clean_token) {
                    // Radius from distance to another point
                    let radius = center.dist(pos);
                    self.model.add_entity(Entity::Circle {
                        center,
                        radius,
                        filled: self.filled_mode,
                    });
                    self.state = CommandState::Idle;
                    self.status_message = "Command:".to_string();
                } else {
                    self.status_message = format!("Invalid radius \"{}\".", clean_token);
                }
            }
            CommandState::WaitingForRectStart => {
                if let Some(pos) = parse_point(&clean_token) {
                    self.state = CommandState::WaitingForRectEnd { start: pos };
                    self.status_message = "Specify other corner:".to_string();
                } else {
                    self.status_message = format!("Invalid point \"{}\".", clean_token);
                }
            }
            CommandState::WaitingForRectEnd { start } => {
                if let Some(pos) = parse_point(&clean_token) {
                    let min = Vector2::new(start.x.min(pos.x), start.y.min(pos.y));
                    let max = Vector2::new(start.x.max(pos.x), start.y.max(pos.y));
                    self.model.add_entity(Entity::Rectangle {
                        min,
                        max,
                        filled: self.filled_mode,
                    });
                    self.state = CommandState::Idle;
                    self.status_message = "Command:".to_string();
                } else {
                    self.status_message = format!("Invalid corner \"{}\".", clean_token);
                }
            }
        }
    }

    pub fn handle_click(&mut self, pos: Vector2) {
        match self.state {
            CommandState::Idle => {
                self.selected_entity_idx = self.model.pick_entity(pos, 5.0);
                if let Some(idx) = self.selected_entity_idx {
                    let entity = &self.model.entities[idx];
                    self.status_message = format!("Selected: {}", entity.type_name());
                } else {
                    self.status_message = "Command:".to_string();
                }
            }
            CommandState::WaitingForLineStart => {
                self.state = CommandState::WaitingForLineEnd { start: pos };
                self.status_message = "Specify next point:".to_string();
                self.command_history
                    .push(format!("Point: {:.2}, {:.2}", pos.x, pos.y));
            }
            CommandState::WaitingForLineEnd { start } => {
                self.model.add_entity(Entity::Line { start, end: pos });
                self.state = CommandState::WaitingForLineEnd { start: pos };
                self.status_message = "Specify next point:".to_string();
                self.command_history
                    .push(format!("Point: {:.2}, {:.2}", pos.x, pos.y));
            }
            CommandState::WaitingForCircleCenter => {
                self.state = CommandState::WaitingForCircleRadius { center: pos };
                self.status_message = "Specify radius:".to_string();
                self.command_history
                    .push(format!("Circle Center: {:.2}, {:.2}", pos.x, pos.y));
            }
            CommandState::WaitingForCircleRadius { center } => {
                let radius = center.dist(pos);
                self.model.add_entity(Entity::Circle {
                    center,
                    radius,
                    filled: self.filled_mode,
                });
                self.state = CommandState::Idle;
                self.status_message = "Command:".to_string();
            }
            CommandState::WaitingForRectStart => {
                self.state = CommandState::WaitingForRectEnd { start: pos };
                self.status_message = "Specify other corner:".to_string();
            }
            CommandState::WaitingForRectEnd { start } => {
                let min = Vector2::new(start.x.min(pos.x), start.y.min(pos.y));
                let max = Vector2::new(start.x.max(pos.x), start.y.max(pos.y));
                self.model.add_entity(Entity::Rectangle {
                    min,
                    max,
                    filled: self.filled_mode,
                });
                self.state = CommandState::Idle;
                self.status_message = "Command:".to_string();
            }
        }
    }
}

fn parse_point(s: &str) -> Option<Vector2> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() == 2 {
        let x = parts[0].trim().parse::<f32>().ok()?;
        let y = parts[1].trim().parse::<f32>().ok()?;
        return Some(Vector2::new(x, y));
    }
    None
}
