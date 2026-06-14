Feature: 存档与读档系统
  作为玩家，我希望能够保存和恢复游戏进度，
  以便在退出后能继续上次的冒险。

  # ── 存档 ──
  Scenario: 保存完整的游戏状态
    Given 玩家坐标为 (15.5, 22.3), 朝向 1.57rad
    And Flag 中有 {"met_elder": true, "has_apple": true}
    And 道具栏有 [("apple", 3), ("sword", 1)]
    And 已解锁精灵力 [Whirlwind, Growth]
    And 金币为 500
    When 执行存档到 "save.dat"
    Then 文件 "save.dat" 被创建
    And 序列化的数据与游戏状态一致

  Scenario: 覆盖已有存档
    Given 已有存档 "save.dat"（旧数据）
    When 执行新存档
    Then "save.dat" 被新数据覆盖
    And 新数据与当前游戏状态一致

  # ── 读档 ──
  Scenario: 读取存档恢复游戏
    Given 文件 "save.dat" 存在，包含:
      | 玩家坐标 | (15.5, 22.3) |
      | 朝向     | 1.57 rad    |
      | Flag     | met_elder=true |
      | 道具     | apple×3, sword×1 |
      | 精灵力   | Whirlwind, Growth |
      | 金币     | 500 |
    When 执行读档
    Then 游戏状态恢复为存档中的值
    And 相机位置设置为 (15.5, 22.3)
    And Flag "met_elder" 为 true
    And 道具栏匹配

  Scenario: 读档后程序化纹理重建
    Given 存档中的精灵力列表为 [Whirlwind, Growth]
    When 读档完成
    Then 精灵力图标纹理已重新生成
    And 重新生成的纹理与原始纹理内容相同

  # ── 边界情况 ──
  Scenario: 空存档槽
    Given 文件 "save.dat" 不存在
    When 尝试读档
    Then 返回 SaveError
    And 提示 "没有存档"

  Scenario: 存档损坏
    Given 文件 "save.dat" 内容无效
    When 尝试读档
    Then 返回 SaveError
    And 提示 "存档损坏"

  # ── 完整闭环 ──
  Scenario: 存档→退出→读档→继续游戏
    Given 玩家在 (10, 10), 有精灵力 [Whirlwind]
    When 存档
    And 程序退出
    And 程序重新启动
    And 读档
    Then 玩家回到 (10, 10)
    And 精灵力 Whirlwind 可用
