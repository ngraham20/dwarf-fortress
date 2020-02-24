mod constants;
mod screen;
mod user_input;
mod objects;
mod map;

use crate::map::*;
use crate::objects::*;
use crate::constants::*;
use crate::screen::*;
use crate::user_input::*;

use tcod::colors::*;
use tcod::console::*;
use tcod::map::{Map as FovMap};

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
    .font("arial10x10.png", FontLayout::Tcod)
    .font_type(FontType::Greyscale)
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .title("Rust/libtcod tutorial")
    .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
    };

    let player = Object::new(0, 0, '@', WHITE);

    let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', YELLOW);

    let mut objects = [player, npc];

    let mut game = Game {
        map: make_map(&mut objects[0]),
    };

    // populate the FOV map, according to the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[(y * MAP_WIDTH + x) as usize].block_sight,
                !game.map[(y * MAP_WIDTH + x) as usize].blocked,
            );
        }
    }

    // force FOV to "recompute" before the game loop
    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {

        tcod.con.clear();

        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut tcod, &mut game, &objects, fov_recompute);

        tcod.root.flush();
        //tcod.root.wait_for_keypress(true);
        let player = &mut objects[0];
        previous_player_position = (player.x, player.y);
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}