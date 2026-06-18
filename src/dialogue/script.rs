use crate::dialogue::DialogueAction;

/// 单条对白
#[derive(Debug, Clone, PartialEq)]
pub struct DialogueLine {
    pub text: &'static str,
    pub actions: &'static [DialogueAction],
}

/// 分支选择
#[derive(Debug, Clone, PartialEq)]
pub struct DialogueChoice {
    pub label: &'static str,
    pub target_page: usize,
    /// 需要 flag 名为条件
    pub require_flag: Option<&'static str>,
    /// 需要亲和力下限
    pub require_affinity: Option<i32>,
    /// 选择后设置此 flag
    pub set_flag: Option<&'static str>,
}

/// 一页对话 = 对白行 + 可选分支
#[derive(Debug, Clone, PartialEq)]
pub struct DialoguePage {
    pub lines: &'static [DialogueLine],
    pub choices: &'static [DialogueChoice],
}

/// 完整对话脚本 = 多页顺序播放
#[derive(Debug, Clone, PartialEq)]
pub struct DialogueScript {
    pub pages: &'static [DialoguePage],
    /// 对话启动时自动设置的 flag（用于 one-time 检测）
    pub start_flag: Option<&'static str>,
}

impl DialogueScript {
    pub fn page_count(&self) -> usize { self.pages.len() }
}

/// NPC 对话数据库（线性查找，n≤3 时比二分更快）
pub fn get_script(id: &str) -> Option<&'static DialogueScript> {
    NPC_SCRIPTS.iter().find(|(k, _)| *k == id).map(|(_, s)| *s)
}

// ── Vale 村 NPC 对话 ──
// 每页全文预拼接，避免运行时 `join("\n")` 分配

const IVAN_TEXT: &str = "你好！我是伊万，Vale 村的铁匠。\n我在这儿打铁已经二十年了。\n如果你需要修理装备，随时可以来找我。";

const IVAN_LINES: &[DialogueLine] = &[
    DialogueLine { text: IVAN_TEXT, actions: &[DialogueAction::SetFlag("met_ivan")] },
];

const MIA_TEXT: &str = "你喜欢这里的池塘吗？夏天有很多鱼。\n有时候我能看到水面上有奇怪的闪光……";

const MIA_LINES: &[DialogueLine] = &[
    DialogueLine { text: MIA_TEXT, actions: &[DialogueAction::SetFlag("met_mia")] },
];

const GARSMIN_TEXT: &str = "我是村里的长老。小心山上的怪物！\n最近山上不太平，你最好准备好再出发。\n如果你学会了精灵力，也许还有机会。";

const GARSMIN_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_TEXT, actions: &[DialogueAction::SetFlag("met_garsmin")] },
];

const GARSMIN_PAGE2_TEXT: &str = "你能感受到体内精灵力的流动吗？\n那是一种古老的力量，善用它可以保护村民。";

const GARSMIN_PAGE2_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE2_TEXT, actions: &[DialogueAction::SetFlag("garsmin_trusted")] },
];

const GARSMIN_PAGE3_TEXT: &str = "你是值得信任的伙伴。让我告诉你一个秘密……\n远古时代，人类与精灵共享这片大地。";

const GARSMIN_PAGE3_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE3_TEXT, actions: &[DialogueAction::SetFlag("garsmin_revealed")] },
];

const SAFE_TEXT: &str = "小心谨慎是正确的。在村里多待些日子吧。";

const SAFE_LINES: &[DialogueLine] = &[
    DialogueLine { text: SAFE_TEXT, actions: &[DialogueAction::SetFlag("garsmin_caution")] },
];

const GARSMIN_CHOICES: &[DialogueChoice] = &[
    DialogueChoice { label: "冒险探索", target_page: 2, require_flag: Some("met_garsmin"), require_affinity: None, set_flag: Some("garsmin_bold") },
    DialogueChoice { label: "小心谨慎", target_page: 4, require_flag: None, require_affinity: None, set_flag: Some("garsmin_caution") },
];

const NPC_SCRIPTS: &[(&str, &DialogueScript)] = &[
    ("garsmin", &DialogueScript {
        pages: &[
            DialoguePage { lines: GARSMIN_LINES, choices: GARSMIN_CHOICES },
            DialoguePage { lines: GARSMIN_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE3_LINES, choices: &[] },
            DialoguePage { lines: SAFE_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_garsmin"),
    }),
    ("ivan", &DialogueScript {
        pages: &[
            DialoguePage { lines: IVAN_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_ivan"),
    }),
    ("mia", &DialogueScript {
        pages: &[
            DialoguePage { lines: MIA_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_mia"),
    }),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ivan_script_exists() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.page_count(), 1);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn mia_script_exists() {
        let s = get_script("mia").unwrap();
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn garsmin_script_exists() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn unknown_script_returns_none() {
        assert!(get_script("nonexistent").is_none());
    }

    #[test]
    fn ivan_first_line_has_flag_action() {
        let s = get_script("ivan").unwrap();
        let line = &s.pages[0].lines[0];
        assert_eq!(line.actions.len(), 1);
        assert_eq!(line.actions[0], DialogueAction::SetFlag("met_ivan"));
    }

    #[test]
    fn ivan_start_flag_is_set() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.start_flag, Some("talked_to_ivan"));
    }
}
