use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub layout: Layout,
    pub appearance: Appearance,
    pub keybindings: Keybindings,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Layout {
    pub mode: String,
    pub master_ratio: f32,
    pub outer_gap: u32,
    pub inner_gap: u32,
}

#[derive(Debug, Deserialize)]
pub struct Appearance {
    pub border_width: u32,
    pub focused_border: String,
    pub unfocused_border: String,
}

#[derive(Debug, Deserialize)]
pub struct Keybindings {
    pub terminal: String,
    pub launcher: String,
    pub close_window: String,
}

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;

    Ok(config)
}
