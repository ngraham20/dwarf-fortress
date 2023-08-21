use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        player_input(self, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renders = self.ecs.read_storage::<Render>();
        for (pos, render) in (&positions, &renders).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
#[storage(VecStorage)]
struct Render {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
#[storage(HashMapStorage)]
struct Player {}

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    use std::cmp::{min, max};
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    for (_p, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + dx));
        pos.y = min(49, max(0, pos.y + dy));
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::W |
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::S |
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::A |
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::D |
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            _ => {}
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let mut gs: State = State {
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Render>();
    gs.ecs.register::<Player>();

    gs.ecs.create_entity()
        .with(Position { x: 5, y: 5 })
        .with(Render { glyph: to_cp437('@'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK)})
        .with(Player {})
        .build();

    gs.ecs.create_entity()
        .with(Position { x: 10, y: 10 })
        .with(Render { glyph: to_cp437('@'), fg: RGB::named(RED), bg: RGB::named(BLACK)})
        .with(Player {})
        .build();
    main_loop(context, gs)
}