//! 外部数据加载器 — 从 JSON 文件加载地图/NPC 数据

use serde::{Deserialize, Serialize};
use std::io::Read;

/// JSON 地图格式
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct JsonMap {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub encounter_rate: u32,
    pub tiles: Vec<Vec<u8>>,
}

/// 加载地图数据 — 从文件系统
pub fn load_map(name: &str) -> Option<JsonMap> {
    let path = format!("data/maps/{}.json", name.to_lowercase());
    let mut file = std::fs::File::open(&path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    serde_json::from_str(&contents).ok()
}

/// NPC 数据格式
#[derive(Debug, Deserialize, Clone)]
pub struct JsonNpc {
    pub id: u32,
    pub name: String,
    pub dialogue_id: String,
    pub x: f32,
    pub y: f32,
    pub facing: String,
    pub patrol: Option<Vec<(f32, f32)>>,
}

/// 加载 NPC 列表
pub fn load_npcs(name: &str) -> Vec<JsonNpc> {
    let path = format!("data/npcs/{}.json", name.to_lowercase());
    match std::fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

/// 从 Reader 加载 JSON 地图
pub fn load_map_from_reader<R: Read>(reader: R) -> Option<JsonMap> {
    serde_json::from_reader(reader).ok()
}

/// 将地图数据导出为 JSON 字符串
pub fn export_map_to_json(name: &str, data: &[u8], width: i32, height: i32, encounter_rate: u32) -> String {
    let mut tiles = Vec::new();
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            let idx = (y * width + x) as usize;
            if idx < data.len() {
                row.push(data[idx]);
            }
        }
        tiles.push(row);
    }
    let map = JsonMap {
        name: name.to_string(),
        width,
        height,
        encounter_rate,
        tiles,
    };
    serde_json::to_string_pretty(&map).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_map_returns_none_for_missing() {
        let result = load_map("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn export_map_produces_valid_json() {
        let json = export_map_to_json("test", &[1, 2, 3, 4], 2, 2, 10);
        let parsed: Result<JsonMap, _> = serde_json::from_str(&json);
        assert!(parsed.is_ok());
        let map = parsed.unwrap();
        assert_eq!(map.width, 2);
        assert_eq!(map.height, 2);
        assert_eq!(map.tiles.len(), 2);
    }
}
