pub mod arc;
pub mod axis;
pub mod circle;
pub mod copy;
pub mod distance;
pub mod executor;
pub mod line;
pub mod r#move;
pub mod rectangle;
pub mod rotate;
pub mod scale;
pub mod text;
pub mod trim;

use crate::model::{CadModel, Vector2};
use std::collections::HashSet;

/// Keyboard modifiers for input constraints
#[derive(Debug, Clone, Copy, Default)]
pub struct InputModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

/// Context passed to commands for execution
pub struct CommandContext<'a> {
    pub model: &'a mut CadModel,
    pub selected_indices: &'a HashSet<usize>,
    pub filled_mode: bool,
    pub modifiers: InputModifiers,
}

/// Category of command - determines behavior and requirements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandCategory {
    /// Creates new entities (Line, Circle, Rectangle)
    Creation,
    /// Manipulates existing entities (Move, Rotate, Scale)
    Manipulation,
    /// Utility commands (Clear, Undo, etc.)
    Utility,
}

/// Result of processing a point in a command
pub enum PointResult {
    /// Command needs more points
    NeedMore { prompt: String },
    /// Command is complete
    Complete,
}

/// Result of processing text input
pub enum InputResult {
    /// Input was handled as a point
    Point(PointResult),
    /// Input was handled as a parameter (e.g., radius)
    Parameter(PointResult),
    /// Input was not valid for this command
    Invalid { message: String },
}

/// Trait that all CAD commands must implement
pub trait Command: std::fmt::Debug {
    /// Returns the command name for display
    fn name(&self) -> &'static str;

    /// Returns the command category
    fn category(&self) -> CommandCategory {
        CommandCategory::Creation
    }

    /// Check if command can be executed in current context
    fn can_execute(&self, ctx: &CommandContext) -> bool {
        match self.category() {
            CommandCategory::Creation => true,
            CommandCategory::Manipulation => !ctx.selected_indices.is_empty(),
            CommandCategory::Utility => true,
        }
    }

    /// Returns the initial prompt when command starts
    fn initial_prompt(&self) -> String;

    /// Returns error message when can_execute fails
    fn cannot_execute_message(&self) -> String {
        match self.category() {
            CommandCategory::Manipulation => "No entities selected.".to_string(),
            _ => "Cannot execute command.".to_string(),
        }
    }

    /// Process a point input with modifiers (from click or parsed coordinates)
    fn push_point(&mut self, pos: Vector2, ctx: &mut CommandContext) -> PointResult;

    /// Apply input constraints based on modifiers (e.g., Shift for ortho)
    fn constrain_point(
        &self,
        pos: Vector2,
        last_point: Option<Vector2>,
        modifiers: InputModifiers,
    ) -> Vector2 {
        // Default: no constraints
        // Override in specific commands for custom behavior
        if modifiers.shift {
            if let Some(last) = last_point {
                // Ortho constraint: snap to horizontal or vertical
                let dx = (pos.x - last.x).abs();
                let dy = (pos.y - last.y).abs();
                if dx > dy {
                    return Vector2::new(pos.x, last.y);
                } else {
                    return Vector2::new(last.x, pos.y);
                }
            }
        }
        pos
    }

    /// Process text input that isn't a point (e.g., radius for circle)
    fn process_input(&mut self, input: &str, ctx: &mut CommandContext) -> InputResult {
        // Default: try to parse as point
        if let Some(pos) = parse_point(input) {
            let constrained =
                self.constrain_point(pos, self.get_points().last().copied(), ctx.modifiers);
            InputResult::Point(self.push_point(constrained, ctx))
        } else {
            InputResult::Invalid {
                message: format!("Invalid input \"{}\".", input),
            }
        }
    }

    /// Get current points for preview drawing
    fn get_points(&self) -> &[Vector2];

    /// Called when command starts (for manipulation commands to capture initial state)
    fn on_start(&mut self, _ctx: &CommandContext) {}

    /// Clone the command (for state management)
    fn clone_box(&self) -> Box<dyn Command>;
}

/// Parse a point from string like "10,20" or "10.5, -20.3"
pub fn parse_point(s: &str) -> Option<Vector2> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() == 2 {
        let x = parts[0].trim().parse::<f32>().ok()?;
        let y = parts[1].trim().parse::<f32>().ok()?;
        return Some(Vector2::new(x, y));
    }
    None
}
