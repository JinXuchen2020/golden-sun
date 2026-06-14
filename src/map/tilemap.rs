//! Vale 村地图数据
//!
//! 32×32 网格，编码对应 `TileKind::from_u8()`:
//!   0=Void 1=Grass 2=Dirt 3=Water 4=Forest 5=Wall 6=Sand 7=Snow
//!   8=Bridge 9=Stairs 10=Flower 11=Roof

use crate::engine::constants;
use crate::map::TileKind;

const MAP_W: i32 = constants::MAP_WIDTH as i32;
const MAP_H: i32 = constants::MAP_HEIGHT as i32;

/// Vale 村地图数据（32×32 常量构造）
const MAP_DATA: [[u8; 32]; 32] = {
    let mut map = [[1u8; 32]; 32];

    // ── 外围森林边界 ──
    let mut i = 0;
    while i < 32 {
        map[0][i] = 4;
        map[1][i] = 4;
        map[30][i] = 4;
        map[31][i] = 4;
        map[i][0] = 4;
        map[i][1] = 4;
        map[i][30] = 4;
        map[i][31] = 4;
        i += 1;
    }

    // ── 过渡带：零星树木 ──
    let mut c = 2;
    while c < 30 {
        if c % 5 == 0 || c % 7 == 0 { map[2][c] = 4; }
        if c % 4 == 0 { map[3][c] = 4; }
        if c % 5 == 0 { map[28][c] = 4; }
        if c % 6 == 0 { map[29][c] = 4; }
        c += 1;
    }

    // ── 房屋 1 (左上) ──
    // 顶墙 row 6, cols 6-10
    let mut x = 6; while x < 11 { map[6][x] = 5; x += 1; }
    // 底墙 row 9, cols 6-10
    let mut x = 6; while x < 11 { map[9][x] = 5; x += 1; }
    map[6][8] = 11; map[6][9] = 11;
    map[7][8] = 11; map[7][9] = 11;
    map[8][8] = 11; map[8][9] = 11;

    // 门前泥路
    map[9][7] = 2;
    map[9][8] = 2;

    // ── 房屋 2 (右上) ──
    let mut x = 20; while x < 25 { map[6][x] = 5; x += 1; }
    let mut x = 20; while x < 25 { map[9][x] = 5; x += 1; }
    // 屋顶 2x2
    map[6][22] = 11; map[6][23] = 11;
    map[7][22] = 11; map[7][23] = 11;
    map[8][22] = 11; map[8][23] = 11;
    // 门前泥路
    map[9][23] = 2;
    map[9][24] = 2;

    // ── 房屋 3 (中间大屋) ──
    // 顶墙 row 11, cols 12-18
    let mut x = 12; while x < 19 { map[11][x] = 5; x += 1; }
    // 底墙 row 15, cols 12-18
    let mut x = 12; while x < 19 { map[15][x] = 5; x += 1; }
    // 左右墙
    let mut y = 12; while y < 15 { map[y][12] = 5; y += 1; }
    let mut y = 12; while y < 15 { map[y][18] = 5; y += 1; }
    // 屋顶 3x3
    let mut ry = 12; while ry < 15 {
        let mut rx = 14; while rx < 17 {
            map[ry][rx] = 11;
            rx += 1;
        }
        ry += 1;
    }
    // 门口
    map[15][15] = 2;

    // ── 村中心泥路 ──
    // 南北主路
    let mut y = 16; while y < 21 { map[y][15] = 2; y += 1; }
    // 东西横路 (row 10)
    let mut x = 8; while x < 24 { map[10][x] = 2; x += 1; }
    // 连接房屋 1 和 2
    map[10][7] = 2; map[10][8] = 2;
    map[10][22] = 2; map[10][23] = 2;

    // ── 装饰花丛 ──
    map[11][6] = 10; map[5][14] = 10;
    map[10][5] = 10; map[5][12] = 10;
    map[5][18] = 10;

    // ── 池塘 (圆形近似) ──
    // 中心 (23, 21), 半径 3.5
    let mut ry = 20i32;
    while ry <= 26 {
        let mut cx = 17i32;
        while cx <= 25 {
            let dx = cx - 21;
            let dy = ry - 23;
            if dx * dx + dy * dy <= 13 {  // 3.5² ≈ 12.25, 取 13
                map[ry as usize][cx as usize] = 3;
            }
            cx += 1;
        }
        ry += 1;
    }

    // 木桥横跨池塘
    map[23][20] = 8;
    map[23][21] = 8;
    map[23][22] = 8;

    // ── 玩家起始点空地 ──
    map[16][15] = 2;
    map[16][14] = 2;
    map[17][15] = 2;

    map
};

pub fn get_tile(x: i32, y: i32) -> TileKind {
    if !(0..MAP_W).contains(&x) || !(0..MAP_H).contains(&y) {
        return TileKind::Void;
    }
    TileKind::from_u8(MAP_DATA[y as usize][x as usize])
}

pub fn is_walkable(x: i32, y: i32) -> bool {
    get_tile(x, y).is_walkable()
}

pub const fn map_size() -> (i32, i32) {
    (MAP_W, MAP_H)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_start_is_walkable() {
        // 玩家起始在 x=15, y=16
        assert!(is_walkable(15, 16));
    }

    #[test]
    fn map_bounds_void() {
        assert_eq!(get_tile(-1, 5), TileKind::Void);
        assert_eq!(get_tile(32, 5), TileKind::Void);
    }

    #[test]
    fn forest_is_not_walkable() {
        assert!(!is_walkable(0, 0));
    }

    #[test]
    fn bridge_is_walkable() {
        // 桥在 (x=20..22, y=23)
        assert!(is_walkable(20, 23));
        assert!(is_walkable(21, 23));
    }

    #[test]
    fn road_is_walkable() {
        assert!(is_walkable(8, 10));   // 东西横路
        assert!(is_walkable(15, 16));  // 玩家起始
        assert!(is_walkable(22, 10));  // 东西横路
    }
}
