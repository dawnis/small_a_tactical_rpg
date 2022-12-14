use nannou::prelude::*;
use core::cfg_fetch;

/// arthropods define enemy types
#[derive(Debug, Clone, Copy)]
pub enum Arthropod {
    Wasp,
}

/// Picks up the texture for each type

impl Arthropod {
    fn toTexture(&self) -> wgpu::Texture {
        cfg_fetch();
    }

    fn toConfig(&self) -> String {
        "sprites.".to_owned() + match self {
            Arthropod::Wasp => "wasp",
            _ => "none"
        }

    }
}
