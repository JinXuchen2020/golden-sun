//! 统一游戏错误类型

use std::fmt;

/// 游戏运行时可能出现的错误
#[derive(Debug)]
pub enum GameError {
    /// 地图数据格式错误
    MapParseError(String),
    /// 精灵/纹理资源加载失败
    AssetLoadError(String),
    /// 存档序列化/反序列化失败
    SaveError(String),
    /// 运行时逻辑错误（不该发生的情况）
    LogicError(String),
    /// 其他 I/O 错误
    IoError(std::io::Error),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::MapParseError(msg) => write!(f, "地图解析错误: {msg}"),
            GameError::AssetLoadError(msg) => write!(f, "资源加载失败: {msg}"),
            GameError::SaveError(msg) => write!(f, "存档错误: {msg}"),
            GameError::LogicError(msg) => write!(f, "运行时逻辑错误: {msg}"),
            GameError::IoError(e) => write!(f, "I/O 错误: {e}"),
        }
    }
}

impl std::error::Error for GameError {}

impl From<std::io::Error> for GameError {
    fn from(e: std::io::Error) -> Self {
        GameError::IoError(e)
    }
}

/// 便捷类型别名
pub type GameResult<T> = Result<T, GameError>;
