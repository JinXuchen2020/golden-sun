//! 存档后端抽象 — 桌面端用文件系统，网页端用 localStorage
//!
//! ## 设计
//! - `StorageBackend` trait 定义统一的存/取/删接口
//! - `FsStorage` — desktop: 读写 `save.dat`（Phase 6 实现）
//! - `LocalStorage` — wasm32: 读写浏览器 localStorage（Phase 6 实现）
//! - Phase 0 提供骨架，Phase 6 填充完整逻辑
//!
//! ## 使用
//! ```ignore
//! // 桌面端
//! let storage: Box<dyn StorageBackend> = FsStorage::new(".");
//! storage.save("save", &bincode_serialized_data)?;
//! let loaded = storage.load("save")?;
//!
//! // 网页端 (wasm32)
//! let storage: Box<dyn StorageBackend> = LocalStorage::new("golden_sun");
//! storage.save("save", &bincode_serialized_data)?;
//! ```

use crate::GameResult;

/// 统一的存档存储接口
pub trait StorageBackend {
    /// 保存数据到指定 key
    fn save(&mut self, key: &str, data: &[u8]) -> GameResult<()>;
    /// 读取指定 key 的数据
    fn load(&self, key: &str) -> GameResult<Option<Vec<u8>>>;
    /// 检查 key 是否存在
    fn exists(&self, key: &str) -> bool;
    /// 删除指定 key 的数据
    fn delete(&mut self, key: &str) -> GameResult<()>;
}

// ── 桌面端: 文件系统存储 ──

/// 桌面端存储后端 — 读写本地文件
///
/// Phase 6 实现 std::fs 逻辑，当前为骨架。
#[derive(Debug)]
pub struct FsStorage {
    save_dir: std::path::PathBuf,
}

impl FsStorage {
    pub fn new(dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            save_dir: dir.into(),
        }
    }

    fn file_path(&self, key: &str) -> std::path::PathBuf {
        // 防御: key 必须是安全文件名
        let safe_key: String = key
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
            .collect();
        self.save_dir.join(format!("{safe_key}.dat"))
    }
}

impl StorageBackend for FsStorage {
    fn save(&mut self, key: &str, data: &[u8]) -> GameResult<()> {
        ensure_save_dir(&self.save_dir)?;
        let path = self.file_path(key);
        // Phase 6: std::fs::write(path, data)?;
        let _ = (path, data);
        Ok(())
    }

    fn load(&self, key: &str) -> GameResult<Option<Vec<u8>>> {
        let path = self.file_path(key);
        // Phase 6:
        // if path.exists() {
        //     Ok(Some(std::fs::read(path)?))
        // } else {
        //     Ok(None)
        // }
        let _ = path;
        Ok(None)
    }

    fn exists(&self, key: &str) -> bool {
        self.file_path(key).exists()
    }

    fn delete(&mut self, key: &str) -> GameResult<()> {
        let path = self.file_path(key);
        // Phase 6: if path.exists() { std::fs::remove_file(path)?; }
        let _ = path;
        Ok(())
    }
}

// ── 网页端: localStorage 存储 ──

/// 网页端存储后端 — 浏览器 localStorage
///
/// 仅在 `target_arch = "wasm32"` 时编译。
/// Phase 6 实现 JSON 序列化逻辑，当前为骨架。
#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub struct LocalStorage {
    prefix: String,
}

#[cfg(target_arch = "wasm32")]
impl LocalStorage {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    fn full_key(&self, key: &str) -> String {
        format!("{}_{}", self.prefix, key)
    }
}

#[cfg(target_arch = "wasm32")]
impl StorageBackend for LocalStorage {
    fn save(&mut self, key: &str, data: &[u8]) -> GameResult<()> {
        let storage = web_sys::window()
            .ok_or(crate::GameError::LogicError("no window".into()))?
            .local_storage()
            .map_err(|_| crate::GameError::SaveError("localStorage不可用".into()))?
            .ok_or(crate::GameError::SaveError("localStorage为null".into()))?;

        // hex 编码绕过 localStorage 的 UTF-16 限制
        let encoded = hex_encode(data);
        storage
            .set_item(&self.full_key(key), &encoded)
            .map_err(|_| crate::GameError::SaveError("写入localStorage失败".into()))?;
        Ok(())
    }

    fn load(&self, key: &str) -> GameResult<Option<Vec<u8>>> {
        let storage = web_sys::window()
            .ok_or(crate::GameError::LogicError("no window".into()))?
            .local_storage()
            .map_err(|_| crate::GameError::SaveError("localStorage不可用".into()))?
            .ok_or(crate::GameError::SaveError("localStorage为null".into()))?;

        match storage.get_item(&self.full_key(key)) {
            Ok(Some(encoded)) => {
                let data = hex_decode(&encoded)
                    .ok_or(crate::GameError::SaveError("hex解码失败".into()))?;
                Ok(Some(data))
            }
            Ok(None) => Ok(None),
            Err(_) => Ok(None),
        }
    }

    fn exists(&self, key: &str) -> bool {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .map(|s| s.get_item(&self.full_key(key)).ok().flatten().is_some())
            .unwrap_or(false)
    }

    fn delete(&mut self, key: &str) -> GameResult<()> {
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
        {
            storage
                .remove_item(&self.full_key(key))
                .map_err(|_| crate::GameError::SaveError("删除失败".into()))?;
        }
        Ok(())
    }
}

// ── 帮助函数 ──

/// hex 编码 — 将二进制数据转为十六进制字符串
/// 用于 localStorage 存储（wasm32，避免 UTF-16 编码问题）
/// Phase 6 可替换为更紧凑的 base64 编码
#[cfg(target_arch = "wasm32")]
fn hex_encode(data: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(data.len() * 2);
    for b in data {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

/// hex 解码
#[cfg(target_arch = "wasm32")]
fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 { return None; }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

// ── 工厂函数 ──

/// 确定桌面端存档目录（仅计算路径，不创建目录）
fn default_save_dir() -> std::path::PathBuf {
    // 优先 APPDATA (Windows)
    if let Ok(dir) = std::env::var("APPDATA") {
        return std::path::PathBuf::from(dir).join("golden-sun");
    }
    // 次优先 XDG_DATA_HOME / HOME (Linux/macOS)
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        return std::path::PathBuf::from(dir).join("golden-sun");
    }
    if let Ok(dir) = std::env::var("HOME") {
        return std::path::PathBuf::from(dir).join(".local/share/golden-sun");
    }
    // fallback: 可执行文件同级 save 目录
    std::path::PathBuf::from("./save")
}

/// 确保存档目录存在（写入时按需创建，通过 From<io::Error> 传播错误）
fn ensure_save_dir(dir: &std::path::Path) -> GameResult<()> {
    Ok(std::fs::create_dir_all(dir)?)
}

/// 根据编译目标创建对应的 StorageBackend
pub fn create_storage() -> Box<dyn StorageBackend> {
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(LocalStorage::new("golden_sun"))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(FsStorage::new(default_save_dir()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fs_storage_file_path_is_safe() {
        let storage = FsStorage::new(".");
        let path = storage.file_path("my_save");
        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        assert_eq!(filename, "my_save.dat");
    }

    #[test]
    fn fs_storage_sanitizes_special_chars() {
        let storage = FsStorage::new(".");
        let path = storage.file_path("bad/../../etc");
        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        assert!(!filename.contains('/'));
        assert!(filename.starts_with("bad"));
    }

    #[test]
    fn fs_storage_exists_nonexistent() {
        let storage = FsStorage::new(".");
        assert!(!storage.exists("nonexistent_save_xyz"));
    }
}
