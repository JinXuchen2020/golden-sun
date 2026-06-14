Feature: 精灵力（Psynergy）地图交互系统
  作为玩家，我希望使用黄金太阳的 7 种精灵力与环境互动，
  以便解开地图上的谜题和障碍。

  Background:
    Given 一张 32x32 的地图
    And 玩家坐标为 (10, 10)

  # ── Whirlwind 旋风 ──
  Scenario Outline: 旋风清除障碍
    Given tile (<tx>, <ty>) 类型为 <input_tile>
    When 使用 Whirlwind 作用于 (<tx>, <ty>)
    Then tile 变为 <output_tile>
    And 地图总计修改了 1 格

    Examples:
      | tx | ty | input_tile | output_tile |
      | 5  | 5  | Vine       | Grass       |
      | 8  | 3  | Windmill   | WindmillActive |

  Scenario: 旋风对普通瓦片无效果
    Given tile (7, 7) 类型为 Grass
    When 使用 Whirlwind 作用于 (7, 7)
    Then tile 仍为 Grass

  # ── Growth 生长 ──
  Scenario: 生长触发种子
    Given tile (6, 6) 类型为 Seed
    When 使用 Growth 作用于 (6, 6)
    Then tile 变为 VineClimbable
    And VineClimbable 是可通行的

  Scenario: 生长对非种子无效果
    Given tile (6, 6) 类型为 Grass
    When 使用 Growth 作用于 (6, 6)
    Then tile 仍为 Grass

  # ── Freeze 冻结 ──
  Scenario: 冻结水面
    Given tile (4, 8) 类型为 Water
    When 使用 Freeze 作用于 (4, 8)
    Then tile 变为 Ice
    And Ice 是可通行的

  Scenario: 冻结非水面无效果
    Given tile (4, 8) 类型为 Grass
    When 使用 Freeze 作用于 (4, 8)
    Then tile 仍为 Grass

  # ── Force 推方块 ──
  Scenario: 推方块沿朝向移动一格
    Given tile (5, 5) 类型为 PushBlock
    And tile (5, 6) 类型为 Grass
    And 玩家面朝南方
    When 使用 Force 作用于 (5, 5)
    Then (5, 5) 变为 Grass
    And (5, 6) 变为 PushBlock

  Scenario: 推方块被阻挡则无效
    Given tile (5, 5) 类型为 PushBlock
    And tile (5, 6) 类型为 Wall
    And 玩家面朝南方
    When 使用 Force 作用于 (5, 5)
    Then (5, 5) 仍为 PushBlock
    And (5, 6) 仍为 Wall

  Scenario: 推动距离限制
    Given tile (3, 3) 类型为 PushBlock
    And 玩家坐标为 (10, 10)
    When 使用 Force 作用于 (3, 3)
    Then 因为距离太远，操作无效

  # ── Catch 抓取 ──
  Scenario: 抓取远处宝箱
    Given tile (2, 8) 类型为 HiddenChest
    And 玩家面向该方向
    When 使用 Catch 作用于 (2, 8)
    Then tile 变为 OpenedChest

  # ── Flash 照亮 ──
  Scenario: 照亮暗区
    Given tile (5, 5) 类型为 DarkArea
    When 使用 Flash 作用于 (5, 5)
    Then (5, 5) 及周围 3x3 的 DarkArea 变为 Grass
    And 影响范围正好是 3x3

  # ── Reveal 透视 ──
  Scenario: 透视发现隐藏宝箱
    Given tile (7, 7) 类型为 HiddenChest
    And HiddenChest 初始不可见
    When 使用 Reveal 作用于 (7, 7)
    Then tile 仍为 HiddenChest
    And 地图标记显示此处有隐藏物

  # ── PP 管理 ──
  Scenario Outline: 精灵力消耗 PP
    Given 当前 PP 值为 <current_pp>
    And <psynergy> 消耗 <cost> PP
    When 尝试使用 <psynergy>
    Then 结果状态为 <result>
    And 剩余 PP 为 <remaining_pp>

    Examples:
      | current_pp | psynergy   | cost | result           | remaining_pp |
      | 10         | Whirlwind  | 2    | Success          | 8            |
      | 10         | Freeze     | 4    | Success          | 6            |
      | 1          | Freeze     | 4    | InsufficientPP   | 1            |
      | 0          | Whirlwind  | 2    | InsufficientPP   | 0            |

  Scenario: 行走恢复 PP
    Given PP 为 5
    When 玩家行走了 10 秒
    Then PP 增加至 6
