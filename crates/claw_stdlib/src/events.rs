use crate::{ArgType, BlockType, Module};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Events {
    OnStart,
    OnKeyPress,
    OnSpriteClick,
    OnBackdropSwitch,
    
    OnGreaterThan,

    OnEvent,
    Broadcast,
    BroadcastAndWait,
}

impl Module for Events {
    fn get_from<'a>(name: &'a str) -> Option<Self> {
        Some(match name {
            "on_start" => Self::OnStart,
            "on_key_press" => Self::OnKeyPress,
            "on_sprite_click" => Self::OnSpriteClick,
            "on_backdrop_switch" => Self::OnBackdropSwitch,

            "on_greater_than" => Self::OnGreaterThan,

            "on_event" => Self::OnEvent,
            "broadcast" => Self::Broadcast,
            "broadcast_and_wait" => Self::BroadcastAndWait,

            _ => return None,
        })
    }

    fn get_type(&self) -> BlockType {
        match self {
            Self::OnStart | Self::OnKeyPress | Self::OnSpriteClick | Self::OnBackdropSwitch => BlockType::Hat,

            Self::OnGreaterThan => BlockType::Hat,

            Self::OnEvent => BlockType::Hat,

            _ => BlockType::Block,
        }
    }

    fn get_args(&self) -> &[ArgType] {
        use ArgType::*;

        match self {
            Self::OnKeyPress => &[KeyCode],
            Self::OnBackdropSwitch => &[Backdrop],

            Self::OnGreaterThan => &[Unsupported, Number],
            Self::Broadcast => &[Event],
            Self::BroadcastAndWait => &[Event],

            _ => &[],
        }
    }
}
