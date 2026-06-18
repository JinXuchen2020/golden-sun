//! # Golden Sun Rust Edition — 共享库入口
//!
//! 所有模块在此声明，供 main.rs 和跨模块引用使用。

#![deny(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]

pub mod audio;
pub mod battle;
pub mod data;
pub mod dialogue;
pub mod engine;
pub mod entity;
pub mod map;
pub mod psynergy;
pub mod scene;
pub mod ui;

// ── 便捷 re-export ──
pub use engine::constants;
pub use engine::error::{GameError, GameResult};
pub use engine::input::{InputBus, InputEvent};
pub use engine::mode7_camera::{Mode7Camera, ScanlineContext};
pub use engine::resources::ResourceManager;
pub use engine::storage::{create_storage, FsStorage, StorageBackend};
pub use engine::texture::TextureCache;
pub use engine::replay;
pub use engine::{Camera, FrameTime, GameState, InputState, PsynergyAnim, TransitionKind, WindowConfig};
pub use data::SaveData;
pub use data::quest::{QuestEntry, QuestLog};
pub use psynergy::{Element, PsynergyType};
pub use scene::{SceneId, SceneRegistry};
