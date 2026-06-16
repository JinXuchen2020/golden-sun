//! 全局常量 — 一处修改，全局生效
//!
//! 所有公称尺寸、魔数在此集中管理，禁止在模块中硬编码。

use macroquad::prelude::Color;

// ── 窗口 ──
/// 默认窗口逻辑宽度（像素）
pub const WINDOW_WIDTH: f32 = 640.0;
/// 默认窗口逻辑高度（像素）
pub const WINDOW_HEIGHT: f32 = 480.0;

/// delta 时间上界（秒），防止帧卡顿导致物理飞跃
pub const DELTA_MAX: f32 = 1.0 / 30.0;
/// delta 时间下界（秒），防止无 vsync 时 delta ≈ 0
pub const DELTA_MIN: f32 = 1.0 / 1000.0;

// ── 纹理 ──
/// Mode 7 渲染目标的像素宽度
pub const RENDER_TARGET_W: u32 = 640;
/// Mode 7 渲染目标的像素高度
pub const RENDER_TARGET_H: u32 = 480;
// GBA 风格纹理滤镜（像素游戏用 Nearest）
// 在 TextureCache 创建时自动设置 FilterMode::Nearest
// ── 地图 ──
/// 单个 tile 的边长（世界坐标单位 = 像素）
pub const TILE_SIZE: f32 = 32.0;
/// 世界坐标到 tile 索引的换算
pub const WORLD_TO_TILE: f32 = 1.0 / TILE_SIZE;
/// 地图尺寸（tile 数）
pub const MAP_WIDTH: u32 = 32;
pub const MAP_HEIGHT: u32 = 32;

// ── Mode 7 相机 ──
/// 默认相机高度
pub const CAMERA_DEFAULT_HEIGHT: f32 = 160.0;
/// 默认 VFOV（弧度）
pub const CAMERA_DEFAULT_FOV: f32 = std::f32::consts::FRAC_PI_4;
/// 地平线位置（屏幕高的比例，0=顶部）
pub const HORIZON_RATIO: f32 = 0.4;
/// 雾化最大距离（世界单位）
pub const FOG_END: f32 = 200.0;
/// 雾化倒数（除法→乘法优化）
pub const INV_FOG_END: f32 = 1.0 / FOG_END;
/// 雾化最小不透明度
pub const FOG_MIN_ALPHA: f32 = 0.3;
/// 天空渐变色（顶部）— 直接返回 macroquad Color，无需调用方转换
pub const SKY_COLOR_TOP: Color = Color { r: 0.471, g: 0.706, b: 1.0, a: 1.0 };
/// 天空渐变色（地平线）
pub const SKY_COLOR_HORIZON: Color = Color { r: 0.863, g: 0.941, b: 1.0, a: 1.0 };
/// 预乘 255 的天空颜色分量（避免每帧乘法）
pub const SKY_R_HORIZON: f32 = SKY_COLOR_HORIZON.r * 255.0;
pub const SKY_G_HORIZON: f32 = SKY_COLOR_HORIZON.g * 255.0;
pub const SKY_B_HORIZON: f32 = SKY_COLOR_HORIZON.b * 255.0;

// ── 绘制颜色 ──
/// 背景清除色（草地绿）
pub const BG_COLOR: Color = Color { r: 86.0/255.0, g: 130.0/255.0, b: 36.0/255.0, a: 1.0 };
/// 副标题文字色
pub const TITLE_TEXT_COLOR: Color = Color { r: 200.0/255.0, g: 220.0/255.0, b: 1.0, a: 1.0 };
pub const PLACEHOLDER_TEXT_COLOR: Color = Color { r: 1.0, g: 200.0/255.0, b: 100.0/255.0, a: 1.0 };
pub const DEBUG_TEXT_COLOR: Color = Color { r: 1.0, g: 1.0, b: 100.0/255.0, a: 180.0/255.0 };

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
/// 屏幕底部边距（像素）
pub const SCREEN_MARGIN_BOTTOM: f32 = 20.0;

// ── 精灵动画 ──
/// 精灵像素尺寸（程序化绘制）
pub const SPRITE_SIZE: u32 = 16;
/// 动画帧间隔（秒）
pub const ANIM_FRAME_DURATION: f32 = 1.0 / 8.0;

// ── NPC ──
/// NPC 交互距离（tile 单位）
pub const NPC_INTERACT_RANGE: f32 = 1.5;
/// NPC 巡逻到达阈值（dist² < 此值视为到达）
pub const NPC_PATROL_ARRIVE_SQ: f32 = 4.0;
/// NPC 巡逻路径点停留时间（秒）
pub const NPC_PATROL_PAUSE_DURATION: f32 = 2.0;

// ── 对话系统（Phase 4） ──
/// 打字机效果速度（字符/秒）
pub const DIALOGUE_CHAR_SPEED: f32 = 30.0;
/// 对话框 Y 坐标（顶部）
pub const DIALOGUE_BOX_Y: f32 = 310.0;
/// 对话框高度
pub const DIALOGUE_BOX_H: f32 = 110.0;
/// 对话文本 X 偏移
pub const DIALOGUE_TEXT_X: f32 = 20.0;
/// 对话文本 Y 偏移
pub const DIALOGUE_TEXT_Y: f32 = 350.0;
/// 对话文本字号
pub const DIALOGUE_TEXT_SIZE: f32 = 17.0;

// ── 精灵力 ──
/// 行走时 PP 恢复间隔（秒）
pub const PP_RECOVER_INTERVAL: f32 = 10.0;
/// 行走时 PP 恢复量
pub const PP_RECOVER_AMOUNT: u32 = 1;
/// 初始 PP 最大值
pub const PP_MAX: u32 = 25;
/// 初始 PP 值
pub const PP_INITIAL: u32 = 15;

