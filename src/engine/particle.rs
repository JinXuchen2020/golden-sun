//! 天气粒子系统 — 降雨/降雪效果

use macroquad::prelude::draw_circle;
use quad_rand;
use macroquad::prelude::Color;

/// 粒子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Rain,
    Snow,
    Sparkle,
    Leaf,
}

/// 单个粒子
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub lifetime: f32,
    pub kind: ParticleKind,
    pub vx: f32,
    pub vy: f32,
    pub max_life: f32,
    pub size: f32,
    pub color: Color,
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
            _ => 0,
        };
        for _ in 0..count.min(self.max_particles.saturating_sub(self.particles.len())) {
            let speed = match kind {
                ParticleKind::Rain => 100.0 + quad_rand::gen_range(-20.0, 20.0),
                ParticleKind::Snow => 30.0 + quad_rand::gen_range(-10.0, 10.0),
                _ => 0.0,
            };
            self.particles.push(Particle {
                x: quad_rand::gen_range(0.0, 640.0),
                y: -10.0,
                speed,
                lifetime: match kind {
                    ParticleKind::Rain => 3.0,
                    ParticleKind::Snow => 5.0,
                    _ => 3.0,
                },
                kind,
                vx: 0.0,
                vy: 0.0,
                max_life: 3.0,
                size: 1.5,
                color: Color::from_rgba(255, 255, 255, 255),
            });
        }
    }

    /// 更新粒子位置
    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.lifetime -= dt;
            match p.kind {
                ParticleKind::Rain => {
                    p.y += p.speed * dt;
                }
                ParticleKind::Snow => {
                    p.y += p.speed * dt;
                    p.x += (p.y * 0.05).sin() * 0.5;
                }
                ParticleKind::Leaf => {
                    p.x += p.vx;
                    p.y += p.vy;
                }
                ParticleKind::Sparkle => {
                    p.x += p.vx;
                    p.y += p.vy;
                }
            }
        }
        self.particles.retain(|p| p.lifetime > 0.0 && p.y < 490.0);
    }

    /// 渲染所有粒子
    pub fn draw(&self) {
        for p in &self.particles {
            let color = match p.kind {
                ParticleKind::Rain => Color::from_rgba(150, 180, 255, 180),
                ParticleKind::Snow => Color::from_rgba(255, 255, 255, 200),
                ParticleKind::Leaf => Color::new(139.0, 90.0, 43.0, p.lifetime / p.max_life * 200.0 / 255.0),
                ParticleKind::Sparkle => Color::new(255.0, 215.0, 0.0, p.lifetime / p.max_life * 255.0 / 255.0),
            };
            let size = match p.kind {
                ParticleKind::Rain => 1.5,
                ParticleKind::Snow => 2.5,
                _ => p.size,
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

    /// 手动添加自定义粒子
    pub fn push_particle(&mut self, p: Particle) {
        if self.particles.len() < self.max_particles {
            self.particles.push(p);
        }
    }

    /// 统计某种粒子类型的数量
    pub fn count_by_kind(&self, kind: ParticleKind) -> usize {
        self.particles.iter().filter(|p| p.kind == kind).count()
    }
}

/// 生成落叶粒子
pub fn generate_leaf_particles(count: usize) -> Vec<Particle> {
    (0..count).map(|_| Particle {
        x: quad_rand::gen_range(0.0, 640.0),
        y: quad_rand::gen_range(0.0, 480.0),
        speed: 0.0,
        vx: -0.5 + quad_rand::gen_range(0.0, 1.0),
        vy: 0.5 + quad_rand::gen_range(0.0, 1.0),
        lifetime: 3.0 + quad_rand::gen_range(0.0, 3.0),
        max_life: 6.0,
        color: Color::new(139.0, 90.0, 43.0, 200.0 / 255.0),
        size: 2.0 + quad_rand::gen_range(0.0, 3.0),
        kind: ParticleKind::Leaf,
    }).collect()
}

/// 生成 Djinn 闪光粒子
pub fn generate_sparkle_particles(count: usize) -> Vec<Particle> {
    (0..count).map(|_| Particle {
        x: quad_rand::gen_range(0.0, 640.0),
        y: quad_rand::gen_range(0.0, 480.0),
        speed: 0.0,
        vx: quad_rand::gen_range(-0.3, 0.3),
        vy: quad_rand::gen_range(-0.3, 0.3),
        lifetime: 1.0 + quad_rand::gen_range(0.0, 2.0),
        max_life: 3.0,
        color: Color::new(255.0, 215.0, 0.0, 1.0),
        size: 1.5 + quad_rand::gen_range(0.0, 2.5),
        kind: ParticleKind::Sparkle,
    }).collect()
}
