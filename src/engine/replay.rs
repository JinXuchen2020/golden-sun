//! 回放系统 — 输入环形缓冲区录制/回放
//!
//! 按 R 开始录制，按 P 回放录制的内容。

use crate::InputState;
use std::cell::RefCell;

const RING_BUFFER_SIZE: usize = 600; // 约10秒 @ 60fps

/// 单帧输入快照
#[derive(Debug, Clone, Copy)]
pub struct InputFrame {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
}

impl From<&InputState> for InputFrame {
    fn from(input: &InputState) -> Self {
        Self {
            up: input.up,
            down: input.down,
            left: input.left,
            right: input.right,
            a: input.a,
            b: input.b,
            start: input.start,
        }
    }
}

/// 回放管理器
pub struct ReplayManager {
    buffer: Vec<InputFrame>,
    recording: bool,
    playing: bool,
    play_index: usize,
}

impl Default for ReplayManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplayManager {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(RING_BUFFER_SIZE),
            recording: false,
            playing: false,
            play_index: 0,
        }
    }

    /// 记录当前帧输入
    pub fn record(&mut self, input: &InputState) {
        if self.recording {
            let frame = InputFrame::from(input);
            if self.buffer.len() < RING_BUFFER_SIZE {
                self.buffer.push(frame);
            } else {
                let idx = self.buffer.len() % RING_BUFFER_SIZE;
                self.buffer[idx] = frame;
            }
        }
    }

    /// 开始录制
    pub fn start_recording(&mut self) {
        self.buffer.clear();
        self.recording = true;
        self.playing = false;
        #[cfg(debug_assertions)]
        eprintln!("开始录制回放");
    }

    /// 停止录制
    pub fn stop_recording(&mut self) {
        self.recording = false;
        #[cfg(debug_assertions)]
        eprintln!("停止录制，共 {} 帧", self.buffer.len());
    }

    /// 开始回放
    pub fn start_playback(&mut self) {
        if !self.buffer.is_empty() {
            self.playing = true;
            self.play_index = 0;
            self.recording = false;
            #[cfg(debug_assertions)]
            eprintln!("开始回放 {} 帧", self.buffer.len());
        }
    }

    /// 停止回放
    pub fn stop_playback(&mut self) {
        self.playing = false;
        self.play_index = 0;
    }

    /// 获取当前回放帧（如果正在回放）
    pub fn current_frame(&self) -> Option<InputFrame> {
        if self.playing && self.play_index < self.buffer.len() {
            Some(self.buffer[self.play_index])
        } else {
            None
        }
    }

    /// 推进回放指针
    pub fn advance_playback(&mut self) {
        if self.playing {
            self.play_index += 1;
        }
    }

    /// 是否有录制数据
    #[must_use]
    pub fn has_recordings(&self) -> bool {
        !self.buffer.is_empty()
    }

    /// 录制帧数
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.buffer.len()
    }
}

// 全局回放管理器（简单实现，使用线程局部存储）
thread_local! {
    static REPLAY: RefCell<ReplayManager> = RefCell::new(ReplayManager::new());
}

/// 获取全局回放管理器引用
pub fn with_replay<F, R>(f: F) -> R
where
    F: FnOnce(&mut ReplayManager) -> R,
{
    REPLAY.with(|r| f(&mut r.borrow_mut()))
}

/// 开始录制
pub fn start_recording() {
    with_replay(|r| r.start_recording());
}

/// 停止录制
pub fn stop_recording() {
    with_replay(|r| r.stop_recording());
}

/// 开始回放
pub fn start_playback() {
    with_replay(|r| r.start_playback());
}

/// 记录输入帧
pub fn record_frame(input: &InputState) {
    with_replay(|r| r.record(input));
}

/// 获取当前回放帧
pub fn current_frame() -> Option<InputFrame> {
    with_replay(|r| r.current_frame())
}

/// 推进回放
pub fn advance_playback() {
    with_replay(|r| r.advance_playback());
}

/// 是否有录制
pub fn has_recordings() -> bool {
    with_replay(|r| r.has_recordings())
}

/// 录制帧数
pub fn frame_count() -> usize {
    with_replay(|r| r.frame_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_manager_records_frames() {
        let mut mgr = ReplayManager::new();
        mgr.start_recording();
        let input = InputState { a: true, ..InputState::new() };
        mgr.record(&input);
        mgr.record(&InputState::new());
        mgr.stop_recording();
        assert_eq!(mgr.frame_count(), 2);
    }

    #[test]
    fn replay_manager_plays_back() {
        let mut mgr = ReplayManager::new();
        mgr.start_recording();
        let input = InputState { a: true, ..InputState::new() };
        mgr.record(&input);
        mgr.stop_recording();

        mgr.start_playback();
        assert!(mgr.playing);
        assert!(mgr.current_frame().is_some());
        assert!(mgr.current_frame().unwrap().a);

        mgr.advance_playback();
        assert!(mgr.current_frame().is_none()); // 超出范围
    }

    #[test]
    fn ring_buffer_wraps() {
        let mut mgr = ReplayManager::new();
        mgr.start_recording();
        for _ in 0..RING_BUFFER_SIZE + 10 {
            mgr.record(&InputState::new());
        }
        assert_eq!(mgr.frame_count(), RING_BUFFER_SIZE);
    }

    #[test]
    fn playback_before_recording_returns_none() {
        let mgr = ReplayManager::new();
        assert!(mgr.current_frame().is_none());
    }
}
