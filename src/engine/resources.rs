//! 全局资源管理器 — 纹理/音频/字体的加载、缓存与生命周期管理
//!
//! ## 设计原则
//! - 各 Phase 的资源通过 `ResourceManager` 统一加载，避免重复创建
//! - 存档/读档时，纹理可通过 ID 重建
//! - Phase 0 仅定义骨架，各 Phase 按需扩展

use std::collections::HashMap;

/// 资源句柄 — 唯一标识一个已加载的资源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceHandle(u64);

/// 纹理资源
#[derive(Debug, Clone)]
pub struct TextureData {
    /// RGBA 像素数据
    pub pixels: Vec<u8>,
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
}

/// 音频样本
#[derive(Debug, Clone)]
pub struct AudioSample {
    /// PCM 采样数据（f32, -1.0..1.0, 单声道）
    pub data: Vec<f32>,
    /// 采样率
    pub sample_rate: u32,
}

/// 全局资源管理器
pub struct ResourceManager {
    textures: HashMap<String, TextureData>,
    audio: HashMap<String, AudioSample>,
    /// 程序化生成的纹理注册表（存档恢复时需重建）
    procedural_ids: Vec<String>,
    next_handle: u64,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            audio: HashMap::new(),
            procedural_ids: Vec::new(),
            next_handle: 0,
        }
    }

    fn gen_handle(&mut self) -> ResourceHandle {
        let h = ResourceHandle(self.next_handle);
        self.next_handle += 1;
        h
    }

    // ── 纹理管理 ──

    /// 存储纹理并返回句柄
    pub fn store_texture(&mut self, id: &str, data: TextureData) -> ResourceHandle {
        let handle = self.gen_handle();
        self.textures.insert(id.to_string(), data);
        handle
    }

    /// 获取纹理（不可变引用）
    pub fn get_texture(&self, id: &str) -> Option<&TextureData> {
        self.textures.get(id)
    }

    /// 标记为程序化纹理（读档时需重建）
    pub fn mark_procedural(&mut self, id: &str) {
        self.procedural_ids.push(id.to_string());
    }

    /// 卸载指定纹理
    pub fn unload_texture(&mut self, id: &str) {
        self.textures.remove(id);
    }

    // ── 音频管理 ──

    /// 存储音频样本
    pub fn store_audio(&mut self, id: &str, sample: AudioSample) -> ResourceHandle {
        let handle = self.gen_handle();
        self.audio.insert(id.to_string(), sample);
        handle
    }

    /// 获取音频样本
    pub fn get_audio(&self, id: &str) -> Option<&AudioSample> {
        self.audio.get(id)
    }

    // ── 生命周期 ──

    /// 卸载所有资源（场景切换/退出时调用）
    pub fn clear(&mut self) {
        self.textures.clear();
        self.audio.clear();
        self.procedural_ids.clear();
    }

    /// 卸载所有程序化纹理（存档恢复时，程序化纹理需重新生成）
    pub fn unload_procedural(&mut self) {
        for id in &self.procedural_ids {
            self.textures.remove(id);
        }
        self.procedural_ids.clear();
    }
}
