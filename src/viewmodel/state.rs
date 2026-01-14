use crate::model::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandType {
    Line,
    Circle,
    Rectangle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandState {
    Idle,
    WaitingForPoints {
        cmd: CommandType,
        points: Vec<Vector2>,
    },
}
