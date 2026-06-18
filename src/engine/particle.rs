//! 天气粒子系统 — 降雨/降雪效果

use macroquad::prelude::draw_circle;
use quad_rand;

/// 粒子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Rain,
    Snow,
}

/// 单个粒子
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub lifetime: f32,
    pub kind: ParticleKind,
}

/// 粒子管理器
#[derive(Debug, Clone)]
pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    /// 每帧生成新粒子
    pub fn spawn(&mut self, dt: f32, kind: ParticleKind) {
        let count = match kind {
            ParticleKind::Rain => (dt * 10.0) as usize,
            ParticleKind::Snow => (dt * 5.0) as usize,
        };
        for _ in 0..count.min(self.max_particles.saturating_sub(self.particles.len())) {
            let speed = match kind {
                ParticleKind::Rain => 100.0 + quad_rand::gen_range(-20.0, 20.0),
                ParticleKind::Snow => 30.0 + quad_rand::gen_range(-10.0, 10.0),
            };
            self.particles.push(Particle {
                x: quad_rand::gen_range(0.0, 640.0),
                y: -10.0,
                speed,
                lifetime: match kind {
                    ParticleKind::Rain => 3.0,
                    ParticleKind::Snow => 5.0,
                },
                kind,
            });
        }
    }

    /// 更新粒子位置
    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.y += p.speed * dt;
            p.lifetime -= dt;
            if p.kind == ParticleKind::Snow {
                p.x += (p.y * 0.05).sin() * 0.5;
            }
        }
        self.particles.retain(|p| p.lifetime > 0.0 && p.y < 490.0);
    }

    /// 渲染所有粒子
    pub fn draw(&self) {
        for p in &self.particles {
            let color = match p.kind {
                ParticleKind::Rain => macroquad::prelude::Color::from_rgba(150, 180, 255, 180),
                ParticleKind::Snow => macroquad::prelude::Color::from_rgba(255, 255, 255, 200),
            };
            let size = match p.kind {
                ParticleKind::Rain => 1.5,
                ParticleKind::Snow => 2.5,
            };
            draw_circle(p.x, p.y, size, color);
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.particles.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.particles.is_empty()
    }
}
