Feature: 回合制战斗系统
  作为玩家，我希望体验黄金太阳风格的回合制战斗，
  包括物理攻击、精灵召唤、元素克制、暴击和逃跑。

  Background:
    Given 我方有 3 名角色
    And 敌方有 3 名敌人

  # ── 物理攻击 ──
  Scenario: 基本物理攻击造成伤害
    Given 攻击方攻击力为 30
    And 防御方防御力为 10
    And 攻击方等级为 5
    When 执行物理攻击
    Then 伤害值在预期范围内 (45..53)

  Scenario: 高防御减免伤害
    Given 攻击方攻击力为 20
    And 防御方防御力为 30
    When 执行物理攻击
    Then 伤害值 <= 10

  # ── 暴击 ──
  Scenario: 速度优势提高暴击率
    Given 攻击方速度 50, 防御方速度 10
    When 计算暴击率
    Then 暴击率为最大值 40%

  Scenario: 速度劣势降低暴击率
    Given 攻击方速度 10, 防御方速度 50
    When 计算暴击率
    Then 暴击率为 3%

  # ── 精灵召唤 ──
  Scenario: 元素克制伤害加成
    Given 攻击方元素为 Earth
    And 防御方元素为 Water
    When 使用精灵力攻击, 威力 50
    Then 元素倍率为 1.25

  Scenario: 元素抗性伤害减免
    Given 攻击方元素为 Earth
    And 防御方元素为 Earth
    When 使用精灵力攻击, 威力 50
    Then 元素倍率为 0.75

  Scenario: 无克制关系无加成
    Given 攻击方元素为 Earth
    And 防御方元素为 Fire
    When 使用精灵力攻击, 威力 50
    Then 元素倍率为 1.0

  # ── 战斗状态机 ──
  Scenario: 完整战斗流程
    Given 进入战斗
    When 状态为 Intro
    Then 短暂显示遇敌动画
    When 进入 PlayerSelect
    Then 显示攻击/精灵/防御/道具/逃跑菜单
    When 选择 Attack 并指定目标
    Then 进入 ExecuteActions
    When 所有行动执行完毕
    Then 进入 EnemyTurn
    When AI 完成行动选择
    Then 进入 ResolveEffects
    When 无单位死亡
    Then 回到 PlayerSelect

  Scenario: 胜利条件
    Given 全部敌人 HP <= 0
    Then 状态切换为 Victory
    And 显示经验值获取
    And 3 秒后回到地图

  Scenario: 全灭条件
    Given 我方全部角色 HP <= 0
    Then 状态切换为 Defeat
    And 显示"全灭"
    And 提示读档

  # ── 逃跑 ──
  Scenario Outline: 逃跑成功率计算
    Given 我方平均速度 <party_speed>, 敌方平均速度 <enemy_speed>
    When 尝试逃跑
    Then 成功率为 <rate>

    Examples:
      | party_speed | enemy_speed | rate |
      | 50          | 10          | 0.9  |
      | 50          | 50          | 0.25 |
      | 10          | 50          | 0.05 |

  # ── AI 决策 ──
  Scenario Outline: AI 根据 HP 选择策略
    Given 敌人 HP 百分比为 <hp_pct>
    When AI 选择行动
    Then 攻击概率约为 <atk>, 精灵概率约为 <psyn>, 防御/道具概率约为 <def>

    Examples:
      | hp_pct | atk | psyn | def |
      | 80     | 0.4 | 0.3  | 0.3 |
      | 50     | 0.5 | 0.4  | 0.1 |
      | 20     | 0.0 | 0.6  | 0.4 |
