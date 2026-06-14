# 黄金太阳 Rust 复刻 — Golden Sun GBA Recreation

用 **Rust + macroquad** 复刻 GBA《黄金太阳》的伪 3D（Mode 7）世界地图与冒险体验。

## 快照

| 维度 | 说明 |
|------|------|
| 语言 | Rust (edition 2024), MSRV 1.85 |
| 框架 | macroquad 0.4 |
| 渲染 | 纯软件 Mode 7（逐行扫描透视投影），每帧经 TextureCache `update()` 上传 GPU |
| 平台 | **桌面端** (Windows/macOS/Linux) + **网页端** (wasm32) |
| 状态 | Phase 0 基建完成 ✅，Phase 1 Mode 7 待开发 |
| 测试 | 82 个 (19 unit + 25 core + 38 tilekind BDD + 2 ignore) |

## 快速开始

```bash
# 桌面端
cargo build --release && cargo run

# 网页端（需要 wasm32 target）
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release --lib
# 产物: target/wasm32-unknown-unknown/release/golden_sun_lib.wasm
```

## 可交付验证

```bash
# Linux/macOS
bash verify.sh

# Windows
.\verify.ps1
```

单个命令自动检查 6 项：Build + Lint、魔数扫描、unwrap 检查、测试、架构完整性、release 构建。

## 阶段化开发 (Phase 0-6)

提示词逐阶段驱动 Agent 开发：

```
Phase 0 [项目骨架] → Phase 1 [Mode7地图] → Phase 2 [精灵NPC] → Phase 4 [对话引擎]
                                                        ↓                     ↓
                                            Phase 3 [精灵力系统]          (触发战斗)
                                                        ↓                     ↓
                                            Phase 5 [战斗系统] ←───────────────┘
                                                        ↓
                                            Phase 6 [UI/音频/标题]
```

| # | 阶段 | 状态 | prompt |
|---|------|------|--------|
| 0 | 项目骨架初始化 | ✅ | `prompts/00_project_setup.md` |
| 1 | Mode 7 世界地图 | ⬜ | `prompts/01_mode7_world_map.md` |
| 2 | 精灵与 NPC | ⬜ | `prompts/02_sprite_entity.md` |
| 3 | 精灵力系统 | ⬜ | `prompts/03_psynergy.md` |
| 4 | 对话引擎 | ⬜ | `prompts/04_dialogue.md` |
| 5 | 战斗系统 | ⬜ | `prompts/05_battle.md` |
| 6 | UI/音频/收尾 | ⬜ | `prompts/06_ui_audio.md` |

## 项目结构

```
golden-sun/
├── Cargo.toml                # 依赖 (macroquad/serde) + LTO + unsafe_code deny
├── Cargo.lock
├── verify.sh                 # Linux/macOS 一键可交付验证
├── verify.ps1                # Windows 一键可交付验证
├── tasks.md                  # 全局任务进度表
├── src/
│   ├── lib.rs                # 库入口 9 模块 + re-export + deny(unsafe_code)
│   ├── main.rs               # GameCtx + 状态路由 + SceneRegistry 交换机
│   ├── engine/
│   │   ├── mod.rs            # 模块声明 + 旧项 re-export
│   │   ├── constants.rs      # 全局常量 (50+ Color 直接输出)
│   │   ├── error.rs          # GameError + GameResult<T>
│   │   ├── camera.rs         # Camera (tile/world 转换, lerp, validate)
│   │   ├── mode7_camera.rs   # Mode7Camera + ScanlineContext (投影 + 雾化)
│   │   ├── game_state.rs     # GameState 7 变体 + TransitionKind
│   │   ├── frame_time.rs     # FrameTime (delta 裁剪保护)
│   │   ├── render_phase.rs   # RenderPhase 枚举
│   │   ├── window_config.rs  # WindowConfig
│   │   ├── input.rs          # InputState + InputBus + InputEvent
│   │   ├── resources.rs      # ResourceManager (泛型纹理/音频)
│   │   ├── storage.rs        # StorageBackend trait (FsStorage / LocalStorage)
│   │   └── texture.rs        # TextureCache (update 复用 + Nearest 滤镜)
│   ├── map/                  # TileKind 22 种 (world_to_tile_index / tile_center)
│   ├── entity/               # Entity 平铺字段 (Phase 2)
│   ├── psynergy/             # PsynergyType 7 种 + Element 4 种 (Phase 3)
│   ├── data/                 # SaveData (Serialize, Phase 6)
│   ├── scene/                # SceneId + SceneRegistry (Phase 4)
│   ├── battle/               # (Phase 5)
│   ├── ui/                   # (Phase 6)
│   └── audio/                # (Phase 6)
├── tests/
│   ├── features/             # BDD Gherkin 规格文件
│   ├── core.rs               # 基础单元测试 (25)
│   ├── tilekind_bdd.rs       # 瓦片 BDD 测试 (38)
│   ├── psynergy_bdd.rs       # (Phase 3 骨架)
│   ├── combat_bdd.rs         # (Phase 5 骨架)
│   ├── dialogue_bdd.rs       # (Phase 4 骨架)
│   └── save_bdd.rs           # (Phase 6 骨架)
├── prompts/                  # 7 个阶段化 Agent 提示词
└── .workbuddy/
    ├── memory/               # Agent 工作记忆
    └── checklist/            # 可交付检查 + 自动修复指南
```

## Phase 0 基建里程碑

| 轮次 | 修复数 | 关键变更 |
|------|--------|----------|
| R1 骨架 | 12 | TextureCache `update()`, TileKind `color()`→`Color`, 测试 6→23, PsynergyType/Element 骨架 |
| R2 模块 | 11 | 模块拆分 1→11, SceneRegistry, SaveData, GameError, `#[non_exhaustive]`, verify.ps1 |
| R3 质量 | 14 | 坐标重命名, Camera z→height, `project()`→`Option`, Debug derives, LTO, `rem_euclid` |
| R4 测试 | 11 | verify.ps1 3 bug 修复, `crate-type="lib"`, 测试 66→82, 颜色常量, `#[must_use]` |
| R5 安全 | 3 | `#[must_use]` x27, `const fn` x6, verify.ps1 clippy guard |
| R6 收官 | 6 | `unsafe_code deny`, lib.rs lints, `pub(crate)` 约束, `Copy` derives, `_` 前缀统一 |

**终态**: 82 tests, 0 警告, 6/6 verify, zero unsafe/panic/unwrap, zero `#[allow(clippy::*)]`。

## 开发约定

- **每 Phase 一 commit**: `verify # PASS → git add -A && git commit -m "Phase N: <desc>"`
- **prompt 即规格**: 验收标准是可执行的 checklist
- **BDD 驱动**: 先写 `.feature` → 写 `_bdd.rs` → 写实现 → `cargo test` 全绿
- **颜色全常量**: 禁止原始 RGBA 元组，全部通过 `constants.rs` 的 `Color` 常量引用
- **坐标规范**: Camera 用 tile 单位 (x/y), 世界像素用 `tile_to_world()`; map 函数用 `world_to_tile_index()` / `tile_center()`
- **输入统一**: 永远通过 `InputBus::consume()` / `has()` 访问输入，禁止直接读 macroquad `is_key_*`
