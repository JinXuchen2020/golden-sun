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
    /// 所属章节
    pub chapter: u32,
    /// 是否已完成
    pub completed: bool,
}

impl QuestEntry {
    #[must_use]
    pub const fn new(id: &'static str, name: &'static str, hint: &'static str, chapter: u32) -> Self {
        Self { id, name, hint, chapter, completed: false }
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

    /// 检查某任务是否存在
    pub fn has(&self, id: &str) -> bool {
        self.entries.iter().any(|e| e.id == id)
    }

    /// 检查某任务是否已完成
    #[must_use]
    pub fn is_completed(&self, id: &str) -> bool {
        self.entries.iter().find(|e| e.id == id).is_some_and(|e| e.completed)
    }

    /// 解锁新任务（若不存在则添加）
    pub fn unlock(&mut self, id: &str) {
        if let Some(entry) = QUEST_TEMPLATES.iter().find(|e| e.id == id) {
            self.add(QuestEntry { id: entry.id, name: entry.name, hint: entry.hint, chapter: entry.chapter, completed: false });
        }
    }
}

/// 任务模板（用于 unlock 时快速查找）
pub const QUEST_TEMPLATES: &[QuestEntry] = &[
    QuestEntry { id: "talk_to_villagers", name: "初访村民", hint: "和 Vale 村的 Ivan、Mia、Garsmin 聊聊", chapter: 1, completed: false },
    QuestEntry { id: "learn_psynergy", name: "觉醒的精灵力", hint: "使用一次精灵力，感受体内的力量", chapter: 1, completed: false },
    QuestEntry { id: "meet_garet", name: "与 Garet 会合", hint: "去村口找 Garet，一起探索 Sol Sanctum", chapter: 1, completed: false },
    QuestEntry { id: "explore_sanctum", name: "圣祭坛之谜", hint: "进入 Vale 后山的 Sol Sanctum", chapter: 1, completed: false },
    QuestEntry { id: "defeat_mythrilgolem", name: "古石像", hint: "击败 Sol Sanctum 深处的 MythrilGolem 守护者", chapter: 1, completed: false },
    QuestEntry { id: "leave_vale", name: "告别 Vale", hint: "带着 Elemental Star 离开 Vale", chapter: 2, completed: false },
    QuestEntry { id: "collect_first_djinn", name: "初遇 Djinn", hint: "在野外找到并收集第一个 Djinn", chapter: 2, completed: false },
    QuestEntry { id: "reach_bilibin", name: "抵达 Bilibin", hint: "穿过密林，到达 Bilibin 镇", chapter: 2, completed: false },
    QuestEntry { id: "explore_cave", name: "洞穴探险", hint: "进入 Kolima 森林深处的洞穴", chapter: 2, completed: false },
    QuestEntry { id: "collect_three_djinn", name: "精灵使之道", hint: "收集 3 个 Djinn", chapter: 2, completed: false },
    QuestEntry { id: "master_psynergy", name: "精灵力大师", hint: "解锁全部 7 种精灵力", chapter: 3, completed: false },
    QuestEntry { id: "first_summon", name: "初次召唤", hint: "在战斗中首次使用召唤", chapter: 3, completed: false },
    QuestEntry { id: "collect_five_djinn", name: "真正的 Adept", hint: "收集 5 个 Djinn", chapter: 3, completed: false },
    QuestEntry { id: "collect_ten_djinn", name: "精灵之力", hint: "收集 10 个 Djinn", chapter: 4, completed: false },
    QuestEntry { id: "collect_all_djinn", name: "炼金术的传承", hint: "收集全部 16 个 Djinn", chapter: 4, completed: false },
];

/// 预定义任务 — 游戏开局即添加
pub fn default_quests() -> Vec<QuestEntry> {
    vec![
        // ── Act 1：Vale 篇（觉醒） ──
        QuestEntry { id: "talk_to_villagers", name: "初访村民", hint: "和 Vale 村的 Ivan、Mia、Garsmin 聊聊", chapter: 1, completed: false },
        QuestEntry { id: "learn_psynergy", name: "觉醒的精灵力", hint: "使用一次精灵力，感受体内的力量", chapter: 1, completed: false },
        QuestEntry { id: "meet_garet", name: "与 Garet 会合", hint: "去村口找 Garet，一起探索 Sol Sanctum", chapter: 1, completed: false },
        QuestEntry { id: "explore_sanctum", name: "圣祭坛之谜", hint: "进入 Vale 后山的 Sol Sanctum", chapter: 1, completed: false },
        QuestEntry { id: "defeat_mythrilgolem", name: "古石像", hint: "击败 Sol Sanctum 深处的 MythrilGolem 守护者", chapter: 1, completed: false },
        // ── Act 2：旅途篇（启程） ──
        QuestEntry { id: "leave_vale", name: "告别 Vale", hint: "带着 Elemental Star 离开 Vale", chapter: 2, completed: false },
        QuestEntry { id: "collect_first_djinn", name: "初遇 Djinn", hint: "在野外找到并收集第一个 Djinn", chapter: 2, completed: false },
        QuestEntry { id: "reach_bilibin", name: "抵达 Bilibin", hint: "穿过密林，到达 Bilibin 镇", chapter: 2, completed: false },
        QuestEntry { id: "explore_cave", name: "洞穴探险", hint: "进入 Kolima 森林深处的洞穴", chapter: 2, completed: false },
        QuestEntry { id: "collect_three_djinn", name: "精灵使之道", hint: "收集 3 个 Djinn", chapter: 2, completed: false },
        // ── Act 3：成长篇（精通） ──
        QuestEntry { id: "master_psynergy", name: "精灵力大师", hint: "解锁全部 7 种精灵力", chapter: 3, completed: false },
        QuestEntry { id: "first_summon", name: "初次召唤", hint: "在战斗中首次使用召唤", chapter: 3, completed: false },
        QuestEntry { id: "collect_five_djinn", name: "真正的 Adept", hint: "收集 5 个 Djinn", chapter: 3, completed: false },
        // ── Act 4：终局篇（传承） ──
        QuestEntry { id: "collect_ten_djinn", name: "精灵之力", hint: "收集 10 个 Djinn", chapter: 4, completed: false },
        QuestEntry { id: "collect_all_djinn", name: "炼金术的传承", hint: "收集全部 16 个 Djinn", chapter: 4, completed: false },
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
        log.add(QuestEntry::new("test_quest", "测试任务", "这是一个测试", 1));
        assert_eq!(log.active_count(), 1);
        assert_eq!(log.active_hint(), Some("这是一个测试"));
        assert_eq!(log.active_name(), Some("测试任务"));
    }

    #[test]
    fn quest_log_duplicate_skipped() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("dup", "Dup", "Hint1", 1));
        log.add(QuestEntry::new("dup", "Dup2", "Hint2", 2));
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0].hint, "Hint1");
    }

    #[test]
    fn quest_log_complete_marks_done() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务一", "提示1", 1));
        log.add(QuestEntry::new("q2", "任务二", "提示2", 1));
        log.complete("q1");
        assert_eq!(log.active_count(), 1);
        assert_eq!(log.active_name(), Some("任务二"));
    }

    #[test]
    fn quest_log_complete_nonexistent_is_noop() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务一", "提示", 1));
        log.complete("nonexistent");
        assert_eq!(log.active_count(), 1);
    }

    #[test]
    fn quest_log_all_completed_returns_none() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务", "提示", 1));
        log.complete("q1");
        assert!(log.active_hint().is_none());
    }

    #[test]
    fn default_quests_provides_fifteen_entries() {
        let quests = default_quests();
        assert_eq!(quests.len(), 15);
    }

    #[test]
    fn quest_log_has_method() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务一", "提示", 1));
        assert!(log.has("q1"));
        assert!(!log.has("nonexistent"));
    }

    #[test]
    fn quest_log_is_completed_method() {
        let mut log = QuestLog::new();
        log.add(QuestEntry::new("q1", "任务一", "提示", 1));
        assert!(!log.is_completed("q1"));
        log.complete("q1");
        assert!(log.is_completed("q1"));
    }

    #[test]
    fn quest_log_unlock_adds_from_templates() {
        let mut log = QuestLog::new();
        log.unlock("leave_vale");
        assert!(log.has("leave_vale"));
        let entry = log.entries.iter().find(|e| e.id == "leave_vale").unwrap();
        assert_eq!(entry.chapter, 2);
    }

    #[test]
    fn quest_templates_has_all_entries() {
        assert_eq!(QUEST_TEMPLATES.len(), 15);
    }
}
