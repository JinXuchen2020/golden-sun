# 黄金太阳 Rust 复刻 — 阶段化提示词工作空间

## 项目概览

用 **Rust + macroquad** 复刻 GBA《黄金太阳 (Golden Sun)》的伪 3D（Mode 7）世界地图体验。

| 维度 | 说明 |
|------|------|
| 语言 | Rust (edition 2024) |
| 框架 | macroquad 0.4 |
| 渲染 | 纯软件 Mode 7（逐行扫描透视投影） |
| 产物 | 原生 exe，双击可运行 |

## 提示词文件导航

提示词按开发阶段编号，**Agent 应按顺序逐份加载执行**：

| 文件 | 阶段 | 内容 | 依赖 |
|------|------|------|------|
| `prompts/00_project_setup.md` | Phase 0 | cargo 项目初始化、模块骨架、可运行窗口 | 无 |
| `prompts/01_mode7_world_map.md` | Phase 1 | Mode 7 伪 3D 渲染器、tilemap、相机控制、玩家移动 | 00 |
| `prompts/02_sprite_entity.md` | Phase 2 | 帧动画系统、NPC、A 键交互 | 01 |
| `prompts/03_psynergy.md` | Phase 3 | 7 种精灵力系统、地图交互解谜 | 02 |
| `prompts/04_dialogue.md` | Phase 4 | 对话树引擎、事件触发器、过场 | 02 |
| `prompts/05_battle.md` | Phase 5 | 回合制战斗系统、伤害计算、AI | 01 |
| `prompts/06_ui_audio.md` | Phase 6 | HUD、菜单、音频合成、存档、标题画面 | 00-05 |

## 使用方式

### Agent 自动执行

1. 加载目标阶段的提示词文件
2. 按文件中的"任务清单"逐项实现
3. 每项完成执行 `cargo check` 验证
4. 全部完成后执行 `cargo run` 验收
5. 标记该阶段任务为完成，进入下一阶段

### 手动执行

```bash
cd golden-sun
cargo run
```

## 项目结构

```
golden-sun/
├── Cargo.toml          # 依赖配置
├── src/
│   ├── main.rs          # 入口 + 主循环
│   ├── engine/          # 渲染/输入/相机
│   ├── map/             # tilemap + mode7
│   ├── entity/          # 玩家/NPC/精灵
│   ├── psynergy/        # 精灵力系统
│   ├── battle/          # 战斗系统
│   ├── scene/           # 对话/事件
│   ├── ui/              # HUD/菜单
│   ├── audio/           # 音频合成
│   └── data/            # 游戏数据
└── prompts/             # 阶段化提示词
```

## 构建

```bash
# 检查编译
cargo check

# 运行
cargo run
```

> 首次运行会自动从 crates.io 下载 macroquad 依赖，需要网络连接。
# golden-sun
