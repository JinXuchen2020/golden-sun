use crate::engine::constants::SPRITE_SIZE;
use crate::entity::Direction;

/// 单帧 RGBA 像素数据
#[derive(Debug, Clone)]
pub struct AnimFrame {
    pub pixels: Vec<u8>,
}

/// 动画序列
#[derive(Debug, Clone)]
pub struct Animation {
    pub frames: Vec<AnimFrame>,
}

/// 动画状态（8 种：4 方向 × idle/walk）
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum AnimState {
    IdleDown, WalkDown,
    IdleUp, WalkUp,
    IdleLeft, WalkLeft,
    IdleRight, WalkRight,
}

impl AnimState {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_dir(dir: Direction, walking: bool) -> Self {
        match (dir, walking) {
            (Direction::Down, false) => Self::IdleDown,
            (Direction::Down, true)  => Self::WalkDown,
            (Direction::Up, false)   => Self::IdleUp,
            (Direction::Up, true)    => Self::WalkUp,
            (Direction::Left, false) => Self::IdleLeft,
            (Direction::Left, true)  => Self::WalkLeft,
            (Direction::Right, false)=> Self::IdleRight,
            (Direction::Right, true) => Self::WalkRight,
        }
    }

    pub fn direction(self) -> Direction {
        match self {
            Self::IdleDown | Self::WalkDown => Direction::Down,
            Self::IdleUp   | Self::WalkUp   => Direction::Up,
            Self::IdleLeft | Self::WalkLeft => Direction::Left,
            Self::IdleRight| Self::WalkRight=> Direction::Right,
        }
    }
}

const S: u32 = SPRITE_SIZE;

fn make_blank_frame() -> AnimFrame {
    AnimFrame {
        pixels: vec![0u8; (S * S * 4) as usize],
    }
}

fn set_px(pixels: &mut [u8], x: u32, y: u32, r: u8, g: u8, b: u8) {
    if x >= S || y >= S { return; }
    let i = ((y * S + x) * 4) as usize;
    pixels[i] = r;
    pixels[i + 1] = g;
    pixels[i + 2] = b;
    pixels[i + 3] = 255;
}

fn fill_rect(pixels: &mut [u8], x0: u32, y0: u32, x1: u32, y1: u32, color: (u8, u8, u8)) {
    for y in y0..=y1.min(S - 1) {
        for x in x0..=x1.min(S - 1) {
            set_px(pixels, x, y, color.0, color.1, color.2);
        }
    }
}

const SKIN: (u8, u8, u8) = (255, 220, 180);
const LEG: (u8, u8, u8) = (80, 60, 60);

/// 绘制罗宾（红色兜帽 + 黄发）
fn draw_robin(pixels: &mut [u8], walk: bool, frame: u32) {
    let leg_offset = if walk && frame == 1 { 1 } else { 0 };

    fill_rect(pixels, 5, 1, 10, 2, (240, 200, 80));
    fill_rect(pixels, 4, 3, 5, 4, (240, 200, 80));
    fill_rect(pixels, 10, 3, 11, 4, (240, 200, 80));
    fill_rect(pixels, 5, 3, 10, 5, SKIN);
    fill_rect(pixels, 4, 6, 11, 9, (200, 40, 40));
    fill_rect(pixels, 3, 7, 4, 8, (200, 40, 40));
    fill_rect(pixels, 11, 7, 12, 8, (200, 40, 40));
    fill_rect(pixels, 3, 9, 4, 11, (200, 40, 40));
    fill_rect(pixels, 11, 9, 12, 11, (200, 40, 40));
    fill_rect(pixels, 5, 10, 7, 13, (60, 40, 100));
    fill_rect(pixels, 8, 10, 10, 13, (60, 40, 100));
    if leg_offset == 1 {
        fill_rect(pixels, 4, 13, 7, 14, (60, 40, 100));
        fill_rect(pixels, 9, 13, 12, 14, (60, 40, 100));
    } else {
        fill_rect(pixels, 5, 13, 7, 14, (60, 40, 100));
        fill_rect(pixels, 8, 13, 10, 14, (60, 40, 100));
    }
}

/// 绘制 NPC（方块身体 + 方向不同色帽子）
fn draw_npc(pixels: &mut [u8], walk: bool, frame: u32, dir: Direction, hat: (u8, u8, u8), body: (u8, u8, u8)) {
    let leg_offset = if walk && frame == 1 { 1 } else { 0 };

    match dir {
        Direction::Down => {
            fill_rect(pixels, 4, 0, 11, 2, hat);
            fill_rect(pixels, 5, 3, 10, 3, hat);
            fill_rect(pixels, 5, 4, 10, 6, SKIN);
            fill_rect(pixels, 4, 7, 11, 10, body);
            fill_rect(pixels, 3, 8, 4, 10, body);
            fill_rect(pixels, 11, 8, 12, 10, body);
        }
        Direction::Up => {
            fill_rect(pixels, 4, 0, 11, 3, hat);
            fill_rect(pixels, 4, 7, 11, 10, body);
            fill_rect(pixels, 3, 8, 4, 10, body);
            fill_rect(pixels, 11, 8, 12, 10, body);
        }
        Direction::Left => {
            fill_rect(pixels, 4, 0, 9, 2, hat);
            fill_rect(pixels, 5, 3, 8, 3, hat);
            fill_rect(pixels, 5, 4, 8, 6, SKIN);
            fill_rect(pixels, 4, 7, 9, 10, body);
            fill_rect(pixels, 3, 8, 4, 10, body);
        }
        Direction::Right => {
            fill_rect(pixels, 6, 0, 11, 2, hat);
            fill_rect(pixels, 7, 3, 10, 3, hat);
            fill_rect(pixels, 7, 4, 10, 6, SKIN);
            fill_rect(pixels, 6, 7, 11, 10, body);
            fill_rect(pixels, 11, 8, 12, 10, body);
        }
    }

    fill_rect(pixels, 5, 11, 7, 13, LEG);
    fill_rect(pixels, 8, 11, 10, 13, LEG);
    if leg_offset == 1 {
        fill_rect(pixels, 4, 13, 7, 14, LEG);
        fill_rect(pixels, 9, 13, 12, 14, LEG);
    } else {
        fill_rect(pixels, 5, 13, 7, 14, LEG);
        fill_rect(pixels, 8, 13, 10, 14, LEG);
    }
}

fn build_anim(frames: Vec<AnimFrame>) -> Animation {
    Animation { frames }
}

/// 生成罗宾的全部动画帧
#[must_use]
pub fn generate_player_animations() -> Vec<(AnimState, Animation)> {
    // 各方向 idle 1 帧 + walk 2 帧
    let mut result = Vec::with_capacity(8);

    for &(state, is_walk) in &[
        (AnimState::IdleDown, false),
        (AnimState::WalkDown, true),
        (AnimState::IdleUp, false),
        (AnimState::WalkUp, true),
        (AnimState::IdleLeft, false),
        (AnimState::WalkLeft, true),
        (AnimState::IdleRight, false),
        (AnimState::WalkRight, true),
    ] {
        let frame_count = if is_walk { 2 } else { 1 };
        let frames: Vec<AnimFrame> = (0..frame_count).map(|f| {
            let mut frame = make_blank_frame();
            draw_robin(&mut frame.pixels, is_walk, f);
            frame
        }).collect();
        result.push((state, build_anim(frames)));
    }
    result
}

/// 生成 NPC 动画（8 方向 × idle/walk = 8 状态）
#[must_use]
pub fn generate_npc_animations(hat: (u8, u8, u8), body: (u8, u8, u8)) -> Vec<(AnimState, Animation)> {
    let mut result = Vec::with_capacity(8);
    for &(state, is_walk) in &[
        (AnimState::IdleDown, false), (AnimState::WalkDown, true),
        (AnimState::IdleUp, false),   (AnimState::WalkUp, true),
        (AnimState::IdleLeft, false), (AnimState::WalkLeft, true),
        (AnimState::IdleRight, false),(AnimState::WalkRight, true),
    ] {
        let dir = state.direction();
        let frame_count = if is_walk { 2 } else { 1 };
        let frames: Vec<AnimFrame> = (0..frame_count).map(|f| {
            let mut frame = make_blank_frame();
            draw_npc(&mut frame.pixels, is_walk, f, dir, hat, body);
            frame
        }).collect();
        result.push((state, build_anim(frames)));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_has_12_frames() {
        let anims = generate_player_animations();
        let total: usize = anims.iter().map(|(_, a)| a.frames.len()).sum();
        // 4 idle (1 each) + 4 walk (2 each) = 12
        assert_eq!(total, 12);
    }

    #[test]
    fn npc_animation_has_different_colors() {
        let npc = generate_npc_animations((255, 0, 0), (100, 100, 200));
        assert_eq!(npc.len(), 8);
    }

    #[test]
    fn frame_size_is_correct() {
        let f = make_blank_frame();
        assert_eq!(f.pixels.len(), (S * S * 4) as usize);
    }

    #[test]
    fn player_robin_has_visible_pixels() {
        let mut frame = make_blank_frame();
        draw_robin(&mut frame.pixels, false, 0);
        let has_content = frame.pixels.iter().any(|&b| b > 0);
        assert!(has_content, "robin sprite should not be empty");
    }
}
