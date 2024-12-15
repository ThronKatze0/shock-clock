use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Block {
    App(AppBlockData),
    Website(WebsiteBlockData),
    Keyword(KeywordBlockData),
}

#[derive(Serialize, Deserialize)]
pub enum ShockStrength {
    Normal,
    Hard,
    Ultra,
}

#[derive(Serialize, Deserialize)]
pub struct AppBlockData {
    app_name: String,
    package_name: String,
    shock_strength: ShockStrength,
}

#[derive(Serialize, Deserialize)]
pub struct WebsiteBlockData {
    url: String,
    shock_strength: ShockStrength,
}

#[derive(Serialize, Deserialize)]
pub struct KeywordBlockData {
    name: String,
    shock_strength: ShockStrength,
}
