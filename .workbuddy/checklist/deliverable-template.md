# Phase N 可交付标准检查清单

> 每个 Phase 完成后，必须逐项通过以下检查方可标记 ✅ 进入下一 Phase。

## 1. 编译与代码质量

- [ ] `cargo check` 零警告
- [ ] `cargo clippy` 零警告（如有 clippy toolchain）
- [ ] `cargo fmt --check` 格式一致（如有 rustfmt）
- [ ] 无 `unwrap()` 裸调用（除显式标记 `#[allow]` 的场景外）
- [ ] 无注释掉的代码块（所有 TODO 有对应 Phase 编号）

## 2. 测试覆盖

- [ ] `cargo test` 全部通过（0 failed, 0 ignored）
- [ ] 本 Phase 相关的 `.feature` 场景全部有对应 `#[test]`
- [ ] 边界情况至少覆盖 2 个（空/越界/最小值/最大值）
- [ ] 新增测试数 ≥ 本 Phase BDD 场景数

## 3. 可运行性

- [ ] `cargo build --release` 成功
- [ ] `cargo run --release` 弹出窗口无 crash
- [ ] 窗口标题显示 "Golden Sun - Rust Edition"
- [ ] 目标帧率：≥ 55fps（release 模式，当前 Phase 功能全开时）
- [ ] 连续运行 30 秒无 panic / 内存异常增长

## 4. 功能完整性

- [ ] prompt 中全部验收标准 checkbox 已勾选
- [ ] 新功能在 `main.rs` 的 `update()` / `draw()` 分支中正确集成
- [ ] 未破坏前一 Phase 的功能（回归验证）
- [ ] 功能开关：新功能可通过 GameState 进入/退出（不强制常驻）

## 5. 架构一致性

- [ ] 新增模块在 `src/lib.rs` 中 `pub mod` 声明
- [ ] 使用的常量全部来自 `src/engine/constants.rs`（无硬编码魔数）
- [ ] 输入通过 `InputBus::consume()` 分发（无直接读 `InputState` 按键）
- [ ] 纹理/资源通过 `ResourceManager` 管理
- [ ] 坐标转换使用 `Camera::tile_to_world()` / `world_to_tile()`

## 6. 文档同步

- [ ] `tasks.md` 中本节状态更新为 ✅
- [ ] `tasks.md` 工作记录新增一行
- [ ] `tests/features/README.md` 状态列更新
- [ ] `.workbuddy/memory/YYYY-MM-DD.md` 记录本 Phase 关键决策
- [ ] 如涉及项目约定变更，更新 `MEMORY.md`

## 7. Git 提交

- [ ] `git status` 无待提交的临时文件 / debug 输出
- [ ] Commit message 格式: `Phase N: <简短描述>`
- [ ] 一个 Phase 一个 commit（每个功能完成后立即 check-in，不攒到最后）
- [ ] `git push` 成功（如有 remote）
- [ ] ⚡ **强制规则**: verify.sh 全部 PASS 后立即提交，否则视为 Phase 未完成

---

## 快速自检命令

```bash
# 一键运行全部自动检查
cargo check && cargo test && cargo clippy 2>/dev/null; echo "---"; cargo run --release &
```

## 阻塞/豁免说明

| 检查项 | 何时可豁免 |
|--------|-----------|
| `clippy` | 未安装 clippy toolchain 时 |
| `rustfmt` | Phase 0 未配置 `.rustfmt.toml` 前 |
| 回归验证 | Phase 0（无前序 Phase） |
| Git push | 无 remote 或网络不可用时 |
