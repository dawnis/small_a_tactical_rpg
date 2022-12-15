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

    pub fn to_texture(&self, app: &App) -> wgpu::Texture {
        let asset_pth = cfg_fetch("assets.sprites");
        let texture_path = Path::new(&asset_pth).join(Path::new(&cfg_fetch(&self.to_config())));
        wgpu::Texture::from_path(app, texture_path).unwrap()
    }

    fn to_config(&self) -> String {
        "sprites.".to_owned() + match self {
            Arthropod::Wasp => "wasp",
            _ => "none"
        }

    }

}
