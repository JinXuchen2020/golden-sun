# BDD 测试规格目录

## 约定

- `.feature` 文件用 Gherkin 语法编写，作为可执行文档
- 对应的 `tests/*_bdd.rs` 包含测试实现
- **开发顺序**：先写 `.feature` → 写 `*_bdd.rs` → 写实现代码 → `cargo test` 全绿

## 文件映射

| Feature 文件 | 测试文件 | 实现 Phase | 状态 |
|-------------|---------|-----------|------|
| `tilekind.feature` | `tilekind_bdd.rs` | Phase 0 | ✅ 已实现 |
| `psynergy.feature` | `psynergy_bdd.rs` | Phase 3 | ⏳ 骨架 |
| `combat.feature` | `combat_bdd.rs` | Phase 5 | ⏳ 骨架 |
| `dialogue.feature` | `dialogue_bdd.rs` | Phase 4 | ⏳ 骨架 |
| `save.feature` | `save_bdd.rs` | Phase 6 | ⏳ 骨架 |

## BDD 工作流

```
Step 1: 写 .feature 文件（本目录）
  └── 方孔 — 产品/设计视角：什么时候做什么，预期什么结果

Step 2: 写 *_bdd.rs 测试骨架
  └── 圆孔 — 开发视角：翻译场景为 Rust 测试代码

Step 3: 写实现代码（src/ 目录）
  └── 运行 cargo test → 红灯 → 实现 → 绿灯

Step 4: 标记完成
  └── 更新本文件的状态列
```

## 示例

```gherkin
Scenario: 冻结水面
  Given tile (4, 8) 类型为 Water
  When 使用 Freeze 作用于 (4, 8)
  Then tile 变为 Ice
```

对应 Rust 测试：
```rust
#[test]
fn freeze_water_to_ice() {
    let mut map = Map::new();
    map.set(4, 8, TileKind::Water);
    apply_psynergy(&mut map, PsynergyType::Freeze, 4, 8);
    assert_eq!(map.get(4, 8), TileKind::Ice);
}
```
