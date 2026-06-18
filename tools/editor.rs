//! 地图编辑器 — 20x20 网格 + 调色板 + 鼠标交互
//!
//! 运行方式: cargo run --bin editor (standalone)
//! 或通过 game/mod.rs 调试模式按 E 启动

use macroquad::prelude::*;
use golden_sun::map::TileKind;
use serde::Serialize;

const GRID_SIZE: i32 = 20;
const TILE_PIXELS: f32 = 24.0;
const PALETTE_X: f32 = 10.0;
const PALETTE_Y: f32 = 10.0;
const CANVAS_X: f32 = 200.0;
const CANVAS_Y: f32 = 10.0;

#[derive(Debug, Clone, Serialize)]
struct EditorMap {
    width: i32,
    height: i32,
    tiles: Vec<Vec<u8>>,
}

struct Editor {
    grid: Vec<Vec<TileKind>>,
    selected_tile: TileKind,
    hovered_cell: (i32, i32),
    exporting: bool,
    export_text: String,
}

impl Editor {
    fn new() -> Self {
        let grid = vec![vec![TileKind::Grass; GRID_SIZE as usize]; GRID_SIZE as usize];
        Self {
            grid,
            selected_tile: TileKind::Grass,
            hovered_cell: (0, 0),
            exporting: false,
            export_text: String::new(),
        }
    }

    fn tile_palette() -> Vec<TileKind> {
        vec![
            TileKind::Grass, TileKind::Dirt, TileKind::Water, TileKind::Forest,
            TileKind::Wall, TileKind::Sand, TileKind::Snow, TileKind::Bridge,
            TileKind::Stairs, TileKind::Flower, TileKind::Roof, TileKind::Vine,
            TileKind::PushBlock, TileKind::DarkArea, TileKind::HiddenChest,
            TileKind::Windmill, TileKind::Waypoint,
        ]
    }

    fn update(&mut self) {
        let mx = mouse_position().0;
        let my = mouse_position().1;

        // 检测是否在调色板区域点击
        let palette = Self::tile_palette();
        let cols = 6;
        let px = mx - PALETTE_X;
        let py = my - PALETTE_Y;
        if px >= 0.0 && px < (cols as f32) * 40.0 && py >= 0.0 && py < ((palette.len() as f32 + cols as f32 - 1.0) / cols as f32) * 40.0 {
            let col = (px / 40.0) as usize;
            let row = (py / 40.0) as usize;
            let idx = row * cols + col;
            if idx < palette.len() {
                self.selected_tile = palette[idx];
            }
        }

        // 检测是否在画布区域
        let cx = mx - CANVAS_X;
        let cy = my - CANVAS_Y;
        if cx >= 0.0 && cx < GRID_SIZE as f32 * TILE_PIXELS && cy >= 0.0 && cy < GRID_SIZE as f32 * TILE_PIXELS {
            self.hovered_cell = (
                (cx / TILE_PIXELS) as i32,
                (cy / TILE_PIXELS) as i32,
            );
        }

        // 左键放置 tile
        if is_mouse_button_down(MouseButton::Left) {
            let (gx, gy) = self.hovered_cell;
            if (0..GRID_SIZE).contains(&gx) && (0..GRID_SIZE).contains(&gy) {
                self.grid[gy as usize][gx as usize] = self.selected_tile;
            }
        }

        // 右键移除（设为 Void）
        if is_mouse_button_down(MouseButton::Right) {
            let (gx, gy) = self.hovered_cell;
            if (0..GRID_SIZE).contains(&gx) && (0..GRID_SIZE).contains(&gy) {
                self.grid[gy as usize][gx as usize] = TileKind::Void;
            }
        }

        // Q 键导出 JSON
        if is_key_pressed(KeyCode::Q) {
            self.export_map();
        }

        // Esc 关闭编辑器
        if is_key_pressed(KeyCode::Escape) {
            self.exporting = false;
        }
    }

    fn export_map(&mut self) {
        let tiles: Vec<Vec<u8>> = self.grid.iter()
            .map(|row| row.iter().map(|t| tile_to_u8(*t)).collect())
            .collect();
        let em = EditorMap { width: GRID_SIZE, height: GRID_SIZE, tiles };
        self.export_text = serde_json::to_string_pretty(&em).unwrap_or_default();
        self.exporting = true;
    }

    fn draw(&self) {
        clear_background(BLACK);

        // 调色板
        let palette = Self::tile_palette();
        let cols = 6;
        draw_text("Palette", PALETTE_X, PALETTE_Y - 10.0, 16.0, WHITE);
        for (i, &tile) in palette.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let x = PALETTE_X + col as f32 * 40.0;
            let y = PALETTE_Y + 10.0 + row as f32 * 40.0;
            let color = tile.color();
            draw_rectangle(x, y, 36.0, 36.0, color);
            draw_rectangle_lines(x, y, 36.0, 36.0, 1.0, if tile == self.selected_tile { YELLOW } else { DARKGRAY });
            draw_text(format!("{}", i + 1), x + 4.0, y + 24.0, 10.0, WHITE);
        }

        // 画布
        draw_text(format!("Canvas ({GRID_SIZE}x{GRID_SIZE})"), CANVAS_X, CANVAS_Y - 10.0, 16.0, WHITE);

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let tile = self.grid[y as usize][x as usize];
                let color = tile.color();
                let px = CANVAS_X + x as f32 * TILE_PIXELS;
                let py = CANVAS_Y + y as f32 * TILE_PIXELS;
                draw_rectangle(px, py, TILE_PIXELS, TILE_PIXELS, color);
                draw_rectangle_lines(px, py, TILE_PIXELS, TILE_PIXELS, 0.5, DARKGRAY);
            }
        }

        // 悬停高亮
        let (hx, hy) = self.hovered_cell;
        if (0..GRID_SIZE).contains(&hx) && (0..GRID_SIZE).contains(&hy) {
            let px = CANVAS_X + hx as f32 * TILE_PIXELS;
            let py = CANVAS_Y + hy as f32 * TILE_PIXELS;
            draw_rectangle_lines(px, py, TILE_PIXELS, TILE_PIXELS, 2.0, YELLOW);
        }

        // 选中 tile 预览
        let preview_color = self.selected_tile.color();
        draw_rectangle(PALETTE_X, PALETTE_Y + 300.0, 36.0, 36.0, preview_color);
        draw_text("Selected tile", PALETTE_X, PALETTE_Y + 345.0, 12.0, WHITE);

        // 操作提示
        draw_text("Left: Place | Right: Clear | Q: Export JSON | Esc: Close", 10.0, 460.0, 14.0, GRAY);

        // 导出面板
        if self.exporting {
            draw_rectangle(50.0, 50.0, 540.0, 380.0, Color::from_rgba(0, 0, 0, 220));
            draw_rectangle_lines(50.0, 50.0, 540.0, 380.0, 2.0, WHITE);
            draw_text("== Map Export (JSON) ==", 70.0, 70.0, 16.0, YELLOW);
            let lines: Vec<&str> = self.export_text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                if i < 20 {
                    draw_text(line, 70.0, 95.0 + i as f32 * 16.0, 12.0, WHITE);
                }
            }
            draw_text("Press Esc to close", 70.0, 400.0, 12.0, GRAY);
        }
    }
}

fn tile_to_u8(t: TileKind) -> u8 {
    match t {
        TileKind::Void => 0,
        TileKind::Grass => 1,
        TileKind::Dirt => 2,
        TileKind::Water => 3,
        TileKind::Forest => 4,
        TileKind::Wall => 5,
        TileKind::Sand => 6,
        TileKind::Snow => 7,
        TileKind::Bridge => 8,
        TileKind::Stairs => 9,
        TileKind::Flower => 10,
        TileKind::Roof => 11,
        TileKind::Vine => 12,
        TileKind::Seed => 13,
        TileKind::Ice => 14,
        TileKind::PushBlock => 15,
        TileKind::Windmill => 16,
        TileKind::WindmillActive => 17,
        TileKind::DarkArea => 18,
        TileKind::HiddenChest => 19,
        TileKind::OpenedChest => 20,
        TileKind::VineClimbable => 21,
        TileKind::Waypoint => 255,
        TileKind::Unknown => 254,
        _ => 0,
    }
}

#[macroquad::main("Golden Sun Map Editor")]
async fn main() {
    let mut editor = Editor::new();
    loop {
        editor.update();
        editor.draw();
        next_frame().await;
    }
}
