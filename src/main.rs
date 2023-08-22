use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

mod state;
pub use state::*;

mod map;
pub use map::*;
pub use map::{Tile, Minable};

mod player;
pub use player::*;

mod components;
pub use components::*;
pub use components::{Position, Player};


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let mut gs: State = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Render>();
    gs.ecs.register::<Player>();

    let map = Map::simple_80x50();
    let (player_x, player_y) = map.player_spawns[0];
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Render {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .build();
    main_loop(context, gs)
}
