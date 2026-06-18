//! UI 系统 — HUD、菜单、标题画面、状态、道具、场景过渡

pub mod font;

use macroquad::prelude::*;
use crate::engine::constants;
use crate::engine::TransitionKind;

/// 绘制游戏内 HUD
pub fn draw_hud(pp: u32, max_pp: u32, location: &str) {
    draw_text(format!("PP: {pp}/{max_pp}"), constants::HUD_X, constants::HUD_Y, constants::HUD_TEXT_SIZE, WHITE);
    draw_text(location, constants::HUD_X + 120.0, constants::HUD_Y, constants::HUD_TEXT_SIZE, WHITE);
}

/// 绘制主菜单背景/标题
pub fn draw_title_screen() {
    draw_text("Golden Sun - Rust Edition", 40.0, 160.0, 36.0, WHITE);
    draw_text("按 Z / Enter 开始", 120.0, 220.0, 20.0, constants::TITLE_TEXT_COLOR);
    draw_text("按 Space 菜单", 160.0, 260.0, 16.0, GRAY);
}

/// 绘制暂停菜单
pub fn draw_pause_menu(selection: usize, items: &[&str]) {
    draw_rectangle(100.0, 80.0, 440.0, 320.0, Color::from_rgba(0, 0, 0, 200));
    draw_rectangle(100.0, 80.0, 440.0, 320.0, Color::from_rgba(60, 60, 60, 100));
    draw_rectangle_lines(100.0, 80.0, 440.0, 320.0, 2.0, WHITE);

    for (i, item) in items.iter().enumerate() {
        let y = 120.0 + i as f32 * 40.0;
        let color = if i == selection { YELLOW } else { WHITE };
        draw_text(if i == selection { "▸ " } else { "  " }, 150.0, y, 24.0, color);
        draw_text(item, 182.0, y, 24.0, color);
    }
}

/// 绘制状态/道具界面
pub fn draw_status_screen(pp: u32, max_pp: u32, gold: u32, psynergy_names: &[&str]) {
    draw_rectangle(50.0, 50.0, 540.0, 380.0, Color::from_rgba(0, 0, 0, 220));
    draw_rectangle_lines(50.0, 50.0, 540.0, 380.0, 2.0, WHITE);

    draw_text("== STATUS ==", 80.0, 90.0, 22.0, YELLOW);
    draw_text(format!("PP: {pp}/{max_pp}"), 80.0, 130.0, 18.0, LIGHTGRAY);
    draw_text(format!("Gold: {gold}"), 80.0, 160.0, 18.0, LIGHTGRAY);

    draw_text("-- Psynergies --", 80.0, 200.0, 16.0, Color::from_rgba(0, 200, 255, 255));
    for (i, name) in psynergy_names.iter().enumerate() {
        draw_text(format!("  {}. {name}", i + 1), 100.0, 230.0 + i as f32 * 24.0, 16.0, WHITE);
    }

    draw_text("Press Cancel to close", 80.0, 390.0, 14.0, GRAY);
}

/// 绘制场景过渡效果
pub fn draw_transition(timer: f32, kind: TransitionKind) {
    match kind {
        TransitionKind::FadeOut => {
            let alpha = timer * 128.0;
            draw_rectangle(0.0, 0.0,
                constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT,
                Color::new(0.0, 0.0, 0.0, alpha.clamp(0.0, 128.0) / 255.0));
        }
        TransitionKind::FadeIn => {
            let alpha = (1.0 - timer) * 128.0;
            draw_rectangle(0.0, 0.0,
                constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT,
                Color::new(0.0, 0.0, 0.0, alpha.clamp(0.0, 128.0) / 255.0));
        }
        TransitionKind::Wipe => {
            let progress = timer;
            draw_rectangle(0.0, 0.0,
                constants::WINDOW_WIDTH * progress, constants::WINDOW_HEIGHT,
                BLACK);
            draw_text(format!("Transition: ({:.0}%)", timer * 100.0),
                20.0, constants::WINDOW_HEIGHT - 20.0, 16.0, WHITE);
        }
    }
}
