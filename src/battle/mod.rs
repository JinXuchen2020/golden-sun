//! 战斗系统（Phase 5）
//!
//! 回合制战斗，支持物理攻击、精灵力、元素克制、简单敌人 AI。

pub mod calculator;
pub mod state;

pub use state::{
    AttackResult, Battle, BattleAction, BattlePhase, BattleTurn, Combatant, StatusEffect,
};
