use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        Map::draw(&self.ecs, ctx);
        player_input(self, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renders = self.ecs.read_storage::<Render>();
        for (pos, render) in (&positions, &renders).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Floor,
    Stone,
}

struct Map {
    width: usize,
    height: usize,
    depth: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    fn idx_xy(&self, pos: usize) -> (i32, i32) {
        ((pos / self.width) as i32, (pos % self.width) as i32)
    }

    fn apply_room_to_map(&mut self, room : &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = Tile::Floor;
            }
        }
    }

    fn simple_80x50() -> Self {
        let mut map = Map {
            width: 80,
            height: 50,
            depth: 1,
            tiles: vec![Tile::Stone; 80*50],
        };
        let room = Rect { x1: 20, x2: 40, y1: 20, y2: 40};
        map.apply_room_to_map(&room);
        map
    }

    fn draw(ecs: &World, ctx: &mut BTerm) {
        let map = ecs.fetch::<Map>();
        for (pos, tile) in map.tiles.iter().enumerate() {
            let (y, x) = map.idx_xy(pos);
            match tile {
                Tile::Floor => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'));
                },
                Tile::Stone => {
                    ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), to_cp437('#'));
                }
            }
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

    let map = Map::simple_80x50();
    gs.ecs.insert(map);

    gs.ecs.create_entity()
        .with(Position { x: 5, y: 5 })
        .with(Render { glyph: to_cp437('@'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK)})
        .with(Player {})
        .build();
    main_loop(context, gs)
}