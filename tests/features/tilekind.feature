Feature: 地图瓦片系统
  作为游戏设计师，我需要在 32x32 的世界地图中定义不同瓦片的属性，
  以便玩家能在可通行区域移动，遇到障碍时被阻挡。

  Background:
    Given 地图尺寸为 32x32

  Scenario Outline: 基础瓦片编解码
    When 将u8值 <value> 解码为TileKind
    Then 结果为 <tile>

    Examples:
      | value | tile      |
      | 0     | Void      |
      | 1     | Grass     |
      | 2     | Dirt      |
      | 3     | Water     |
      | 4     | Forest    |
      | 5     | Wall      |
      | 6     | Sand      |
      | 7     | Snow      |
      | 8     | Bridge    |
      | 9     | Stairs    |
      | 15    | PushBlock |
      | 255   | Unknown   |

  Scenario Outline: 可通行性判定
    Given 瓦片类型为 <tile>
    Then 可通行性为 <walkable>

    Examples:
      | tile     | walkable |
      | Grass    | true     |
      | Dirt     | true     |
      | Water    | false    |
      | Forest   | false    |
      | Wall     | false    |
      | Bridge   | true     |
      | Sand     | true     |
      | Snow     | true     |
      | Stairs   | true     |
      | Ice      | true     |
      | Vine     | false    |
      | PushBlock| false    |
      | VineClimbable | true  |
      | Unknown  | false    |

  Scenario Outline: 可交互性判定
    Given 瓦片类型为 <tile>
    Then 可交互性为 <interactive>

    Examples:
      | tile        | interactive |
      | Vine        | true        |
      | Seed        | true        |
      | PushBlock   | true        |
      | Windmill    | true        |
      | DarkArea    | true        |
      | HiddenChest | true        |
      | Grass       | false       |
      | Water       | false       |
      | Wall        | false       |
      | Ice         | false       |

  Scenario: 每种瓦片有独立颜色
    Given 两张不同瓦片
    When 查询它们的 color()
    Then 返回的RGB值不相等
    And 不是默认的黑色

  Scenario: 地图边界访问
    Given 32x32 地图
    When 访问 (0, 0) 位置
    Then 返回有效的瓦片类型
    When 访问 (31, 31) 位置
    Then 返回有效的瓦片类型
    When 访问 (32, 32) 位置
    Then 这是越界访问
