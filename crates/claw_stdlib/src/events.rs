use crate::{EventModule, Module};

/// Lol funny name.
#[derive(Clone, Copy, Debug)]
pub enum EventEvents {
    FlagClicked,
    SpriteClicked,
}

impl Module for EventEvents {
    fn from_ident(ident: &str) -> Option<Self> {
        Some(match ident {
            "flag_clicked" => Self::FlagClicked,
            "sprite_clicked" => Self::SpriteClicked,
            _ => return None,
        })
    }

    fn opcode(&self) -> &'static str {
        match self {
            Self::FlagClicked => "event_whenflagclicked",
            Self::SpriteClicked => "event_whenthisspriteclicked",
        }
    }
}

impl EventModule for EventEvents {}
