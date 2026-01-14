pub mod handlers;
pub mod state;

use crate::model::{CadModel, Vector2};
pub use state::{CommandState, CommandType};

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

        let tokens: Vec<&str> = input_text.split_whitespace().collect();
        for token in tokens {
            self.handle_token(token);
        }
    }

    fn handle_token(&mut self, token: &str) {
        let clean_token = token.trim().to_lowercase();

        // We need to work around borrowing self.state and calling methods on self
        // So we take the state out temporarily or just pass parts of self to handlers
        let current_state = self.state.clone();
        match current_state {
            CommandState::Idle => {
                handlers::handle_idle_token(self, &clean_token);
            }
            CommandState::WaitingForPoints { cmd, mut points } => {
                handlers::handle_point_token(self, cmd, &mut points, &clean_token);
            }
        }
    }

    pub fn handle_click(&mut self, pos: Vector2) {
        let current_state = self.state.clone();
        match current_state {
            CommandState::Idle => {
                self.selected_entity_idx = self.model.pick_entity(pos, 5.0);
                if let Some(idx) = self.selected_entity_idx {
                    let entity = &self.model.entities[idx];
                    self.status_message = format!("Selected: {}", entity.type_name());
                } else {
                    self.status_message = "Command:".to_string();
                }
            }
            CommandState::WaitingForPoints { .. } => {
                handlers::push_point(self, pos);
            }
        }
    }
}

pub fn parse_point(s: &str) -> Option<Vector2> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() == 2 {
        let x = parts[0].trim().parse::<f32>().ok()?;
        let y = parts[1].trim().parse::<f32>().ok()?;
        return Some(Vector2::new(x, y));
    }
    None
}
