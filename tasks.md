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
| 0 | 项目骨架初始化 | ✅ | `prompts/00_project_setup.md` | 无 | Phase 0 V5: +StorageBackend 双端 +wasm |
| 1 | Mode 7 世界地图 | ⬜ | `prompts/01_mode7_world_map.md` | Phase 0 | MVP：能走动的 Vale 村 |
| 2 | 精灵与 NPC | ⬜ | `prompts/02_sprite_entity.md` | Phase 1 | 帧动画 + NPC 交互 |
| 3 | 精灵力系统 | ⬜ | `prompts/03_psynergy.md` | Phase 2 | 7 种精灵力 |
| 4 | 对话引擎 | ⬜ | `prompts/04_dialogue.md` | Phase 2 | 对话树 + 事件 |
| 5 | 战斗系统 | ⬜ | `prompts/05_battle.md` | Phase 1, Phase 3 | 回合制战斗（需Phase3的PsynergyType+Element） |
| 6 | UI/音频/收尾 | ⬜ | `prompts/06_ui_audio.md` | Phase 1-5 | HUD + 存档 + 标题 + GBA音频 + 完整流程 |

## 架构资产（Phase 0 V2 已完成）

| 资产 | 文件 | 用途 |
|------|------|------|
| 全局常量 | `src/engine/constants.rs` | 所有魔数集中管理（40+常量） |
| 统一 TileKind | `src/map/mod.rs` | 17 种 tile + color()/is_walkable()/is_interactive() |
| 坐标系统 | `src/engine/mod.rs` | Camera::tile_to_world() / world_to_tile() / tile_index() / world_pos() |
| 输入总线 | `src/engine/input.rs` | InputEvent 枚举 + InputBus consume() 分发 |
| 资源管理 | `src/engine/resources.rs` | ResourceManager 统一纹理/音频生命周期 |
| 窗口配置 | `src/engine/mod.rs` | WindowConfig — 一处改全局生效 |
| 渲染层序 | `src/engine/mod.rs` | RenderPhase 枚举定义各层绘制顺序 |
| 测试框架 | `tests/core.rs` | 6 个单元测试（坐标、TileKind、碰撞检测） |

## 工作记录

| 日期 | 阶段 | 操作 | 结果 |
|------|------|------|------|
| 2026-06-14 | Phase 0 | V1: 骨架初始化 | 模块目录、共享类型、错误处理、依赖补齐 |
| 2026-06-14 | Phase 0 | V2: 深度架构优化（9项） | TileKind统一、坐标系统、InputBus、ResourceManager、constants、WindowConfig、RenderPhase、测试框架、全部prompt同步更新 |
| 2026-06-14 | Phase 0 | V3: 性能基底 | TextureCache纹理池（复用GPU纹理句柄）、FrameTime delta裁剪保护（1ms~33ms）、常量RENDER_TARGET_W/H、纹理滤镜Nearest |
| 2026-06-14 | Phase 0 | V5: 双端支持 | StorageBackend trait + FsStorage(desktop) + LocalStorage(wasm32)、web-sys条件依赖、Cargo.toml双target配置 |
