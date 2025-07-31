
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoMap {
    pub canvas: NewCanvas,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCanvas {
    pub width: usize,
    pub height: usize,
}

pub fn get_info_map(file_name: &String) -> Result<InfoMap, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(file_name)?;
    let root: InfoMap = serde_json::from_str(&json_str)?;
    Ok(root)
}
