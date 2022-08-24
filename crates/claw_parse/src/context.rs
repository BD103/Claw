use std::fmt;

#[derive(Debug, Clone)]
pub struct Context {
    pub scope: Scope,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            scope: Scope::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Stage,
    Sprite(String),
    StageFn,
    SpriteFn(String),
}

impl Default for Scope {
    fn default() -> Self {
        Self::Stage
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stage => write!(f, "Stage"),
            Self::Sprite(name) => write!(f, "Sprite '{}'", name),
            Self::StageFn => write!(f, "Stage function"),
            Self::SpriteFn(name) => write!(f, "Sprite function '{}'", name),
        }
    }
}
