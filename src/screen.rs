use crate::constants::*;
use crate::map::*;
use crate::objects::*;

use tcod::console::*;
use tcod::map::{Map as FovMap};

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub fov: FovMap,
}

pub fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool) {

    if fov_recompute {
        let player = &objects[0];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALG);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[(y * MAP_WIDTH + x) as usize].block_sight;
            let color = match (visible, wall) {
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };
            let explored = &mut game.map[(y * MAP_WIDTH + x) as usize].explored;
            if visible {
                *explored = true;
            }
            if *explored {
                tcod.con
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    // draw all objects in the list
    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }

    blit(
        &tcod.con,  //from
        (0, 0),  // at location
        (SCREEN_WIDTH, SCREEN_HEIGHT),  // with width, height
        &mut tcod.root,  // to
        (0, 0),  // at location
        1.0,  // foreground opacity
        1.0,  // background opacity
    );
}