//! 全局常量 — 一处修改，全局生效
//!
//! 所有公称尺寸、魔数在此集中管理，禁止在模块中硬编码。

// ── 窗口 ──
/// 默认窗口逻辑宽度（像素）
pub const WINDOW_WIDTH: f32 = 640.0;
/// 默认窗口逻辑高度（像素）
pub const WINDOW_HEIGHT: f32 = 480.0;
/// 目标帧率
pub const TARGET_FPS: u32 = 60;
/// delta 时间上界（秒），防止帧卡顿导致物理飞跃
pub const DELTA_MAX: f32 = 1.0 / 30.0;
/// delta 时间下界（秒），防止无 vsync 时 delta ≈ 0
pub const DELTA_MIN: f32 = 1.0 / 1000.0;

// ── 纹理 ──
/// Mode 7 渲染目标的像素宽度
pub const RENDER_TARGET_W: u32 = 640;
/// Mode 7 渲染目标的像素高度
pub const RENDER_TARGET_H: u32 = 480;
/// GBA 风格纹理滤镜（像素游戏用 Nearest）
/// 在 TextureCache 创建时自动设置 FilterMode::Nearest// ── 地图 ──
/// 单个 tile 的边长（世界坐标单位 = 像素）
pub const TILE_SIZE: f32 = 32.0;
/// 世界坐标到 tile 索引的换算
pub const WORLD_TO_TILE: f32 = 1.0 / TILE_SIZE;
/// 地图尺寸（tile 数）
pub const MAP_WIDTH: u32 = 32;
pub const MAP_HEIGHT: u32 = 32;

// ── Mode 7 相机 ──
/// 默认相机高度
pub const CAMERA_DEFAULT_Z: f32 = 160.0;
/// 默认 VFOV（弧度）
pub const CAMERA_DEFAULT_FOV: f32 = std::f32::consts::FRAC_PI_4;
/// 地平线位置（屏幕高的比例，0=顶部）
pub const HORIZON_RATIO: f32 = 0.4;
/// 雾化起始距离（世界单位）
pub const FOG_START: f32 = 80.0;
/// 雾化最大距离（世界单位）
pub const FOG_END: f32 = 200.0;
/// 雾化最小不透明度
pub const FOG_MIN_ALPHA: f32 = 0.3;
/// 天空渐变色（顶部）
pub const SKY_COLOR_TOP: (u8, u8, u8) = (120, 180, 255);
/// 天空渐变色（地平线）
pub const SKY_COLOR_HORIZON: (u8, u8, u8) = (220, 240, 255);

// ── 相机插值 ──
/// lerp 跟随速度（值越大越快）
pub const CAMERA_LERP_SPEED: f32 = 6.0;

// ── 玩家 ──
/// 移动速度（tile/秒）
pub const PLAYER_SPEED: f32 = 3.0;
/// 加速时速度倍率
pub const PLAYER_SPRINT_MULTIPLIER: f32 = 1.8;
/// 旋转速度（弧度/秒）
pub const PLAYER_TURN_SPEED: f32 = 3.0;

// ── 精灵动画 ──
/// 精灵像素尺寸（程序化绘制）
pub const SPRITE_SIZE: u32 = 16;
/// 行走动画帧率
pub const WALK_ANIM_FPS: f32 = 8.0;

// ── NPC ──
/// NPC 交互距离（tile 单位）
pub const NPC_INTERACT_RANGE: f32 = 1.5;

// ── 对话 ──
/// 打字机效果字符间隔（秒）
pub const TYPEWRITER_INTERVAL: f32 = 0.05;

// ── 精灵力 ──
/// PP 行走恢复间隔（秒）
pub const PP_RECOVER_INTERVAL: f32 = 10.0;
/// PP 行走恢复量
pub const PP_RECOVER_AMOUNT: u32 = 1;

// ── 战斗 ──
/// 物理攻击基础倍率
pub const PHYSICAL_ATK_MULTIPLIER: f32 = 2.0;
/// 物理防御基础倍率
pub const PHYSICAL_DEF_MULTIPLIER: f32 = 1.5;
/// 暴击率系数
pub const CRIT_RATE_COEFFICIENT: f32 = 0.15;
/// 暴击最大概率
pub const CRIT_RATE_MAX: f32 = 0.4;
/// 暴击伤害倍率
pub const CRIT_DAMAGE_MULTIPLIER: f32 = 1.5;
/// 元素克制伤害倍率
pub const ELEMENT_ADVANTAGE_MULTIPLIER: f32 = 1.25;
/// 元素抗性减伤倍率
pub const ELEMENT_RESISTANCE_MULTIPLIER: f32 = 0.75;
/// 逃跑成功率分母系数
pub const FLEE_SPEED_COEFFICIENT: f32 = 0.5;
/// 逃跑最大成功率
pub const FLEE_MAX_CHANCE: f32 = 0.9;

// ── 音频 ──
/// 音频采样率（Hz）
pub const AUDIO_SAMPLE_RATE: u32 = 44100;
/// 确认音频率（Hz）
pub const SFX_CONFIRM_FREQ: f32 = 440.0;
/// 取消音频率（Hz）
pub const SFX_CANCEL_FREQ: f32 = 220.0;
/// 确认音持续时间（毫秒）
pub const SFX_CONFIRM_MS: u32 = 100;
/// 取消音持续时间（毫秒）
pub const SFX_CANCEL_MS: u32 = 80;
