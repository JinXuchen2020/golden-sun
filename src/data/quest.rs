//! 任务/日志系统 — QuestLog 结构体和任务管理

/// 任务条目
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestEntry {
    /// 任务 ID
    pub id: &'static str,
    /// 任务名称（显示用）
    pub name: &'static str,
    /// 任务描述/提示
    pub hint: &'static str,
    /// 是否已完成
    pub completed: bool,
}

impl QuestEntry {
    #[must_use]
    pub const fn new(id: &'static str, name: &'static str, hint: &'static str) -> Self {
        Self { id, name, hint, completed: false }
    }
}

/// 任务日志 — 当前活跃任务列表
#[derive(Debug, Clone, Default)]
pub struct QuestLog {
    pub entries: Vec<QuestEntry>,
}

impl QuestLog {
    #[must_use]
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// 添加任务（若已存在则跳过）
    pub fn add(&mut self, entry: QuestEntry) {
        if !self.entries.iter().any(|e| e.id == entry.id) {
            self.entries.push(entry);
        }
    }

    /// 标记任务完成
    pub fn complete(&mut self, id: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.completed = true;
        }
    }

    /// 获取当前第一个未完成的任务（用于 HUD 提示）
    #[must_use]
    pub fn active_hint(&self) -> Option<&'static str> {
        self.entries.iter().find(|e| !e.completed).map(|e| e.hint)
    }

    /// 获取当前第一个未完成的任务名称
    #[must_use]
    pub fn active_name(&self) -> Option<&'static str> {
        self.entries.iter().find(|e| !e.completed).map(|e| e.name)
    }

    /// 获取未完成的任务数量
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.completed).count()
    }
}

/// 预定义任务 — 游戏开局即添加
pub fn default_quests() -> Vec<QuestEntry> {
    vec![
        QuestEntry::new("intro_talk", "初遇村民", "与 Vale 村的村民交谈，了解情况"),
        QuestEntry::new("first_psynergy", "初次精灵力", "学会并使用第一种精灵力"),
        QuestEntry::new("explore_forest", "探索密林", "离开 Vale 村，探索野外森林"),
        QuestEntry::new("first_djinn", "初遇 Djinn", "在地图上找到并收集第一个 Djinn 精灵"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_log_empty_by_default() {
        let log = QuestLog::new();
        assert_eq!(log.active_count(), 0);
        assert!(log.active_hint().is_none());
    }

    #[test]
    fn quest_log_add_and_active() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("test_quest", "测试任务", "这是一个测试"));
        assert_eq!(log.active_count(), 1);
        assert_eq!(log.active_hint(), Some("这是一个测试"));
        assert_eq!(log.active_name(), Some("测试任务"));
    }

    #[test]
    fn quest_log_duplicate_skipped() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("dup", "Dup", "Hint1"));
        log.add(QuestEntry::new("dup", "Dup2", "Hint2"));
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0].hint, "Hint1");
    }

    #[test]
    fn quest_log_complete_marks_done() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务一", "提示1"));
        log.add(QuestEntry::new("q2", "任务二", "提示2"));
        log.complete("q1");
        assert_eq!(log.active_count(), 1);
        assert_eq!(log.active_name(), Some("任务二"));
    }

    #[test]
    fn quest_log_complete_nonexistent_is_noop() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务一", "提示"));
        log.complete("nonexistent");
        assert_eq!(log.active_count(), 1);
    }

    #[test]
    fn quest_log_all_completed_returns_none() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务", "提示"));
        log.complete("q1");
        assert!(log.active_hint().is_none());
    }

    #[test]
    fn default_quests_provides_four_entries() {
        let quests = default_quests();
        assert_eq!(quests.len(), 4);
    }
}
