use nannou::prelude::*;
use crate::cfg_fetch;
use std::path::Path;

/// arthropods define enemy types
#[derive(Debug, Clone, Copy)]
pub enum Arthropod {
    Wasp,
}

/// Picks up the texture for each type

impl Arthropod {

    fn toTexture(&self, app: &App) -> wgpu::Texture {
        let texture_path = Path::new(self.assets()).join(Path::new(&cfg_fetch(&self.toConfig())));
        wgpu::Texture::from_path(app, texture_path).unwrap()
    }

    fn toConfig(&self) -> String {
        "sprites.".to_owned() + match self {
            Arthropod::Wasp => "wasp",
            _ => "none"
        }

    }

    fn assets(&self) -> &Path {
        let asset_pth = cfg_fetch("assets.sprites");
        &Path::new(&asset_pth)
    }
}
