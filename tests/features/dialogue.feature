Feature: 对话树与事件系统
  作为玩家，我希望与 NPC 对话时体验到黄金太阳风格的对话系统，
  包括打字机效果、分支选择和剧情触发。

  # ── 对话树 ──
  Scenario: 线性对话流程
    Given 对话节点 "intro" 的内容为 "欢迎来到Vale村"
    And 该节点没有分支选项
    And 结束时触发 SetFlag("met_elder", true)
    When 从 "intro" 开始对话
    And 逐字显示完毕后按 A 键
    Then 设置 flag "met_elder" 为 true

  Scenario: 分支对话
    Given 对话节点 "choice1":
      | 选项文本    | 下一节点   |
      | "接受任务" | "accept"   |
      | "拒绝任务" | "decline"  |
    When 选择 "接受任务"
    Then 跳转到节点 "accept"

  Scenario: 条件分支
    Given 对话节点 "elder_talk" 有选项:
      | 选项文本      | 下一节点    | 条件               |
      | "关于任务"    | "about_quest" | flag "met_elder" = true |
      | "你是谁"      | "who_are_you" | 无条件                 |
    When flag "met_elder" 为 true
    Then "关于任务" 选项可见
    And "你是谁" 选项可见

  # ── 打字机效果 ──
  Scenario: 逐字显示
    Given 对话文本为 "Hello"
    And 打字机间隔为 0.05 秒
    When 开始播放
    Then 0 秒时显示 ""
    Then 0.05 秒时显示 "H"
    Then 0.20 秒时显示 "Hell"
    Then 0.25 秒时显示 "Hello"
    And 文本完整后停止计时

  Scenario: A 键加速显示
    Given 对话正在播放中, 已显示 3 个字符
    When 按 A 键
    Then 立即显示全部文本

  # ── 对话动作 ──
  Scenario Outline: 对话结束触发动作
    Given 对话节点结束时执行 <action>
    When 对话结束
    Then <effect>

    Examples:
      | action                          | effect                      |
      | GiveItem("apple", 1)           | 道具列表新增 "apple" ×1      |
      | SetFlag("quest_active", true)  | flag "quest_active" 为 true  |
      | UnlockPsynergy(Whirlwind)      | 获得精灵力 Whirlwind         |
      | Teleport(20, 15)               | 玩家坐标变为 (20, 15)        |
      | StartBattle("slime_x2")        | 状态切换为 Battle            |

  # ── 事件触发器 ──
  Scenario: 踩到事件瓦片触发
    Given 事件 OnTile(8, 8) 关联对话 "hidden_scene"
    When 玩家移动到 (8, 8)
    Then 自动触发对话 "hidden_scene"

  Scenario: 区域触发
    Given 事件 OnEnterRegion(5, 5, 8, 8) 关联 SetFlag("entered_zone")
    When 玩家进入 (6, 6)
    Then flag "entered_zone" 设置为 true

  Scenario: 一次性事件
    Given 事件 "mob_spawn" 是一次性触发
    When 第一次触发
    Then 执行事件关联的 StartBattle
    When 第二次走到触发点
    Then 不触发

  # ── Flag 系统 ──
  Scenario: Flag 持久化
    Given FlagManager 中有 {"has_sword": false, "has_shield": true}
    When 设置 "has_sword" = true
    Then "has_sword" 为 true
    And "has_shield" 仍为 true
    And 不存在的 flag 默认为 false
