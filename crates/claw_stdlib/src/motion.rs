use crate::{ArgType, BlockType, Module};

/// Exposes blocks for manipulating the position of sprites.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Motion {
    MoveSteps,
    TurnLeft,
    TurnRight,

    Goto,
    GotoXY,
    GlideTo,
    GlideToXY,

    PointInDirection,
    PointTowards,

    ChangeX,
    SetX,
    ChangeY,
    SetY,

    IfOnEdgeBounce,

    SetRotationStyle,

    GetX,
    GetY,
    GetDirection,
}

impl Module for Motion {
    fn get_from(name: &str) -> Option<Self> {
        Some(match name {
            "move_steps" => Self::MoveSteps,
            "turn_left" => Self::TurnLeft,
            "turn_right" => Self::TurnRight,

            "goto" => Self::Goto,
            "goto_xy" => Self::GotoXY,
            "glide_to" => Self::GlideTo,
            "glide_to_xy" => Self::GlideToXY,

            "point_in_direction" => Self::PointInDirection,
            "point_towards" => Self::PointTowards,

            "change_x" => Self::ChangeX,
            "set_x" => Self::SetX,
            "change_y" => Self::ChangeY,
            "set_y" => Self::SetY,

            "if_on_edge_bounce" => Self::IfOnEdgeBounce,

            "set_rotation_style" => Self::SetRotationStyle,

            "get_x" => Self::GetX,
            "get_y" => Self::GetY,
            "get_direction" => Self::GetDirection,

            _ => return None,
        })
    }

    fn get_type(&self) -> BlockType {
        match self {
            Self::GetX | Self::GetY | Self::GetDirection => BlockType::Number,
            _ => BlockType::Block,
        }
    }

    fn get_args(&self) -> &[ArgType] {
        use ArgType::*;

        match self {
            Self::MoveSteps | Self::TurnLeft | Self::TurnRight => &[Number],

            Self::Goto => &[Sprite],
            Self::GotoXY => &[Number, Number],
            Self::GlideTo => &[Number, Sprite],
            Self::GlideToXY => &[Number, Number, Number],

            Self::PointInDirection => &[Number],
            Self::PointTowards => &[Sprite],

            Self::ChangeX | Self::SetX | Self::ChangeY | Self::SetY => &[Number],

            Self::SetRotationStyle => &[Unsupported],

            _ => &[],
        }
    }
}
