use crate::dialogue::DialogueAction;

/// 单条对白
#[derive(Debug, Clone)]
pub struct DialogueLine {
    pub text: &'static str,
    pub actions: &'static [DialogueAction],
}

/// 分支选择
#[derive(Debug, Clone)]
pub struct DialogueChoice {
    pub label: &'static str,
    pub target_page: usize,
    /// 需要 flag 名为条件
    pub require_flag: Option<&'static str>,
    /// 选择后设置此 flag
    pub set_flag: Option<&'static str>,
}

/// 一页对话 = 对白行 + 可选分支
#[derive(Debug, Clone)]
pub struct DialoguePage {
    pub lines: &'static [DialogueLine],
    pub choices: &'static [DialogueChoice],
}

/// 完整对话脚本 = 多页顺序播放
#[derive(Debug, Clone)]
pub struct DialogueScript {
    pub pages: &'static [DialoguePage],
    /// 对话启动时自动设置的 flag（用于 one-time 检测）
    pub start_flag: Option<&'static str>,
}

impl DialogueScript {
    pub fn page_count(&self) -> usize { self.pages.len() }
}

/// NPC 对话数据库
pub fn get_script(id: &str) -> Option<&'static DialogueScript> {
    NPC_SCRIPTS.iter().find(|(k, _)| *k == id).map(|(_, v)| *v)
}

// ── Vale 村 NPC 对话 ──

const IVAN_LINES: &[DialogueLine] = &[
    DialogueLine { text: "你好！我是伊万，Vale 村的铁匠。", actions: &[DialogueAction::SetFlag("met_ivan")] },
    DialogueLine { text: "我在这儿打铁已经二十年了。", actions: &[] },
    DialogueLine { text: "如果你需要修理装备，随时可以来找我。", actions: &[] },
];

const MIA_LINES: &[DialogueLine] = &[
    DialogueLine { text: "你喜欢这里的池塘吗？夏天有很多鱼。", actions: &[DialogueAction::SetFlag("met_mia")] },
    DialogueLine { text: "有时候我能看到水面上有奇怪的闪光……", actions: &[] },
];

const GARSMIN_LINES: &[DialogueLine] = &[
    DialogueLine { text: "我是村里的长老。小心山上的怪物！", actions: &[DialogueAction::SetFlag("met_garsmin")] },
    DialogueLine { text: "最近山上不太平，你最好准备好再出发。", actions: &[] },
    DialogueLine { text: "如果你学会了精灵力，也许还有机会。", actions: &[] },
];

const NPC_SCRIPTS: &[(&str, &DialogueScript)] = &[
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
    ("garsmin", &DialogueScript {
        pages: &[
            DialoguePage { lines: GARSMIN_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_garsmin"),
    }),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ivan_script_exists() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.page_count(), 1);
        assert_eq!(s.pages[0].lines.len(), 3);
    }

    #[test]
    fn mia_script_exists() {
        let s = get_script("mia").unwrap();
        assert_eq!(s.pages[0].lines.len(), 2);
    }

    #[test]
    fn garsmin_script_exists() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[0].lines.len(), 3);
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
