# Golden Sun 项目长期记录

## 项目约定
- 每个 Phase 完成 → `git add -A && git commit -m "Phase N: <描述>"`，verify.sh PASS 后立即提交
- 所有常量集中在 `src/engine/constants.rs`，禁止在各 Phase 模块中硬编码魔数
- 坐标系统：tile 单位用 `Camera.x/y`，世界像素用 `Camera::tile_to_world()` 转换
- TileKind 在 `src/map/mod.rs` 统一定义（当前 22 种），各 Phase 只能使用不能重复声明
- 输入通过 `InputBus::consume()` 分发，禁止直接读 `InputState` 按键（持续按下状态除外）
- 纹理/音频通过 `ResourceManager` 统一管理，标记为 `procedural` 的纹理在存档恢复时重建
- `cargo check` 必须零警告后提交，测试通过方可进入下一 Phase

## 架构约束
- Phase 1: 模式7渲染 + 玩家移动（使用 Camera + TileKind 现有方法）
- Phase 2: 精灵动画 + NPC（使用 ResourceManager 注册纹理）
- Phase 3: 精灵力系统（扩展 TileKind 交互 tile）
- Phase 4: 对话引擎（依赖 Phase 2）
- Phase 5: 战斗系统（依赖 Phase 1 + Phase 3 的 PsynergyType/Element）
- Phase 6: UI/音频/归档（依赖 1-5 全部）

## 测试约定 (BDD)
- `.feature` 文件在 `tests/features/` — 用 Gherkin 语法定义功能规格
- 实现文件 `tests/*_bdd.rs` — 每个 Scenario 对应一个 `#[test] fn`
- 开发顺序: `.feature` → `*_bdd.rs` 骨架 → 实现代码 → `cargo test` 全绿
- 当前已实现: `tilekind_bdd.rs` (38 测试), `core.rs` (6 测试)
- 待实现骨架: `psynergy_bdd.rs` (Phase 3), `combat_bdd.rs` (Phase 5), `dialogue_bdd.rs` (Phase 4), `save_bdd.rs` (Phase 6)

## Phase 7 扩展范围（2026-06-19 制定）
- **7.1 主线剧情**：NPC 对话丰富（Ivan/Mia/Garsmin 各扩至 4-7 页，新增 Garet 3 页）、10 任务主线链、故事驱动逻辑
- **7.2 新场景**：Sol Sanctum 16×16 室内迷宫、MythrilGolem Boss、场景出口/互连映射
- **7.3 装备系统**：11 件装备（武/防/饰三槽）、商店 UI、Ivan 对话触发
- **7.4 Boss 机制**：特殊行动 AI、阶段切换（50% 血怒气）、双倍奖励
- **7.5 召唤系统**：9 个召唤（4 元素各 2-3 个）、Djinn Standby 消耗机制
- **7.6 升级特效**：LevelUp 状态机动画、战斗统计与评级面板
- **7.7 NPC 丰富**：好感度增长、重复对话问候、世界氛围闲聊
- **7.8 测试验证**：对话/场景/装备/召唤/Boss 单元测试

Agent 提示词位置: `prompts/07_story_content_master.md`

## 构建
- 桌面: `cargo build --release && cargo run`
- 网页: `cargo build --target wasm32-unknown-unknown --release --lib` → `golden_sun_lib.wasm`

## 依赖版本
- Rust edition 2024
- macroquad 0.4; serde 1 + derive; bincode 1; glam 0.29
- crates.io 镜像: rsproxy.cn (sparse 协议)
