# 黄金太阳复刻 — 任务进度总表

> 此文件由 Agent 维护，每完成一个阶段更新状态。
>
> **可交付验证**: 每个 Phase 完成后运行 `bash verify.sh` + 对照 `.workbuddy/checklist/deliverable-template.md` 逐项检查，全部通过方可标记 ✅。

## 任务流水线

```
Phase 0 [项目骨架] → Phase 1 [Mode7世界地图] → Phase 2 [精灵与NPC] → Phase 4 [对话引擎]
                                                          ↓                     ↓
                                              Phase 3 [精灵力系统]        (触发战斗)
                                                          ↓                     ↓
                                              Phase 5 [战斗系统] ←─────────────┘
                                                          ↓
                                              Phase 6 [UI/音频/收尾/标题]
```

## 状态标记

- ⬜ **待开始** — 尚未执行
- 🔄 **进行中** — Agent 正在执行
- ✅ **已完成** — 验收通过
- ❌ **已阻塞** — 依赖未满足，暂停

## 阶段状态

| # | 阶段 | 状态 | 提示词文件 | 依赖 | 备注 |
|---|------|------|------------|------|------|
| 0 | 项目骨架初始化 | ✅ | `prompts/00_project_setup.md` | 无 | V6: 引擎模块拆分 + PsynergyType/Element预定义 + SceneId/SaveData/SceneRegistry |
| 1 | Mode 7 世界地图 | ✅ | `prompts/01_mode7_world_map.md` | Phase 0 | tilemap(32×32 Vale村)+mode7渲染+碰撞检测+玩家移动+菱形标记 |
| 2 | 精灵与 NPC | ✅ | `prompts/02_sprite_entity.md` | Phase 0 | 帧动画 + 3NPC + WalkPattern巡逻 + GPU纹理渲染 |
| 3 | 精灵力系统 | ✅ | `prompts/03_psynergy.md` | Phase 0 | 7种精灵力 + PP管理 + 选择UI + Force/Flash/Reveal |
| 4 | 对话引擎 | ✅ | `prompts/04_dialogue.md` | Phase 2 | DialogueState打字机 + StoryFlags + 3NPC脚本 |
| 5 | 战斗系统 | ✅ | `prompts/05_battle.md` | Phase 0, Phase 3 | 回合制战斗 + 元素克制 + AI + 随机遇敌 |
| 6 | UI/音频/收尾 | ⚠️ | `prompts/06_ui_audio.md` | Phase 1-5 | HUD + 菜单 + 存档 + 方波SFX + 场景过渡 (~60%) |

## 架构资产（Phase 0 优化 V6 — 最新）

| 资产 | 文件 | 用途 |
|------|------|------|
| 全局常量 | `src/engine/constants.rs` | 所有魔数集中管理（40+常量），SKY_COLOR 改用 Color 类型 |
| 统一 TileKind | `src/map/mod.rs` | 22 种 tile + color()→Color / is_walkable() / is_interactive() |
| 相机 | `src/engine/camera.rs` | Camera (拆分自 mod.rs) + tile/world 坐标转换 + lerp |
| Mode7 相机 | `src/engine/mode7_camera.rs` | 透视投影计算，隔离 2D tile 坐标与 3D 渲染空间 |
| 输入总线 | `src/engine/input.rs` | InputState + InputEvent + InputBus consume() 统一分发 |
| 状态机 | `src/engine/game_state.rs` | GameState (7态) + TransitionKind + timer |
| 渲染层序 | `src/engine/render_phase.rs` | RenderPhase 枚举 (8层) |
| 窗口配置 | `src/engine/window_config.rs` | WindowConfig — 一处改全局生效 |
| 帧时序 | `src/engine/frame_time.rs` | FrameTime + delta 裁剪保护 |
| 纹理缓存 | `src/engine/texture.rs` | TextureCache 复用 GPU 句柄 (update 替代 from_image) |
| 资源管理 | `src/engine/resources.rs` | ResourceManager 统一纹理/音频生命周期 |
| 存档后端 | `src/engine/storage.rs` | StorageBackend trait + FsStorage(%APPDATA%) + LocalStorage |
| 错误处理 | `src/engine/error.rs` | GameError + GameResult + From<&str>/From<String> |
| 精灵力类型 | `src/psynergy/mod.rs` | PsynergyType (7种) + Element (4种) 预定义，PP 消耗 |
| 场景管理 | `src/scene/mod.rs` | SceneId + SceneRegistry 场景切换骨架 |
| 存档模型 | `src/data/mod.rs` | SaveData 序列化契约（全 Phase 共享） |
| 实体系统 | `src/entity/mod.rs` | Entity 平铺结构 + EntityKind |
| 验证脚本 | `verify.sh` / `verify.ps1` | 双平台一键验证 (含 clippy) |

## 新架构亮点

| 特性 | 说明 |
|------|------|
| 🔀 并行阶段 | Phase 2(精灵) 不再依赖 Phase 1(Mode7)；Phase 5(战斗) 的 PsynergyType/Element 已预定义 |
| 🧩 模块化 | engine/ 拆分为 11 个独立文件，避免 mod.rs 膨胀 |
| 🛡️ 防御性 | 公共枚举加 #[non_exhaustive]；Camera 含 validate() 校验；GameError 有 From<&str> |
| 💾 数据契约 | SaveData 统一定义所有 Phase 的存档字段，避免各阶段自行拼凑 |
| 🪟 跨平台 | verify.ps1 支持 Windows 原生 PowerShell |
| 🔄 过渡系统 | GameState::Transition 含动画类型/计时/来源/目标字段 |

## 工作记录

| 日期 | 阶段 | 操作 | 结果 |
|------|------|------|------|
| 2026-06-14 | Phase 0 | V1: 骨架初始化 | 模块目录、共享类型、错误处理、依赖补齐 |
| 2026-06-14 | Phase 0 | V2: 深度架构优化（9项） | TileKind统一、坐标系统、InputBus、ResourceManager、constants、WindowConfig、RenderPhase、测试框架、全部prompt同步更新 |
| 2026-06-14 | Phase 0 | V3: 性能基底 | TextureCache纹理池（复用GPU纹理句柄）、FrameTime delta裁剪保护、常量RENDER_TARGET_W/H、纹理滤镜Nearest |
| 2026-06-14 | Phase 0 | V5: 双端支持 | StorageBackend trait + FsStorage(desktop) + LocalStorage(wasm32)、web-sys条件依赖、Cargo.toml双target配置 |
| 2026-06-14 | Phase 0 | V6: 终极架构优化（9项） | 引擎模块拆分(11文件)、Mode7Camera、PsynergyType/Element预定义、SceneId/SaveData骨架、TileKind::color()→Color、SKY_COLOR→Color、TextureCache::update()、verify.ps1、Default impls、#[non_exhaustive]、GameError From&lt;str&gt; |
