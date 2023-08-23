use std::collections::HashSet;
use rand::seq::SliceRandom;

use bracket_lib::prelude::*;
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Floor,
    Minable(Minable),
}

#[derive(PartialEq, Copy, Clone)]
pub enum Minable {
    Stone,
    Iron,
    Copper,
}

impl Minable {
    pub fn get_rgb(&self) -> (RGB, RGB) {
        match self {
            Minable::Stone => (RGB::from_u8(111, 111, 111), RGB::from_u8(0, 0, 0)),
            Minable::Iron => (RGB::from_u8(144, 227, 245), RGB::from_u8(0, 0, 0)),
            Minable::Copper => (RGB::from_u8(191, 109, 73), RGB::from_u8(0, 0, 0)),
        }
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub tiles: Vec<Tile>,
    pub player_spawns: Vec<(i32, i32)>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    pub fn idx_xy(&self, pos: usize) -> (i32, i32) {
        ((pos / self.width) as i32, (pos % self.width) as i32)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = Tile::Floor;
            }
        }
        let center = (
            (room.x1 + room.x2) / 2,
            (room.y1 as i32 + room.y2 as i32) / 2,
        );
        self.player_spawns.push(center);
    }

    pub fn seed_ore_veins(&mut self) {
        // make 10 veins
        let mut rng = RandomNumberGenerator::new();
        for _i in 0..10 {
            let ores = vec![Minable::Copper, Minable::Iron];
            let ore = Tile::Minable(*ores.choose(&mut rand::thread_rng()).unwrap());
            let vein = self.gen_ore_vein(rng.roll_dice(1, (self.height*self.width) as i32) as usize);
            if let Some(v) = vein {
                for pos in v {
                    self.tiles[pos] = ore;
                }
            }
        }
    }

    fn gen_ore_vein(&self, pos: usize) -> Option<Vec<usize>> {
        // seed the first ore tile at a random location
        let mut seed: Option<usize> = None;
        let mut vein: Option<Vec<usize>>= None;
        match self.tiles[pos] {
            Tile::Minable(Minable::Stone) => {
                seed = Some(pos);
            },
            _ => {},
        }
        if let Some(s) = seed {
            let mut generated: Vec<usize> = Vec::new();
            let mut frontiers = Vec::new();
            frontiers.push(s);
            for _i in 0..15 {
                self.try_grow_ore_vein(&mut generated, &mut frontiers);
            }
            vein = Some(generated);
        };
        vein
    }

    fn try_grow_ore_vein(&self, vein: &mut Vec<usize>, frontiers: &mut Vec<usize>) {
        let mut rng = RandomNumberGenerator::new();

        // get random frontier
        let next = rng.random_slice_index(&frontiers).unwrap();
        // move frontier to vein
        let pos = frontiers[next];
        vein.push(pos);
        // add neighbors to frontier
        // n,s,e,w if not edge and not in vein
        // n
        if pos >= self.width && !frontiers.contains(&(pos-self.width)) {
            frontiers.push(pos-self.width);
        }
        // s
        if pos + self.width <= self.width*self.height && !frontiers.contains(&(pos+self.width)) {
            frontiers.push(pos+self.width);
        }
        // e
        if pos + 1 <= self.width*self.height && !frontiers.contains(&(pos+1)) {
            frontiers.push(pos+1);
        }
        // w
        if pos > 0 && !frontiers.contains(&(pos-1)) {
            frontiers.push(pos-1);
        }
        // pop frontier
        frontiers.swap_remove(next);
    }

    pub fn simple_80x50() -> Self {
        let mut map = Map {
            width: 80,
            height: 50,
            depth: 1,
            tiles: vec![Tile::Minable(Minable::Stone); 80 * 50],
            player_spawns: Vec::new(),
        };
        let room = Rect {
            x1: 30,
            x2: 50,
            y1: 15,
            y2: 35,
        };
        map.apply_room_to_map(&room);
        map.seed_ore_veins();
        map
    }

    pub fn draw(ecs: &World, ctx: &mut BTerm) {
        let map = ecs.fetch::<Map>();
        for (pos, tile) in map.tiles.iter().enumerate() {
            let (y, x) = map.idx_xy(pos);
            match tile {
                Tile::Floor => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_f32(0., 0., 0.),
                        to_cp437('.'),
                    );
                }
                Tile::Minable(ore) => {
                    let (fg, bg) = ore.get_rgb();
                    ctx.set(x, y, fg, bg, to_cp437('#'));
                }
            }
        }
    }
}