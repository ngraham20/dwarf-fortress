use tcod::colors::*;
use tcod::map::FovAlgorithm;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub const FOV_ALG: FovAlgorithm = FovAlgorithm::Basic;  // default FOV algorithm
pub const FOV_LIGHT_WALLS: bool = true;  // light walls or not
pub const TORCH_RADIUS: i32 = 10;

pub const COLOR_DARK_WALL: Color = Color {r: 0, g: 0, b: 100};
pub const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
pub const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

pub const LIMIT_FPS: i32 = 20;

pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 30;