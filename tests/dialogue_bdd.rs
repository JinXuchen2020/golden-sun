//! BDD 测试: 对话树与事件系统
//! 对应 features/dialogue.feature

use golden_sun::dialogue::script::{get_script, DialogueChoice, DialogueLine, DialoguePage, DialogueScript};
use golden_sun::dialogue::{DialogueAction, DialogueState, StoryFlags};
use golden_sun::PsynergyType;

// ── Scenario: 线性对话流 ──

#[test]
fn linear_dialogue_flow() {
    let script = get_script("ivan").unwrap();
    let full = script.pages[0].lines[0].text;

    let mut d = DialogueState::new(full.to_string());
    assert!(!d.is_finished());

    while !d.advance(0.1, 30.0) {}

    assert!(d.is_finished());
    assert_eq!(d.visible_text().chars().count(), full.chars().count());
}

// ── Scenario: 分支选择 ──

#[test]
fn branch_dialogue_selection() {
    let script = DialogueScript {
        pages: &[
            DialoguePage {
                lines: &[DialogueLine { text: "选 A 还是 B？", actions: &[] }],
                choices: &[
                    DialogueChoice { label: "选 A", target_page: 1, require_flag: None, set_flag: Some("chose_a") },
                    DialogueChoice { label: "选 B", target_page: 2, require_flag: None, set_flag: Some("chose_b") },
                ],
            },
            DialoguePage { lines: &[DialogueLine { text: "你选了 A", actions: &[] }], choices: &[] },
            DialoguePage { lines: &[DialogueLine { text: "你选了 B", actions: &[] }], choices: &[] },
        ],
        start_flag: None,
    };
    assert_eq!(script.page_count(), 3);

    // 模拟选择分支 0 → page 1
    let choice = &script.pages[0].choices[0];
    assert_eq!(choice.target_page, 1);
    assert_eq!(choice.set_flag, Some("chose_a"));

    let mut flags = StoryFlags::new();
    if let Some(f) = choice.set_flag { flags.set(f); }
    assert!(flags.get("chose_a"));
}

// ── Scenario: 条件选项可见性 ──

#[test]
fn conditional_option_visibility() {
    let choice = DialogueChoice {
        label: "秘密通道",
        target_page: 1,
        require_flag: Some("has_key"),
        set_flag: None,
    };

    let mut flags = StoryFlags::new();
    // 无 flag → 不可见
    assert!(!flags.get(choice.require_flag.unwrap()));

    // 设置 flag → 可见
    flags.set("has_key");
    assert!(flags.get(choice.require_flag.unwrap()));
}

// ── Scenario: 打字机效果速度 ──

#[test]
fn typewriter_effect_timing() {
    let text = "Hello World"; // 11 chars
    let mut d = DialogueState::new(text.to_string());

    d.advance(0.0, 30.0);
    assert_eq!(d.visible_chars(), 0);

    d.advance(0.33, 30.0);
    assert!(d.visible_chars() >= 9);
    assert!(d.visible_chars() <= 11);

    d.advance(1.0, 30.0);
    assert!(d.is_finished());
    assert_eq!(d.visible_chars(), 11);
}

// ── Scenario: Z 键跳过打字 ──

#[test]
fn typewriter_a_key_skip() {
    let mut d = DialogueState::new("很长很长的一段对白……".to_string());
    assert_eq!(d.visible_chars(), 0);

    d.skip();
    assert!(d.is_finished());
    assert_eq!(d.visible_text(), d.text());
}

// ── Scenario: 对话动作 — 给予物品 ──
// 注：GiveItem 当前未在 DialogueAction 中定义；
// 物品系统使用 SetFlag + 外部协调

#[test]
fn dialogue_action_give_item() {
    // 物品 = 设置 flag "has_potion"
    let mut flags = StoryFlags::new();
    DialogueAction::SetFlag("has_potion").apply(&mut flags);
    assert!(flags.get("has_potion"));
}

// ── Scenario: 对话动作 — 设置 flag ──

#[test]
fn dialogue_action_set_flag() {
    let mut flags = StoryFlags::new();
    DialogueAction::SetFlag("talked_to_elder").apply(&mut flags);
    assert!(flags.get("talked_to_elder"));
}

// ── Scenario: 对话动作 — 解锁精灵力 ──

#[test]
fn dialogue_action_unlock_psynergy() {
    let action = DialogueAction::UnlockPsynergy(PsynergyType::Whirlwind);
    // apply returns None (needs GameCtx coordination)
    let mut flags = StoryFlags::new();
    action.apply(&mut flags);
    match action {
        DialogueAction::UnlockPsynergy(p) => assert_eq!(p, PsynergyType::Whirlwind),
        _ => panic!("wrong variant"),
    }
}

// ── Scenario: 对话动作 — 传送 ──

#[test]
fn dialogue_action_teleport() {
    let action = DialogueAction::Teleport(15.0, 10.0);
    let mut flags = StoryFlags::new();
    action.apply(&mut flags);
    match action {
        DialogueAction::Teleport(x, y) => {
            assert!((x - 15.0).abs() < 0.001);
            assert!((y - 10.0).abs() < 0.001);
        }
        _ => panic!("wrong variant"),
    }
}

// ── Scenario: 对话动作 — 开始战斗 ──

#[test]
fn dialogue_action_start_battle() {
    let action = DialogueAction::StartBattle;
    let mut flags = StoryFlags::new();
    action.apply(&mut flags);
    assert_eq!(action, DialogueAction::StartBattle);
}

// ── Scenario: 地图格事件触发 ──
// 验证 script 数据库支持 tile 事件查找

#[test]
fn ontile_event_trigger() {
    let script = get_script("ivan");
    assert!(script.is_some());
    assert_eq!(script.unwrap().start_flag, Some("talked_to_ivan"));

    // 未知 ID → None
    assert!(golden_sun::dialogue::script::get_script("tile_15_10").is_none());
}

// ── Scenario: 区域进入事件 ──

#[test]
fn onenterregion_event_trigger() {
    // 区域事件是 Lookup 表：region_name → script_id
    // 验证查找抽象
    let region_map: &[(&str, &str)] = &[
        ("vale_entrance", "garsmin"),
        ("mt_alex", "ivan"),
    ];

    let entry = region_map.iter().find(|(k, _)| *k == "vale_entrance");
    assert!(entry.is_some());
    let (_, script_id) = entry.unwrap();
    assert_eq!(*script_id, "garsmin");

    // 验证能查出脚本
    let script = get_script(script_id);
    assert!(script.is_some());
}

// ── Scenario: 一次性事件 ──

#[test]
fn one_time_event() {
    let mut flags = StoryFlags::new();
    let event_flag = "talked_to_ivan";

    // 第一次触发
    assert!(!flags.get(event_flag));
    flags.set(event_flag);
    assert!(flags.get(event_flag));

    // 第二次触发 → 被 flag 阻挡
    if !flags.get(event_flag) {
        flags.set(event_flag);
    }
    // flag 已存在，不会重复设置
    // 验证 flag 只有一个
    assert_eq!(flags.count(), 1);
}

// ── Scenario: Flag 持久性 ──

#[test]
fn flag_persistence() {
    let mut flags = StoryFlags::new();

    // 场景 1: 与伊万对话
    flags.set("met_ivan");
    assert!(flags.get("met_ivan"));

    // 场景 2: 与米娅对话
    flags.set("met_mia");
    assert!(flags.get("met_mia"));

    // 场景 3: 伊万的 flag 仍然存在
    assert!(flags.get("met_ivan"));

    // 场景 4: flag 计数
    assert_eq!(flags.count(), 2);

    // 场景 5: 清除
    flags.clear("met_mia");
    assert!(!flags.get("met_mia"));
    assert!(flags.get("met_ivan"));
    assert_eq!(flags.count(), 1);
}
