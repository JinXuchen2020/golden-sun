# Phase 0: 项目骨架初始化（已完成）

## 目标
创建完整的 Rust + macroquad 项目骨架，确保能编译运行一个窗口。

## 已完成工作

### ✅ 0.1 Cargo.toml
- `macroquad = "0.4"` — 跨平台 2D/3D 游戏框架
- `serde = { version = "1", features = ["derive"] }` — 存档序列化
- `bincode = "1"` — 二进制序列化格式
- `glam = "0.29"` — 数学库（向量/矩阵操作）

### ✅ 0.2 模块目录结构
```
src/
├── lib.rs              # 库入口，re-export 所有模块
├── main.rs             # 入口 + 状态路由主循环
├── engine/
│   ├── mod.rs          # 核心类型：GameState / Camera / InputState / FrameTime
│   └── error.rs        # 统一错误类型：GameError + GameResult<T>
├── map/
│   └── mod.rs          # TileKind 枚举（14 种瓦片）
├── entity/
│   └── mod.rs          # 占位符（Phase 2）
├── psynergy/
│   └── mod.rs          # 占位符（Phase 3）
├── battle/
│   └── mod.rs          # 占位符（Phase 5）
├── scene/
│   └── mod.rs          # 占位符（Phase 4）
├── ui/
│   └── mod.rs          # 占位符（Phase 6）
└── audio/
    └── mod.rs          # 占位符（Phase 6）
```

### ✅ 0.3 main.rs
- `#[macroquad::main("Golden Sun - Rust Edition")]` 入口
- `GameCtx` 持有 GameState / Camera / InputState / FrameTime
- 状态路由：Title → WorldMap → Dialog/Battle/Menu/Psynergy/Transition
- Title 状态下按 Z/Enter 进入 WorldMap

### ✅ 0.4 核心类型（src/engine/mod.rs）
| 类型 | 说明 |
|------|------|
| `GameState` | Title / WorldMap / Dialog / Battle / Menu / Psynergy / Transition |
| `Camera` | x/y/z 坐标 + rotation + target_x/target_y（lerp 插值） |
| `InputState` | up/down/left/right/a/b/start/select |
| `FrameTime` | delta（帧时间差）+ elapsed（总时间） |

### ✅ 0.5 错误处理（src/engine/error.rs）
- `GameError` 枚举：MapParseError / AssetLoadError / SaveError / LogicError / IoError
- `GameResult<T>` 类型别名
- 实现了 `From<std::io::Error>`

## 可交付标准
每个 Phase 完成后，必须通过 **7 项可交付检查**才能进入下一 Phase。

详见 `.workbuddy/checklist/deliverable-template.md`，一键验证：`bash verify.sh`

**快速自检命令**:
```bash
cargo check && cargo test && cargo build --release && bash verify.sh
```

## 后续阶段引用规范
- 共享类型通过 `use golden_sun::engine::*` 或 `use crate::engine::*` 引入
- 统一错误通过 `use golden_sun::GameResult` 引入
- 新增子模块需在 `src/lib.rs` 中声明 `pub mod`，无需改 main.rs
- main.rs 中的 GameCtx 会随阶段扩展，新增字段即可

## 双端构建

```
桌面端 (Windows/macOS/Linux):
  cargo build --release && cargo run

网页端 (wasm32):
  cargo build --target wasm32-unknown-unknown --release --lib
  产物: target/wasm32-unknown-unknown/release/golden_sun_lib.wasm
  配合 HTML shell 即可部署到静态托管

提示词通用性:
  Phase 1-6 的 prompt 无需区分平台
  所有平台差异已被 macroquad + StorageBackend + TextureCache 封死

## 开发约定
- **Commit 纪律**: 每个 Phase 验收通过后立即 `git add -A && git commit -m "Phase N: <描述>"`，不攒代码
- **重构优先**: 每个 Phase 开始前先跑 `cargo check && cargo test` 确认基线干净
- **prompt 即规格**: prompt 文件中的验收标准是可执行的 checklist，不满足不提交
```
