use crate::{ArgType, BlockType, Module};

/// Exposes blocks for manipulating the look of sprites and backdrops.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Looks {
    SayFor,
    Say,
    ThinkFor,
    Think,

    SwitchCostume,
    NextCostume,
    SwitchBackdrop,
    NextBackdrop,
    // SwitchBackdropAndWait,
    ChangeSize,
    SetSize,

    ChangeEffect,
    SetEffect,
    ClearEffects,

    Show,
    Hide,

    GoToLayer,
    ChangeLayers,

    GetCostumeNumber,
    GetCostumeName,
    GetBackdropNumber,
    GetBackdropName,
    GetSize,
}

impl Module for Looks {
    fn get_from<'a>(name: &'a str) -> Option<Self> {
        Some(match name {
            "say_for" => Self::SayFor,
            "say" => Self::Say,
            "think_for" => Self::ThinkFor,
            "think" => Self::Think,

            "switch_costume" => Self::SwitchCostume,
            "next_costume" => Self::NextCostume,
            "switch_backdrop" => Self::SwitchBackdrop,
            "next_backdrop" => Self::NextBackdrop,

            "change_size" => Self::ChangeSize,
            "set_size" => Self::SetSize,

            "change_effect" => Self::ChangeEffect,
            "set_effect" => Self::SetEffect,
            "clear_effects" => Self::ClearEffects,

            "show" => Self::Show,
            "hide" => Self::Hide,

            "goto_layer" => Self::GoToLayer,
            "change_layers" => Self::ChangeLayers,

            "get_costume_number" => Self::GetCostumeNumber,
            "get_costume_name" => Self::GetCostumeName,
            "get_backdrop_number" => Self::GetBackdropNumber,
            "get_backdrop_name" => Self::GetBackdropName,
            "get_size" => Self::GetSize,

            _ => return None,
        })
    }

    fn get_type(&self) -> BlockType {
        match self {
            Self::GetCostumeNumber | Self::GetBackdropNumber | Self::GetSize => BlockType::Number,
            Self::GetCostumeName | Self::GetBackdropName => BlockType::String,
            _ => BlockType::Block,
        }
    }

    fn get_args(&self) -> &[ArgType] {
        use ArgType::*;

        match self {
            Self::SayFor | Self::ThinkFor => &[String, Number],
            Self::Say | Self::Think => &[String],

            Self::SwitchCostume | Self::SwitchBackdrop => &[Costume],

            Self::ChangeSize | Self::SetSize => &[Number],

            Self::ChangeEffect | Self::SetEffect => &[Effect, Number],

            Self::GoToLayer => &[Unsupported],
            Self::ChangeLayers => &[Unsupported, Number],

            _ => &[],
        }
    }
}
