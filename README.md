# 黄金太阳 Rust 复刻 — Golden Sun GBA Recreation

用 **Rust + macroquad** 复刻 GBA《黄金太阳》的伪 3D（Mode 7）世界地图与冒险体验。

## 快照

| 维度 | 说明 |
|------|------|
| 语言 | Rust (edition 2024) |
| 框架 | macroquad 0.4 |
| 渲染 | 纯软件 Mode 7（逐行扫描透视投影），每帧经 TextureCache 上传 GPU |
| 平台 | **桌面端** (Windows/macOS/Linux) + **网页端** (wasm32) |
| 状态 | Phase 0 基建完成 ✅，Phase 1 Mode 7 待开发 |
| 测试 | 47 个 (BDD Gherkin 规格驱动) |

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
bash verify.sh
```

单个命令自动检查：编译质量、魔数扫描、测试覆盖、架构完整性、release 构建。

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
├── Cargo.toml                # 依赖 (macroquad/serde/bincode/glam) + wasm32 target
├── verify.sh                 # 一键可交付验证脚本
├── tasks.md                  # 全局任务进度表
├── src/
│   ├── lib.rs                # 库入口，声明 9 个模块
│   ├── main.rs               # GameCtx + 状态路由主循环
│   ├── engine/
│   │   ├── mod.rs            # Camera / GameState / InputState / FrameTime / RenderPhase
│   │   ├── constants.rs      # 全局常量 (40+)
│   │   ├── error.rs          # GameError + GameResult<T>
│   │   ├── input.rs          # InputEvent 枚举 + InputBus 分发
│   │   ├── resources.rs      # ResourceManager (纹理/音频生命周期)
│   │   ├── storage.rs        # StorageBackend trait (桌面FsStorage / 网页LocalStorage)
│   │   └── texture.rs        # TextureCache (GPU 纹理复用 + Nearest 滤镜)
│   ├── map/                  # TileKind 22 种
│   ├── entity/               # Entity 平铺字段设计 (ECS 预留)
│   ├── psynergy/             # (Phase 3)
│   ├── battle/               # (Phase 5)
│   ├── scene/                # (Phase 4)
│   ├── ui/                   # (Phase 6)
│   └── audio/                # (Phase 6)
├── tests/
│   ├── features/             # BDD Gherkin 规格文件 (tilekind/psynergy/combat/dialogue/save)
│   ├── core.rs               # 基础单元测试 (6)
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

| 迭代 | 成果 |
|------|------|
| V1 骨架 | 模块目录、共享类型、错误处理、依赖补齐 |
| V2 架构 | TileKind 统一、坐标系统、InputBus、ResourceManager、constants、BDD |
| V3 验证 | verify.sh、fix-workflow、可交付模板 |
| V4 性能 | TextureCache、delta 裁剪、Nearest 滤镜 |
| V5 双端 | StorageBackend trait、wasm 目标配置 |
| V6 ECS预留 | Entity 平铺字段、prompt 约束 |

## 开发约定

- **每 Phase 一 commit**: `verify.sh PASS → git add -A && git commit -m "Phase N: <desc>"`
- **prompt 即规格**: 验收标准是可执行的 checklist
- **BDD 驱动**: 先写 `.feature` → 写 `_bdd.rs` → 写实现 → `cargo test` 全绿
