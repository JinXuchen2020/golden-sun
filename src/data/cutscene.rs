//! 过场动画系统 - 脚本化叙事序列

use crate::SceneId;
use crate::engine::ItemType;

/// 过场动画指令
#[derive(Debug, Clone)]
pub enum CutsceneCmd {
    /// 设置剧情 flag
    SetFlag(&'static str),
    /// 显示对话框文字（自动推进，无需玩家按 A）
    AutoDialog(&'static str),
    /// 等待玩家按 A 继续
    WaitForConfirm,
    /// 等待指定秒数
    Wait(f32),
    /// 全屏闪光
    Flash(f32),
    /// 渐黑
    FadeToBlack(f32),
    /// 从渐黑恢复
    FadeFromBlack(f32),
    /// 移动 NPC (index, target_x, target_y, speed)
    MoveNpc(usize, f32, f32, f32),
    /// 设置摄像机 (x, y)
    SetCamera(f32, f32),
    /// 开始战斗
    StartBattle,
    /// 切换场景
    SwitchScene(SceneId),
    /// 播放音效
    PlaySfx(&'static str),
    /// 播放背景音乐
    PlayBgm(&'static str),
    /// 给予道具
    GiveItem(ItemType, u32),
    /// 升级
    LevelUp(u32),
}

/// 过场动画定义
#[derive(Debug, Clone)]
pub struct Cutscene {
    pub id: &'static str,
    pub commands: &'static [CutsceneCmd],
}

/// 所有过场动画
pub fn all_cutscenes() -> &'static [Cutscene] {
    &[OPENING_PROLOGUE, OPENING_CUTSCENE, AFTER_SANCTUM, LEAVE_VALE]
}

// ── 序章（开场前） ──

const PROLOGUE_DIALOG1: &str = "在遥远的古代，人类掌握了炼金术的力量。";
const PROLOGUE_DIALOG2: &str = "地、水、火、风——四种元素之力被少数人操控，\n他们被称为「精灵使」(Adept)。";
const PROLOGUE_DIALOG3: &str = "然而炼金术的力量过于强大，几乎将世界推向毁灭的边缘。";

pub const OPENING_PROLOGUE: Cutscene = Cutscene {
    id: "opening_prologue",
    commands: &[
        CutsceneCmd::FadeToBlack(1.5),
        CutsceneCmd::AutoDialog(PROLOGUE_DIALOG1),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(PROLOGUE_DIALOG2),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(PROLOGUE_DIALOG3),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::FadeFromBlack(1.5),
        CutsceneCmd::SetFlag("prologue_seen"),
    ],
};

// ── 开场动画 ──

const OPENING_DIALOG1: &str = "远古时代，炼金术掌控着世界的力量。";
const OPENING_DIALOG2: &str = "四种元素 - 地、水、火、风 - 被引导者操控，\n他们被称为「精灵使」(Adept)。";
const OPENING_DIALOG3: &str = "但炼金术的力量太过强大，几乎毁灭了世界。";
const OPENING_DIALOG4: &str = "为了拯救世界，古代的贤者们将炼金术封印在\n圣祭坛(Sol Sanctum)深处…";
const OPENING_DIALOG5: &str = "…并创造 Elemental Stars 作为钥匙，\n将这股力量永远封锁。";
const OPENING_DIALOG6: &str = "如今，数百年过去了…\nVale 村——一个被群山环抱的宁静村庄——";
const OPENING_DIALOG7: &str = "村民过着与世隔绝的生活，\n古老的传说渐渐被遗忘…";
const OPENING_DIALOG8: &str = "但封印正在减弱，\n山上的怪物变得越来越多…";

pub const OPENING_CUTSCENE: Cutscene = Cutscene {
    id: "opening",
    commands: &[
        CutsceneCmd::FadeToBlack(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG1),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG2),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG3),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG4),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG5),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG6),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG7),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(OPENING_DIALOG8),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::FadeFromBlack(1.0),
        CutsceneCmd::SetFlag("opening_seen"),
    ],
};

// ── Sol Sanctum Boss 战后 ──

const SANCTUM_DLG1: &str = "石像崩塌了，Elemental Star 落在你手中。";
const SANCTUM_DLG2: &str = "你能感受到它蕴含的无穷力量。";
const SANCTUM_DLG3: &str = "远处传来古老的声音：\n「Elemental Star 的守护者已被击败…」";
const SANCTUM_DLG4: &str = "「…但真正的旅程才刚刚开始。\n寻找其他的 Elemental Stars，\n否则世界的平衡将永远被打破。」";

pub const AFTER_SANCTUM: Cutscene = Cutscene {
    id: "after_sanctum",
    commands: &[
        CutsceneCmd::AutoDialog(SANCTUM_DLG1),
        CutsceneCmd::AutoDialog(SANCTUM_DLG2),
        CutsceneCmd::AutoDialog(SANCTUM_DLG3),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(SANCTUM_DLG4),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::SetFlag("completed_sol_sanctum"),
    ],
};

// ── 离开 Vale ──

const LEAVE_DLG1: &str = "Isaac 和 Garet 站在村口，\n回望他们长大的 Vale 村。";
const LEAVE_DLG2: &str = "Garet: 「准备好了吗，Isaac？\n前方的世界在等着我们。」";
const LEAVE_DLG3: &str = "带着 Elemental Star 和长老的嘱托，\n两位年轻的 Adept 踏上了旅程…";

pub const LEAVE_VALE: Cutscene = Cutscene {
    id: "leave_vale",
    commands: &[
        CutsceneCmd::AutoDialog(LEAVE_DLG1),
        CutsceneCmd::AutoDialog(LEAVE_DLG2),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog(LEAVE_DLG3),
        CutsceneCmd::Wait(1.5),
        CutsceneCmd::SetFlag("left_vale"),
    ],
};
