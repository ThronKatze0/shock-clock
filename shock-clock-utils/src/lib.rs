use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub id: usize,
    pub name: String,
    shock_strength: ShockStrength,
    block_type: BlockType,
}

#[derive(Serialize, Deserialize, Clone)]
enum BlockType {
    App(AppBlockData),
    Website(WebsiteBlockData),
    Keyword(KeywordBlockData),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ShockStrength {
    Normal,
    Hard,
    Ultra,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppBlockData {
    pub package_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebsiteBlockData {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeywordBlockData {
    pub name: String,
}

pub mod ble;
