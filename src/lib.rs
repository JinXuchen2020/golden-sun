//! # Golden Sun Rust Edition — 共享库入口
//!
//! 所有模块在此声明，供 main.rs 和跨模块引用使用。

pub mod engine;
pub mod map;
pub mod entity;
pub mod psynergy;
pub mod battle;
pub mod scene;
pub mod ui;
pub mod audio;

// ── 便捷 re-export ──
pub use engine::constants;
pub use engine::error::{GameError, GameResult};
pub use engine::input::{InputBus, InputEvent};
pub use engine::resources::ResourceManager;
pub use engine::storage::{create_storage, FsStorage, StorageBackend};
pub use engine::texture::TextureCache;
pub use engine::{Camera, FrameTime, GameState, InputState, RenderPhase, WindowConfig};
