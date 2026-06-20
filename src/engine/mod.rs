//! 核心引擎模块 — 所有 Phase 共享的基础类型与接口

pub mod camera;
pub mod constants;
pub mod error;
pub mod frame_time;
pub mod game_state;
pub mod input;
pub mod mode7_camera;
pub mod resources;
pub mod storage;
pub mod texture;
pub mod window_config;
pub mod particle;
pub mod replay;

pub use camera::Camera;
pub use frame_time::FrameTime;
pub use game_state::{GameState, PsynergyAnim, TransitionKind};
pub use input::{InputBus, InputEvent, InputState};
pub use window_config::WindowConfig;
pub use constants::ItemType;
