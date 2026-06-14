# Phase 6: UI / 音频 / 收尾打磨

## 目标
完成 HUD、菜单系统、GBA 风格音频合成、存档、标题画面，打通完整游戏流程。

## 共享类型引用（来自 Phase 0）
```rust
use golden_sun::engine::{GameState, InputState, FrameTime, Camera, WindowConfig};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::resources::{ResourceManager, AudioSample, TextureData};
use golden_sun::engine::storage::StorageBackend;  // ← V5 新增
use golden_sun::engine::constants::{
    AUDIO_SAMPLE_RATE, SFX_CONFIRM_FREQ, SFX_CANCEL_FREQ,
    SFX_CONFIRM_MS, SFX_CANCEL_MS, WINDOW_WIDTH, WINDOW_HEIGHT,
};
use golden_sun::psynergy::types::PsynergyType;      // Phase 3
use golden_sun::battle::state::BattleUnit;           // Phase 5
use golden_sun::GameResult;
```
- 存档: 通过 `StorageBackend` trait 抽象，桌面端 `FsStorage`（文件系统），网页端 `LocalStorage`（浏览器 localStorage）
- 音频: `ResourceManager::store_audio()` 统一管理音频样本
- 纹理: `ResourceManager::get_texture()` 获取已注册纹理，`TextureCache` 管理 GPU 纹理
- `WindowConfig` — 窗口参数统一入口
- 纹理: `ResourceManager::get_texture()` 获取已注册纹理
- `WindowConfig` — 窗口参数统一入口

## 前置依赖
- Phase 1-5 全部完成

## 任务清单

### 6.1 游戏内 HUD
文件: `src/ui/hud.rs`
- 左上角 HP/PP 像素条
- 右上角精灵数
- 底部位置名称提示

### 6.2 像素字体渲染
文件: `src/ui/font.rs`
```rust
pub struct PixelFont {
    pub glyphs: HashMap<char, [[u8; 8]; 8]>,  // 8x8 位图
}
```
- A-Z, 0-9, 基本标点
- 用于对话/菜单/HUD 所有文字
- 注册到 `ResourceManager` 作为程序化纹理

### 6.3 主菜单系统
文件: `src/ui/menu.rs`
```
主菜单: 新游戏 / 继续 / 设置
暂停菜单: 继续 / 道具 / 精灵 / 状态 / 存档 / 设置 / 返回标题
```
- 使用 `InputBus` 消费方向 + Confirm/Cancel

### 6.4 道具 + 状态界面
- `Inventory { items, gold }`
- 状态显示: 等级/HP/PP/攻/防/速/精灵力

### 6.5 存档系统
```rust
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub player_pos: (f32, f32),
    pub player_rotation: f32,
    pub flags: HashMap<String, bool>,
    pub inventory: Vec<(String, u32)>,
    pub gold: u32,
    pub psynergies: Vec<PsynergyType>,
    pub party: Vec<BattleUnit>,
}
```
- 文件: `save.dat`，使用 bincode 序列化
- 读档后重建 `ResourceManager` 中的程序化纹理

### 6.6 音频系统
文件: `src/audio/synth.rs`

使用 `AUDIO_SAMPLE_RATE(44100Hz)` 生成方波样本：
```
BGM: Vale 村主题、战斗 BGM（4 小节循环）
SFX:
  - 确认音: SFX_CONFIRM_FREQ(440Hz) 方波, SFX_CONFIRM_MS(100ms)
  - 取消音: SFX_CANCEL_FREQ(220Hz) 方波, SFX_CANCEL_MS(80ms)
  - 受伤/升级/施法音效
```
- 预生成 → `ResourceManager::store_audio()`

### 6.7 标题画面 + 场景过渡
- GBA 风格标题: 大像素字 + Mode7 背景
- 过渡: 闪光/淡入淡出

### 6.8 完整游戏流程闭环
标题 → Vale村 → 村长对话 → 获取精灵力 → 解谜 → 野外 → 遇敌 → 战斗 → 存档 → 读档

## 验收标准
- [ ] `cargo test` 全部通过
- [ ] HUD 正确显示 HP/PP/精灵数
- [ ] 像素字体渲染正常
- [ ] 菜单系统完整可用
- [ ] 存档/读档（bincode + serde）正常
- [ ] GBA 风格音效/BGM 合成正确
- [ ] 标题画面显示
- [ ] 完整游戏闭环可跑通
