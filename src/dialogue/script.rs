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

/// 重复对话 — 玩家再次与 NPC 交谈时显示的简短问候
pub fn get_repeat_line(npc_id: &str) -> &'static str {
    match npc_id {
        "ivan" => "又是你！需要修理装备吗？\n随时欢迎。",
        "mia" => "你还在村子里啊！池塘里好像有什么东西…\n要不要去看看？",
        "garsmin" => "你的冒险怎么样了？\n记住，力量越大责任越大。",
        "garet" => "Isaac！我们去下一站吧！\n世界在等着我们！",
        _ => "有什么事吗？",
    }
}

// ── System scripts ──

const SANCTUM_CENTER_TEXT: &str = "你触碰到了 Elemental Star 的封印。\n一股强大的力量从圣祭坛中涌出……\n一个守护者出现了！";

const SANCTUM_CENTER_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("_sanctum_boss_ready"),
    DialogueAction::StartBattle,
];

const SANCTUM_AFTERMATH_TEXT: &str = "你获得了 Elemental Star。\n封印被打开了，世界即将迎来巨变。";

const SANCTUM_AFTERMATH_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("completed_sol_sanctum"),
];

const SANCTUM_CENTER_LINE: DialogueLine = DialogueLine {
    text: SANCTUM_CENTER_TEXT,
    actions: SANCTUM_CENTER_ACTIONS,
};

const SANCTUM_CENTER_PAGE: DialoguePage = DialoguePage {
    lines: &[SANCTUM_CENTER_LINE],
    choices: &[],
};

pub const SYSTEM_SANCTUM_CENTER: DialogueScript = DialogueScript {
    pages: &[SANCTUM_CENTER_PAGE],
    start_flag: None,
};

const SANCTUM_AFTERMATH_LINE: DialogueLine = DialogueLine {
    text: SANCTUM_AFTERMATH_TEXT,
    actions: SANCTUM_AFTERMATH_ACTIONS,
};

const SANCTUM_AFTERMATH_PAGE: DialoguePage = DialoguePage {
    lines: &[SANCTUM_AFTERMATH_LINE],
    choices: &[],
};

pub const SYSTEM_SANCTUM_AFTERMATH: DialogueScript = DialogueScript {
    pages: &[SANCTUM_AFTERMATH_PAGE],
    start_flag: None,
};

// ── NPC 对话 ──

const IVAN_PAGE1_TEXT: &str = "你好！我是伊万，Vale 村的铁匠。\n我在这儿打铁已经二十年了。\n如果你需要修理装备，随时可以来找我。";

const IVAN_PAGE1_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("met_ivan"),
];

const IVAN_PAGE1_LINES: &[DialogueLine] = &[
    DialogueLine { text: IVAN_PAGE1_TEXT, actions: IVAN_PAGE1_ACTIONS },
];

const IVAN_PAGE1_CHOICES: &[DialogueChoice] = &[
    DialogueChoice { label: "看看商品", target_page: 1, require_flag: Some("met_ivan"), require_affinity: None, set_flag: None },
];

const IVAN_PAGE2_TEXT: &str = "听说长老告诉了你关于精灵的事？\n确实，Vale 村自古就流传着这样的传说。\n山上的 Sol Sanctum 里封印着远古的力量…";

const IVAN_PAGE2_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("ivan_revealed"),
];

const IVAN_PAGE2_LINES: &[DialogueLine] = &[
    DialogueLine { text: IVAN_PAGE2_TEXT, actions: IVAN_PAGE2_ACTIONS },
];

const IVAN_PAGE3_TEXT: &str = "最近山上的怪物越来越多了。\n据说是因为圣祭坛的封印在减弱…\n年轻人，你有责任去看看。";

const IVAN_PAGE3_LINES: &[DialogueLine] = &[
    DialogueLine { text: IVAN_PAGE3_TEXT, actions: &[] },
];

const IVAN_PAGE4_TEXT: &str = "你从圣祭坛回来了？！\n天啊，这么说那些传说都是真的…\n世界可能会因此而改变。";

const IVAN_PAGE4_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("ivan_heard_news"),
];

const IVAN_PAGE4_LINES: &[DialogueLine] = &[
    DialogueLine { text: IVAN_PAGE4_TEXT, actions: IVAN_PAGE4_ACTIONS },
];

const MIA_PAGE1_TEXT: &str = "你喜欢这里的池塘吗？夏天有很多鱼。\n有时候我能看到水面上有奇怪的闪光……";

const MIA_PAGE1_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("met_mia"),
];

const MIA_PAGE1_LINES: &[DialogueLine] = &[
    DialogueLine { text: MIA_PAGE1_TEXT, actions: MIA_PAGE1_ACTIONS },
];

const MIA_PAGE2_TEXT: &str = "你也注意到水面闪光了吗？\n有人说那是 Sol Sanctum 里的精灵在呼唤…\n只有被选中的 Adept 才能听到。";

const MIA_PAGE2_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("mia_revealed"),
];

const MIA_PAGE2_LINES: &[DialogueLine] = &[
    DialogueLine { text: MIA_PAGE2_TEXT, actions: MIA_PAGE2_ACTIONS },
];

const MIA_PAGE3_TEXT: &str = "Garet 经常在山脚下练习战斗。\n他说他想要变强，保护村子。\n你也许应该去找他聊聊。";

const MIA_PAGE3_LINES: &[DialogueLine] = &[
    DialogueLine { text: MIA_PAGE3_TEXT, actions: &[] },
];

const MIA_PAGE4_TEXT: &str = "所以世界真的要变了…\n传说当炼金术重新苏醒时，\n大地将会发生翻天覆地的变化。\n请保重，Isaac。";

const MIA_PAGE4_LINES: &[DialogueLine] = &[
    DialogueLine { text: MIA_PAGE4_TEXT, actions: &[] },
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

const GARSMIN_PAGE4_TEXT: &str = "小心谨慎是正确的。在村里多待些日子吧。";

const GARSMIN_PAGE4_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE4_TEXT, actions: &[DialogueAction::SetFlag("garsmin_caution")] },
];

const GARSMIN_PAGE5_TEXT: &str = "你体内有精灵力的天赋，我看得出来。\nSol Sanctum 就在村后山的顶端。\n去那里看看吧，但小心山上的怪物。";

const GARSMIN_PAGE5_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garsmin_sent_to_sanctum"),
];

const GARSMIN_PAGE5_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE5_TEXT, actions: GARSMIN_PAGE5_ACTIONS },
];

const GARSMIN_PAGE6_TEXT: &str = "你去了 Sol Sanctum… 古老的封印被打开了。\n这不是你的错，这是命运的指引。\n现在，世界各地的 Elemental Stars 正在等待被找到。";

const GARSMIN_PAGE6_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garsmin_complete_1"),
];

const GARSMIN_PAGE6_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE6_TEXT, actions: GARSMIN_PAGE6_ACTIONS },
];

const GARSMIN_PAGE7_TEXT: &str = "带上 Garet，离开 Vale 吧。\n前往 Bilibin 镇，那里有更多的线索。\n记住，你和精灵之间的联系就是世界的希望。";

const GARSMIN_PAGE7_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garsmin_farewell"),
];

const GARSMIN_PAGE7_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE7_TEXT, actions: GARSMIN_PAGE7_ACTIONS },
];

const GARSMIN_CHOICES: &[DialogueChoice] = &[
    DialogueChoice { label: "冒险探索", target_page: 2, require_flag: Some("met_garsmin"), require_affinity: None, set_flag: Some("garsmin_bold") },
    DialogueChoice { label: "小心谨慎", target_page: 4, require_flag: None, require_affinity: None, set_flag: Some("garsmin_caution") },
];

const GARRET_PAGE1_TEXT: &str = "嘿，Isaac！你也是来找怪物练手的？\n山上的怪物越来越猖狂了。\n听说 Sol Sanctum 那边有动静…";

const GARRET_PAGE1_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("met_garet"),
];

const GARRET_PAGE1_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARRET_PAGE1_TEXT, actions: GARRET_PAGE1_ACTIONS },
];

const GARRET_PAGE2_TEXT: &str = "长老也让你去 Sol Sanctum？\n好，我也正要去看看。\n等我准备好我们就出发！";

const GARRET_PAGE2_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garet_ready"),
];

const GARRET_PAGE2_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARRET_PAGE2_TEXT, actions: GARRET_PAGE2_ACTIONS },
];

const GARRET_PAGE3_TEXT: &str = "我的天…圣祭坛里的东西…\n所以我们真的要踏上旅程了？\nVale 永远是我的家，但外面的世界在召唤。";

const GARRET_PAGE3_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARRET_PAGE3_TEXT, actions: &[] },
];

const NPC_SCRIPTS: &[(&str, &DialogueScript)] = &[
    ("garsmin", &DialogueScript {
        pages: &[
            DialoguePage { lines: GARSMIN_LINES, choices: GARSMIN_CHOICES },
            DialoguePage { lines: GARSMIN_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE3_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE4_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE5_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE6_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE7_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_garsmin"),
    }),
    ("ivan", &DialogueScript {
        pages: &[
            DialoguePage { lines: IVAN_PAGE1_LINES, choices: IVAN_PAGE1_CHOICES },
            DialoguePage { lines: IVAN_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: IVAN_PAGE3_LINES, choices: &[] },
            DialoguePage { lines: IVAN_PAGE4_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_ivan"),
    }),
    ("mia", &DialogueScript {
        pages: &[
            DialoguePage { lines: MIA_PAGE1_LINES, choices: &[] },
            DialoguePage { lines: MIA_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: MIA_PAGE3_LINES, choices: &[] },
            DialoguePage { lines: MIA_PAGE4_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_mia"),
    }),
    ("garet", &DialogueScript {
        pages: &[
            DialoguePage { lines: GARRET_PAGE1_LINES, choices: &[] },
            DialoguePage { lines: GARRET_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: GARRET_PAGE3_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_garet"),
    }),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ivan_script_has_four_pages() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.page_count(), 4);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn mia_script_has_four_pages() {
        let s = get_script("mia").unwrap();
        assert_eq!(s.page_count(), 4);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn garsmin_script_has_seven_pages() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.page_count(), 7);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn garet_script_exists() {
        let s = get_script("garet").unwrap();
        assert_eq!(s.page_count(), 3);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn garet_page1_sets_met_garet() {
        let s = get_script("garet").unwrap();
        let line = &s.pages[0].lines[0];
        assert_eq!(line.actions.len(), 1);
        assert_eq!(line.actions[0], DialogueAction::SetFlag("met_garet"));
    }

    #[test]
    fn garet_page2_requires_met_garsmin() {
        let s = get_script("garet").unwrap();
        assert_eq!(s.pages[1].lines[0].actions.len(), 1);
        assert_eq!(s.pages[1].lines[0].actions[0], DialogueAction::SetFlag("garet_ready"));
    }

    #[test]
    fn ivan_first_page_has_shop_choice() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.pages[0].choices.len(), 1);
        assert_eq!(s.pages[0].choices[0].label, "看看商品");
        assert_eq!(s.pages[0].choices[0].target_page, 1);
        assert_eq!(s.pages[0].choices[0].require_flag, Some("met_ivan"));
    }

    #[test]
    fn ivan_page2_sets_revealed_flag() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.pages[1].lines[0].actions[0], DialogueAction::SetFlag("ivan_revealed"));
    }

    #[test]
    fn ivan_page4_sets_heard_news_flag() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.pages[3].lines[0].actions[0], DialogueAction::SetFlag("ivan_heard_news"));
    }

    #[test]
    fn mia_page2_sets_revealed_flag() {
        let s = get_script("mia").unwrap();
        assert_eq!(s.pages[1].lines[0].actions[0], DialogueAction::SetFlag("mia_revealed"));
    }

    #[test]
    fn garsmin_page5_sets_sent_to_sanctum() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[4].lines[0].actions[0], DialogueAction::SetFlag("garsmin_sent_to_sanctum"));
    }

    #[test]
    fn garsmin_page6_sets_complete_1() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[5].lines[0].actions[0], DialogueAction::SetFlag("garsmin_complete_1"));
    }

    #[test]
    fn garsmin_page7_sets_farewell() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[6].lines[0].actions[0], DialogueAction::SetFlag("garsmin_farewell"));
    }

    #[test]
    fn unknown_script_returns_none() {
        assert!(get_script("nonexistent").is_none());
    }

    #[test]
    fn ivan_start_flag_is_set() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.start_flag, Some("talked_to_ivan"));
    }

    #[test]
    fn sanctum_center_script() {
        assert_eq!(SYSTEM_SANCTUM_CENTER.pages.len(), 1);
        assert_eq!(SYSTEM_SANCTUM_CENTER.pages[0].lines.len(), 1);
        let actions = SYSTEM_SANCTUM_CENTER.pages[0].lines[0].actions;
        assert_eq!(actions.len(), 2);
        assert!(actions.contains(&DialogueAction::SetFlag("_sanctum_boss_ready")));
        assert!(actions.contains(&DialogueAction::StartBattle));
    }

    #[test]
    fn sanctum_aftermath_script() {
        assert_eq!(SYSTEM_SANCTUM_AFTERMATH.pages.len(), 1);
        assert_eq!(SYSTEM_SANCTUM_AFTERMATH.pages[0].lines.len(), 1);
        let actions = SYSTEM_SANCTUM_AFTERMATH.pages[0].lines[0].actions;
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], DialogueAction::SetFlag("completed_sol_sanctum"));
    }
}
