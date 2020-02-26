mod constants;
mod screen;
mod user_input;
mod object;
mod tile;
mod map;
mod game;

use tcod::colors::RED;
use crate::map::*;
use crate::object::*;
use crate::constants::*;
use crate::screen::*;
use crate::user_input::*;
use tcod::console::*;
use tcod::map::{Map as FovMap};
use tcod::input::{self, Event};

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
        panel: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        key: Default::default(),
        mouse: Default::default(),
    };

    let mut objects = vec![];

    let mut game = Game {
        map: make_map(&mut objects),
        messages: Messages::new(),
    };

    game.messages.add(
        "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
        RED,
    );

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

        let fov_recompute = previous_player_position != (objects[PLAYER].x, objects[0].y);

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        }
        render_all(&mut tcod, &mut game, &objects, fov_recompute);

        tcod.root.flush();
        //tcod.root.wait_for_keypress(true);
        let player = &mut objects[PLAYER];
        previous_player_position = objects[PLAYER].pos();
        let player_action = handle_keys(&mut tcod, &mut game, &mut objects);
        if player_action == PlayerAction::Exit {
            break;
        }

        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &tcod, &mut game, &mut objects);
                }
            }
        }
    }
}