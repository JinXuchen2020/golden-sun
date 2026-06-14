//! 输入事件总线 — 统一分发按键事件，防止多模块争抢同一输入
//!
//! ## 使用方式
//! 1. `InputBus::poll()` 每帧从 `InputState` 读取裸按键，转换为 `InputEvent`
//! 2. 各模块按优先级调用 `consume()` 消费事件
//! 3. 已消费的事件不会传递给后续消费者
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

use super::InputState;

/// 语义化的输入事件（解耦按键映射与游戏逻辑）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    /// 上（移动光标/选项）
    Up,
    /// 下
    Down,
    /// 左
    Left,
    /// 右
    Right,
    /// 确认（A/Z/Enter）
    Confirm,
    /// 取消/返回（B/X/Escape）
    Cancel,
    /// 菜单/暂停（Space/Start）
    Menu,
    /// 精灵力/辅助键（Shift/Select）
    Secondary,
}

/// 输入事件总线
pub struct InputBus {
    events: Vec<InputEvent>,
}

impl InputBus {
    pub fn new() -> Self {
        Self {
            events: Vec::with_capacity(8),
        }
    }

    /// 从 InputState 轮询，生成事件列表
    pub fn poll(&mut self, input: &InputState) {
        self.events.clear();

        // 方向
        if input.up    { self.events.push(InputEvent::Up); }
        if input.down  { self.events.push(InputEvent::Down); }
        if input.left  { self.events.push(InputEvent::Left); }
        if input.right { self.events.push(InputEvent::Right); }
        // 动作键
        if input.a       { self.events.push(InputEvent::Confirm); }
        if input.b       { self.events.push(InputEvent::Cancel); }
        if input.start   { self.events.push(InputEvent::Menu); }
        if input.select  { self.events.push(InputEvent::Secondary); }
    }

    /// 检查事件列表中是否存在某事件
    pub fn has(&self, event: InputEvent) -> bool {
        self.events.contains(&event)
    }

    /// 消费事件：检查并移除（返回 true 表示事件存在并被消费）
    pub fn consume(&mut self, event: InputEvent) -> bool {
        if let Some(pos) = self.events.iter().position(|&e| e == event) {
            self.events.remove(pos);
            true
        } else {
            false
        }
    }

    /// 获取所有剩余事件（调试用）
    pub fn remaining(&self) -> &[InputEvent] {
        &self.events
    }

    /// 本帧是否有任何事件
    pub fn has_any(&self) -> bool {
        !self.events.is_empty()
    }
}
