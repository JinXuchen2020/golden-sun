//! 输入系统 — InputState（裸按键）+ InputBus（事件总线）
//!
//! ## 架构
//! - `InputState`: 每帧从 macroquad 读取裸按键状态
//! - `InputBus`: 将 InputState 转换为语义化 InputEvent，供各模块消费
//!
//! ## 优先级约定
//! | 状态 | 消费模块 |
//! |------|----------|
//! | Transition | (全部锁定) |
//! | Dialog | dialogue.rs 消费 Confirm/Cancel |
//! | Psynergy | psynergy/mod.rs 消费方向/Confirm/Cancel |
//! | Battle | battle/state.rs 消费方向/Confirm/Cancel |
//! | Menu | menu.rs 消费方向/Confirm/Cancel |
//! | WorldMap | player.rs 消费方向/A; input.rs 消费 精灵力B/菜单Start |
//! | Title | main.rs 消费 Confirm |

use macroquad::prelude::*;

/// 当前帧的输入状态（裸按键读值，供 InputBus 消费）
#[derive(Debug, Clone, Copy)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl InputState {
    pub const fn new() -> Self {
        Self {
            up: false, down: false, left: false, right: false,
            a: false, b: false, start: false, select: false,
        }
    }

    pub fn poll(&mut self) {
        self.up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        self.down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
        self.left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        self.right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        self.a = is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Enter);
        self.b = is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Escape);
        self.start = is_key_pressed(KeyCode::Space);
        self.select = is_key_pressed(KeyCode::LeftShift);
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// 语义化的输入事件（解耦按键映射与游戏逻辑）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Cancel,
    Menu,
    Secondary,
}

/// 输入事件总线
#[derive(Debug)]
pub struct InputBus {
    events: Vec<InputEvent>,
}

impl Default for InputBus {
    fn default() -> Self {
        Self::new()
    }
}

impl InputBus {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(8),
        }
    }

    pub fn poll(&mut self, input: &InputState) {
        self.events.clear();
        if input.up    { self.events.push(InputEvent::Up); }
        if input.down  { self.events.push(InputEvent::Down); }
        if input.left  { self.events.push(InputEvent::Left); }
        if input.right { self.events.push(InputEvent::Right); }
        if input.a       { self.events.push(InputEvent::Confirm); }
        if input.b       { self.events.push(InputEvent::Cancel); }
        if input.start   { self.events.push(InputEvent::Menu); }
        if input.select  { self.events.push(InputEvent::Secondary); }
    }

    #[must_use]
    pub fn has(&self, event: InputEvent) -> bool {
        self.events.contains(&event)
    }

    pub fn consume(&mut self, event: InputEvent) -> bool {
        if let Some(pos) = self.events.iter().position(|&e| e == event) {
            self.events.remove(pos);
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn remaining(&self) -> &[InputEvent] {
        &self.events
    }

    #[must_use]
    pub fn has_any(&self) -> bool {
        !self.events.is_empty()
    }
}
