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
    DialogueChoice { label: "看看商品", target_page: 1, require_flag: Some("met_ivan"), require_affinity: None, set_flag: Some("_open_shop") },
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

// ── Innkeeper NPC (stub) ──

const INNKEEPER_PAGE1_TEXT: &str = "欢迎来到旅馆！一晚10金币，要休息一下吗？";
const INNKEEPER_PAGE1_LINES: &[DialogueLine] = &[
    DialogueLine { text: INNKEEPER_PAGE1_TEXT, actions: &[DialogueAction::SetFlag("met_innkeeper")] },
];
const INNKEEPER_PAGE1_CHOICES: &[DialogueChoice] = &[
    DialogueChoice { label: "住宿 (10G)", target_page: 0, require_flag: None, require_affinity: None, set_flag: Some("_rest_at_inn") },
    DialogueChoice { label: "不用了", target_page: 1, require_flag: None, require_affinity: None, set_flag: None },
];
const INNKEEPER_PAGE2_TEXT: &str = "好的，注意安全！";
const INNKEEPER_PAGE2_LINES: &[DialogueLine] = &[
    DialogueLine { text: INNKEEPER_PAGE2_TEXT, actions: &[] },
];

const IVAN_PAGE5_TEXT: &str = "你们要去 Bilibin？好选择。\n我年轻时去过一次，是个热闹的镇子。\n记得在路上的旅馆休息，山道很危险。";

const IVAN_PAGE5_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("ivan_travel_tip"),
];

const IVAN_PAGE5_LINES: &[DialogueLine] = &[
    DialogueLine { text: IVAN_PAGE5_TEXT, actions: IVAN_PAGE5_ACTIONS },
];

const IVAN_PAGE6_TEXT: &str = "传说中，四个 Elemental Stars 分别藏在世界的四个角落。\n如果你真的找到了一个，那么剩下的三个也在呼唤你。\n世界之轮已经开始转动了…";

const IVAN_PAGE6_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("ivan_revealed_stars"),
];

const IVAN_PAGE6_LINES: &[DialogueLine] = &[
    DialogueLine { text: IVAN_PAGE6_TEXT, actions: IVAN_PAGE6_ACTIONS },
];

const MIA_PAGE5_TEXT: &str = "池塘里的闪光…现在我知道那是精灵的力量了。\n也许我也能感受到它。\n保重，Isaac。";

const MIA_PAGE5_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("mia_farewell"),
];

const MIA_PAGE5_LINES: &[DialogueLine] = &[
    DialogueLine { text: MIA_PAGE5_TEXT, actions: MIA_PAGE5_ACTIONS },
];

const MIA_PAGE6_TEXT: &str = "你收集到 Djinn 了！\n它们是很特别的精灵，只选择有缘人。\n看来你是真正的 Adept。";

const MIA_PAGE6_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("mia_djinn_talk"),
];

const MIA_PAGE6_LINES: &[DialogueLine] = &[
    DialogueLine { text: MIA_PAGE6_TEXT, actions: MIA_PAGE6_ACTIONS },
];

const GARSMIN_PAGE8_TEXT: &str = "Elemental Star…你做到了。\n在圣祭坛沉睡了数百年的力量终于重现于世。\n你知道吗？传说中一共有四颗 Elemental Stars。\n每一颗都代表着一种元素力量。";

const GARSMIN_PAGE8_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garsmin_revealed_stars"),
];

const GARSMIN_PAGE8_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE8_TEXT, actions: GARSMIN_PAGE8_ACTIONS },
];

const GARSMIN_PAGE9_TEXT: &str = "数百年前，伟大的贤者们将炼金术封印在\n四颗 Elemental Stars 之中。\n你的祖先就是其中一位守护者。\n现在，这责任落在了你肩上。";

const GARSMIN_PAGE9_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garsmin_ancestor"),
];

const GARSMIN_PAGE9_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE9_TEXT, actions: GARSMIN_PAGE9_ACTIONS },
];

const GARSMIN_PAGE10_TEXT: &str = "离开 Vale 吧，孩子。\n世界在等待着你。\n记住——精灵的力量来自于内心，\n而不是来自于 Elemental Stars。";

const GARSMIN_PAGE10_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garsmin_final_blessing"),
];

const GARSMIN_PAGE10_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARSMIN_PAGE10_TEXT, actions: GARSMIN_PAGE10_ACTIONS },
];

const GARRET_PAGE4_TEXT: &str = "长老说得对！我们该出发了。\n我已经准备好了，你呢？\nIsaac，一起闯荡世界吧！";

const GARRET_PAGE4_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garet_ready_to_go"),
];

const GARRET_PAGE4_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARRET_PAGE4_TEXT, actions: GARRET_PAGE4_ACTIONS },
];

const GARRET_PAGE5_TEXT: &str = "哇，外面的空气真不一样！\n你看那边的森林，比 Vale 周围的茂密多了。\n我感觉我们很快就会遇到有趣的事！";

const GARRET_PAGE5_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("garet_excited"),
];

const GARRET_PAGE5_LINES: &[DialogueLine] = &[
    DialogueLine { text: GARRET_PAGE5_TEXT, actions: GARRET_PAGE5_ACTIONS },
];

const HERMIT_PAGE1_TEXT: &str = "嘘…小点声，我在冥想。\n森林里住着许多古老的生灵，\n它们不喜欢被打扰。";

const HERMIT_PAGE1_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("met_hermit"),
];

const HERMIT_PAGE1_LINES: &[DialogueLine] = &[
    DialogueLine { text: HERMIT_PAGE1_TEXT, actions: HERMIT_PAGE1_ACTIONS },
];

const HERMIT_PAGE2_TEXT: &str = "你身上有精灵力的气息…\n你是 Adept 吧？难怪森林对你这么友好。\n向东走有个洞穴，那里的能量很不稳定。";

const HERMIT_PAGE2_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("hermit_hint"),
];

const HERMIT_PAGE2_LINES: &[DialogueLine] = &[
    DialogueLine { text: HERMIT_PAGE2_TEXT, actions: HERMIT_PAGE2_ACTIONS },
];

const PROSPECTOR_PAGE1_TEXT: &str = "嘿！你也是来挖宝的？\n别误会，这洞里没什么值钱的东西…\n不过据说深处有奇怪的光芒。";

const PROSPECTOR_PAGE1_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("met_prospector"),
];

const PROSPECTOR_PAGE1_LINES: &[DialogueLine] = &[
    DialogueLine { text: PROSPECTOR_PAGE1_TEXT, actions: PROSPECTOR_PAGE1_ACTIONS },
];

const PROSPECTOR_PAGE2_TEXT: &str = "你要往深处走？那小心点。\n我听到过沉重的脚步声，像是有什么大家伙在睡觉。\n……也许你该带上些恢复药。";

const PROSPECTOR_PAGE2_ACTIONS: &[DialogueAction] = &[
    DialogueAction::SetFlag("prospector_warning"),
];

const PROSPECTOR_PAGE2_LINES: &[DialogueLine] = &[
    DialogueLine { text: PROSPECTOR_PAGE2_TEXT, actions: PROSPECTOR_PAGE2_ACTIONS },
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
            DialoguePage { lines: GARSMIN_PAGE8_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE9_LINES, choices: &[] },
            DialoguePage { lines: GARSMIN_PAGE10_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_garsmin"),
    }),
    ("ivan", &DialogueScript {
        pages: &[
            DialoguePage { lines: IVAN_PAGE1_LINES, choices: IVAN_PAGE1_CHOICES },
            DialoguePage { lines: IVAN_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: IVAN_PAGE3_LINES, choices: &[] },
            DialoguePage { lines: IVAN_PAGE4_LINES, choices: &[] },
            DialoguePage { lines: IVAN_PAGE5_LINES, choices: &[
                DialogueChoice { label: "继续说", target_page: 5, require_flag: Some("garet_ready"), require_affinity: None, set_flag: None },
            ]},
            DialoguePage { lines: IVAN_PAGE6_LINES, choices: &[
                DialogueChoice { label: "继续说", target_page: 0, require_flag: Some("ivan_travel_tip"), require_affinity: None, set_flag: None },
            ]},
        ],
        start_flag: Some("talked_to_ivan"),
    }),
    ("mia", &DialogueScript {
        pages: &[
            DialoguePage { lines: MIA_PAGE1_LINES, choices: &[] },
            DialoguePage { lines: MIA_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: MIA_PAGE3_LINES, choices: &[] },
            DialoguePage { lines: MIA_PAGE4_LINES, choices: &[] },
            DialoguePage { lines: MIA_PAGE5_LINES, choices: &[
                DialogueChoice { label: "继续说", target_page: 0, require_flag: Some("completed_sol_sanctum"), require_affinity: None, set_flag: None },
            ]},
            DialoguePage { lines: MIA_PAGE6_LINES, choices: &[
                DialogueChoice { label: "继续说", target_page: 0, require_flag: Some("mia_farewell"), require_affinity: None, set_flag: None },
            ]},
        ],
        start_flag: Some("talked_to_mia"),
    }),
    ("garet", &DialogueScript {
        pages: &[
            DialoguePage { lines: GARRET_PAGE1_LINES, choices: &[] },
            DialoguePage { lines: GARRET_PAGE2_LINES, choices: &[] },
            DialoguePage { lines: GARRET_PAGE3_LINES, choices: &[] },
            DialoguePage { lines: GARRET_PAGE4_LINES, choices: &[
                DialogueChoice { label: "继续说", target_page: 0, require_flag: Some("garsmin_final_blessing"), require_affinity: None, set_flag: None },
            ]},
            DialoguePage { lines: GARRET_PAGE5_LINES, choices: &[
                DialogueChoice { label: "继续说", target_page: 0, require_flag: Some("garet_ready_to_go"), require_affinity: None, set_flag: None },
            ]},
        ],
        start_flag: Some("talked_to_garet"),
    }),
    ("innkeeper", &DialogueScript {
        pages: &[
            DialoguePage { lines: INNKEEPER_PAGE1_LINES, choices: INNKEEPER_PAGE1_CHOICES },
            DialoguePage { lines: INNKEEPER_PAGE2_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_innkeeper"),
    }),
    ("forest_hermit", &DialogueScript {
        pages: &[
            DialoguePage { lines: HERMIT_PAGE1_LINES, choices: &[] },
            DialoguePage { lines: HERMIT_PAGE2_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_hermit"),
    }),
    ("cave_prospector", &DialogueScript {
        pages: &[
            DialoguePage { lines: PROSPECTOR_PAGE1_LINES, choices: &[] },
            DialoguePage { lines: PROSPECTOR_PAGE2_LINES, choices: &[] },
        ],
        start_flag: Some("talked_to_prospector"),
    }),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ivan_script_has_six_pages() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.page_count(), 6);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn mia_script_has_six_pages() {
        let s = get_script("mia").unwrap();
        assert_eq!(s.page_count(), 6);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn garsmin_script_has_ten_pages() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.page_count(), 10);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn garet_script_has_five_pages() {
        let s = get_script("garet").unwrap();
        assert_eq!(s.page_count(), 5);
        assert_eq!(s.pages[0].lines.len(), 1);
    }

    #[test]
    fn garet_page2_sets_garet_ready() {
        let s = get_script("garet").unwrap();
        assert_eq!(s.pages[1].lines[0].actions.len(), 1);
        assert_eq!(s.pages[1].lines[0].actions[0], DialogueAction::SetFlag("garet_ready"));
    }

    #[test]
    fn ivan_page5_sets_travel_tip() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.pages[4].lines[0].actions[0], DialogueAction::SetFlag("ivan_travel_tip"));
    }

    #[test]
    fn ivan_page6_sets_revealed_stars() {
        let s = get_script("ivan").unwrap();
        assert_eq!(s.pages[5].lines[0].actions[0], DialogueAction::SetFlag("ivan_revealed_stars"));
    }

    #[test]
    fn mia_page5_sets_farewell() {
        let s = get_script("mia").unwrap();
        assert_eq!(s.pages[4].lines[0].actions[0], DialogueAction::SetFlag("mia_farewell"));
    }

    #[test]
    fn mia_page6_sets_djinn_talk() {
        let s = get_script("mia").unwrap();
        assert_eq!(s.pages[5].lines[0].actions[0], DialogueAction::SetFlag("mia_djinn_talk"));
    }

    #[test]
    fn garsmin_page8_sets_revealed_stars() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[7].lines[0].actions[0], DialogueAction::SetFlag("garsmin_revealed_stars"));
    }

    #[test]
    fn garsmin_page9_sets_ancestor() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[8].lines[0].actions[0], DialogueAction::SetFlag("garsmin_ancestor"));
    }

    #[test]
    fn garsmin_page10_sets_final_blessing() {
        let s = get_script("garsmin").unwrap();
        assert_eq!(s.pages[9].lines[0].actions[0], DialogueAction::SetFlag("garsmin_final_blessing"));
    }

    #[test]
    fn garet_page4_sets_ready_to_go() {
        let s = get_script("garet").unwrap();
        assert_eq!(s.pages[3].lines[0].actions[0], DialogueAction::SetFlag("garet_ready_to_go"));
    }

    #[test]
    fn garet_page5_sets_excited() {
        let s = get_script("garet").unwrap();
        assert_eq!(s.pages[4].lines[0].actions[0], DialogueAction::SetFlag("garet_excited"));
    }

    #[test]
    fn forest_hermit_script_exists() {
        let s = get_script("forest_hermit").unwrap();
        assert_eq!(s.page_count(), 2);
        assert_eq!(s.start_flag, Some("talked_to_hermit"));
    }

    #[test]
    fn cave_prospector_script_exists() {
        let s = get_script("cave_prospector").unwrap();
        assert_eq!(s.page_count(), 2);
        assert_eq!(s.start_flag, Some("talked_to_prospector"));
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
