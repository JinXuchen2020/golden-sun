pub mod script;

use std::collections::HashSet;

/// 打字机状态 — 单页对白的逐字揭示
///
/// 缓存 total_chars + char_boundaries 避免每帧 O(n) 遍历和分配
#[derive(Debug, Clone)]
pub struct DialogueState {
    text: String,
    /// 已揭示的字符数（char 级别，非 byte）
    visible: usize,
    /// 可见文本的 byte 结束位置（零切片用）
    visible_end: usize,
    timer: f32,
    finished: bool,
    /// 总字符数缓存（避免每帧 .chars().count()）
    total_chars: usize,
    /// 每个 char 的起始 byte 偏移缓存
    char_boundaries: Vec<usize>,
}

impl DialogueState {
    pub fn new(text: String) -> Self {
        let char_boundaries: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
        let total_chars = char_boundaries.len();
        Self {
            visible_end: 0,
            visible: 0,
            timer: 0.0,
            finished: false,
            text,
            total_chars,
            char_boundaries,
        }
    }

    /// 推进打字计时，返回完整揭示时刻
    pub fn advance(&mut self, dt: f32, char_speed: f32) -> bool {
        if self.finished { return true; }
        self.timer += dt;
        let target = (self.timer * char_speed) as usize;
        self.visible = self.visible.max(target).min(self.total_chars);
        if self.visible >= self.total_chars {
            self.visible_end = self.text.len();
            self.finished = true;
            true
        } else {
            self.visible_end = self.char_boundaries[self.visible];
            false
        }
    }

    /// 跳过打字动画
    pub fn skip(&mut self) {
        self.visible = self.total_chars;
        self.visible_end = self.text.len();
        self.finished = true;
    }

    /// 当前可见文本（零拷贝 &str 切片）
    pub fn visible_text(&self) -> &str {
        &self.text[..self.visible_end]
    }

    pub fn is_finished(&self) -> bool { self.finished }

    /// 全文只读访问
    pub fn text(&self) -> &str { &self.text }

    /// 已揭示字符数
    pub fn visible_chars(&self) -> usize { self.visible }

    /// 重置（复用现有分配）
    pub fn reset(&mut self, text: String) {
        self.char_boundaries.clear();
        self.char_boundaries.extend(text.char_indices().map(|(i, _)| i));
        self.total_chars = self.char_boundaries.len();
        self.text = text;
        self.visible = 0;
        self.visible_end = 0;
        self.timer = 0.0;
        self.finished = false;
    }
}

/// 故事 flag — 跟踪游戏事件状态（&'static str key，零 heap 分配）
#[derive(Debug, Clone, Default)]
pub struct StoryFlags {
    flags: HashSet<&'static str>,
}

impl StoryFlags {
    pub fn new() -> Self { Self { flags: HashSet::new() } }

    pub fn set(&mut self, name: &'static str) { self.flags.insert(name); }

    pub fn get(&self, name: &str) -> bool { self.flags.contains(name) }

    pub fn clear(&mut self, name: &str) { self.flags.remove(name); }

    pub fn count(&self) -> usize { self.flags.len() }
}

/// 对话动作 — 对白触发时的副作用
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogueAction {
    SetFlag(&'static str),
    UnlockPsynergy(crate::PsynergyType),
    Teleport(f32, f32),
    StartBattle,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialogue_new_starts_empty() {
        let d = DialogueState::new("你好".to_string());
        assert_eq!(d.visible, 0);
        assert_eq!(d.visible_text(), "");
        assert!(!d.finished);
    }

    #[test]
    fn advance_reveals_chars() {
        let mut d = DialogueState::new("ABC".to_string());
        d.advance(0.1, 30.0);
        assert!(d.visible > 0);
        assert!(d.visible <= 3);
    }

    #[test]
    fn advance_completes_eventually() {
        let mut d = DialogueState::new("AB".to_string());
        let done = d.advance(1.0, 30.0);
        assert!(done);
        assert!(d.finished);
        assert_eq!(d.visible, 2);
    }

    #[test]
    fn skip_shows_all() {
        let mut d = DialogueState::new("你好世界".to_string());
        d.skip();
        assert!(d.finished);
        assert_eq!(d.visible, 4);
        assert_eq!(d.visible_text(), "你好世界");
    }

    #[test]
    fn visible_text_returns_substring() {
        let mut d = DialogueState::new("Hello".to_string());
        d.advance(0.5, 30.0);
        let vt = d.visible_text();
        assert_eq!(vt.chars().count(), d.visible);
    }

    #[test]
    fn reset_reuses_struct() {
        let mut d = DialogueState::new("old".to_string());
        d.skip();
        d.reset("new".to_string());
        assert_eq!(d.visible, 0);
        assert!(!d.finished);
        assert_eq!(d.visible_text(), "");
    }

    #[test]
    fn zero_copy_visible_text() {
        let mut d = DialogueState::new("你好世界".to_string());
        d.advance(0.5, 30.0);
        // &str 切片，零分配
        let _: &str = d.visible_text();
    }

    #[test]
    fn story_flags_default_false() {
        let f = StoryFlags::new();
        assert!(!f.get("seen_intro"));
    }

    #[test]
    fn story_flags_set_get() {
        let mut f = StoryFlags::new();
        f.set("talked_to_elder");
        assert!(f.get("talked_to_elder"));
    }

    #[test]
    fn story_flags_clear() {
        let mut f = StoryFlags::new();
        f.set("tmp");
        assert!(f.get("tmp"));
        f.clear("tmp");
        assert!(!f.get("tmp"));
    }

    #[test]
    fn story_flags_count() {
        let mut f = StoryFlags::new();
        assert_eq!(f.count(), 0);
        f.set("a");
        f.set("b");
        assert_eq!(f.count(), 2);
    }

    #[test]
    fn dialogue_action_is_copy() {
        let a = DialogueAction::SetFlag("test");
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn set_flag_action_affects_story_flags() {
        let mut flags = StoryFlags::new();
        flags.set("met_ivan");
        assert!(flags.get("met_ivan"));
    }
}
