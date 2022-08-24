use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Target {
    pub variables: Vec<String>,
    pub lists: Vec<String>,
    pub events: Vec<String>,
    pub functions: Vec<()>,
}

impl Default for Target {
    fn default() -> Self {
        Target {
            variables: Vec::new(),
            lists: Vec::new(),
            events: Vec::new(),
            functions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,

    target: Target,
}

impl Sprite {
    pub fn new(name: String) -> Self {
        Sprite {
            name,
            target: Target::default(),
        }
    }
}

impl Deref for Sprite {
    type Target = Target;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl DerefMut for Sprite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target
    }
}

#[derive(Debug)]
pub struct Stage {
    pub sprites: Vec<Sprite>,

    target: Target,
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            sprites: Vec::new(),
            target: Target::default(),
        }
    }
}

impl Deref for Stage {
    type Target = Target;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl DerefMut for Stage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target
    }
}
