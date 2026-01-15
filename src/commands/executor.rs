use crate::commands::arc::ArcCommand;
use crate::commands::axis::AxisCommand;
use crate::commands::circle::CircleCommand;
use crate::commands::copy::CopyCommand;
use crate::commands::distance::DistanceCommand;
use crate::commands::line::LineCommand;
use crate::commands::r#move::MoveCommand;
use crate::commands::offset::OffsetCommand;
use crate::commands::rectangle::RectangleCommand;
use crate::commands::rotate::RotateCommand;
use crate::commands::scale::ScaleCommand;
use crate::commands::text::TextCommand;
use crate::commands::trim::TrimCommand;
use crate::commands::{Command, CommandContext, InputModifiers, InputResult, PointResult};
use crate::model::{CadModel, Vector2};
use std::collections::{HashMap, HashSet};

/// Factory function type for creating commands
type CommandFactory = fn() -> Box<dyn Command>;

/// Registry of available commands with their aliases
pub struct CommandRegistry {
    commands: HashMap<&'static str, CommandFactory>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
        };

        // Register drawing commands
        registry.register("line", || Box::new(LineCommand::new()));
        registry.register("l", || Box::new(LineCommand::new()));

        registry.register("circle", || Box::new(CircleCommand::new()));
        registry.register("c", || Box::new(CircleCommand::new()));

        registry.register("rect", || Box::new(RectangleCommand::new()));
        registry.register("rectangle", || Box::new(RectangleCommand::new()));

        registry.register("arc", || Box::new(ArcCommand::new()));

        // Register manipulation commands
        registry.register("move", || Box::new(MoveCommand::new()));
        registry.register("w", || Box::new(MoveCommand::new()));

        registry.register("rotate", || Box::new(RotateCommand::new()));
        registry.register("e", || Box::new(RotateCommand::new()));

        registry.register("scale", || Box::new(ScaleCommand::new()));
        registry.register("r", || Box::new(ScaleCommand::new()));

        // Register copy/cut commands
        registry.register("copy", || Box::new(CopyCommand::new()));
        registry.register("co", || Box::new(CopyCommand::new()));
        registry.register("cut", || Box::new(CopyCommand::new_cut()));
        registry.register("x", || Box::new(CopyCommand::new_cut()));

        // Register construction commands
        registry.register("axis", || Box::new(AxisCommand::new()));
        registry.register("aks", || Box::new(AxisCommand::new()));
        registry.register("a", || Box::new(AxisCommand::new()));

        // Register edit commands
        registry.register("trim", || Box::new(TrimCommand::new()));
        registry.register("t", || Box::new(TrimCommand::new()));

        registry.register("offset", || Box::new(OffsetCommand::new()));
        registry.register("o", || Box::new(OffsetCommand::new()));

        // Register annotation commands
        registry.register("text", || Box::new(TextCommand::new()));
        registry.register("distance", || Box::new(DistanceCommand::new()));
        registry.register("dist", || Box::new(DistanceCommand::new()));

        registry.register("measure", || {
            Box::new(crate::commands::create::measure::MeasureCommand::new())
        });
        registry.register("dim", || {
            Box::new(crate::commands::create::measure::MeasureCommand::new())
        });

        registry
    }

    pub fn register(&mut self, name: &'static str, factory: CommandFactory) {
        self.commands.insert(name, factory);
    }

    pub fn create(&self, name: &str) -> Option<Box<dyn Command>> {
        self.commands.get(name).map(|factory| factory())
    }
}

/// Manages the active command and coordinates with the model
pub struct CommandExecutor {
    registry: CommandRegistry,
    active_command: Option<Box<dyn Command>>,
    pub status_message: String,
    pub filled_mode: bool,
    pub modifiers: InputModifiers,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::new(),
            active_command: None,
            status_message: "Command:".to_string(),
            filled_mode: false,
            modifiers: InputModifiers::default(),
        }
    }

    /// Update keyboard modifiers (called from view)
    pub fn set_modifiers(&mut self, modifiers: InputModifiers) {
        self.modifiers = modifiers;
    }

    /// Try to start a new command by name
    pub fn start_command(
        &mut self,
        name: &str,
        model: &mut CadModel,
        selected_indices: &HashSet<usize>,
    ) -> bool {
        if let Some(mut cmd) = self.registry.create(name) {
            // Check if command can execute in current context
            let ctx = CommandContext {
                model,
                selected_indices,
                filled_mode: self.filled_mode,
                modifiers: self.modifiers,
            };

            if !cmd.can_execute(&ctx) {
                self.status_message = cmd.cannot_execute_message();
                return false;
            }

            // Call on_start for commands that need initial setup
            cmd.on_start(&ctx);

            self.status_message = cmd.initial_prompt();
            self.active_command = Some(cmd);
            true
        } else {
            false
        }
    }

    /// Cancel the current command
    pub fn cancel(&mut self) {
        self.active_command = None;
        self.status_message = "Command:".to_string();
    }

    /// Check if a command is active
    pub fn is_active(&self) -> bool {
        self.active_command.is_some()
    }

    /// Process a click/point input
    pub fn push_point(
        &mut self,
        pos: Vector2,
        model: &mut CadModel,
        selected_indices: &HashSet<usize>,
    ) {
        if let Some(cmd) = &mut self.active_command {
            let mut ctx = CommandContext {
                model,
                selected_indices,
                filled_mode: self.filled_mode,
                modifiers: self.modifiers,
            };

            // Apply constraints based on modifiers
            let constrained_pos =
                cmd.constrain_point(pos, cmd.get_points().last().copied(), self.modifiers);

            match cmd.push_point(constrained_pos, &mut ctx) {
                PointResult::NeedMore { prompt } => {
                    self.status_message = prompt;
                }
                PointResult::Complete => {
                    self.cancel();
                }
            }
        }
    }

    /// Process text input
    pub fn process_input(
        &mut self,
        input: &str,
        model: &mut CadModel,
        selected_indices: &HashSet<usize>,
    ) {
        // First, check if it's a new command
        let clean = input.trim().to_lowercase();
        if self.start_command(&clean, model, selected_indices) {
            return;
        }

        // If no active command, show error
        if self.active_command.is_none() {
            self.status_message = format!("Unknown command \"{}\".", clean);
            return;
        }

        // Process with active command
        if let Some(cmd) = &mut self.active_command {
            let mut ctx = CommandContext {
                model,
                selected_indices,
                filled_mode: self.filled_mode,
                modifiers: self.modifiers,
            };

            match cmd.process_input(&clean, &mut ctx) {
                InputResult::Point(PointResult::Complete)
                | InputResult::Parameter(PointResult::Complete) => {
                    self.cancel();
                }
                InputResult::Point(PointResult::NeedMore { prompt })
                | InputResult::Parameter(PointResult::NeedMore { prompt }) => {
                    self.status_message = prompt;
                }
                InputResult::Invalid { message } => {
                    self.status_message = message;
                }
            }
        }
    }

    /// Get points from active command for preview
    pub fn get_preview_points(&self) -> Option<(&dyn Command, &[Vector2])> {
        self.active_command
            .as_ref()
            .map(|cmd| (cmd.as_ref(), cmd.get_points()))
    }

    /// Toggle filled mode
    pub fn toggle_filled(&mut self) -> bool {
        self.filled_mode = !self.filled_mode;
        self.filled_mode
    }

    /// Toggle arc direction (CW/CCW) if arc command is active
    pub fn toggle_arc_direction(&mut self) -> bool {
        if let Some(cmd) = &mut self.active_command {
            if cmd.name() == "Arc" {
                if let Some(any) = cmd.as_any_mut() {
                    if let Some(arc_cmd) = any.downcast_mut::<crate::commands::arc::ArcCommand>() {
                        arc_cmd.toggle_direction();
                        let dir = if arc_cmd.clockwise { "CW" } else { "CCW" };
                        self.status_message =
                            format!("Specify end point [{}] (R to reverse):", dir);
                        return true;
                    }
                }
            }
        }
        false
    }
}
