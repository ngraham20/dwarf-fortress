use crate::objects::Rect;
use crate::constants::ROOM_MAX_SIZE;
use crate::constants::ROOM_MIN_SIZE;
use crate::constants::MAX_ROOMS;
use crate::constants::MAP_WIDTH;
use crate::constants::MAP_HEIGHT;
use crate::objects::Object;
use crate::objects::Tile;
use rand::Rng;
use std::cmp;

type Map = Vec<Tile>;

pub struct Game {
    pub map: Map,
}

pub fn make_map(player: &mut Object) -> Map {
    // fill map with "blocked" tiles
    let mut map = vec![Tile::wall(); (MAP_HEIGHT * MAP_WIDTH) as usize];

    let mut rooms = vec![];  // the rooms

    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);  // pick a random width
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);  // pick a random height

        // pick a random position
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        // generate the new room
        let new_room = Rect::new(x, y, w, h);

        // is this new room allowed?
        let failed = rooms
            .iter()  // generate iterative from the collection
            .any(|other_room| new_room.intersects_with(other_room));  // if any of the other rooms intersects the new room, then the new room cannot be created

        if !failed {
            // if we're here, there are no intersections, so this is a valid room

            // paint it to the map's tiles
            create_room(new_room, &mut map);

            let (new_x, new_y) = new_room.center();  // the x y coordinates are the center ones

            if rooms.is_empty() {
                // if rooms list is empty, then we're dealing with the first room

                // put the player in the middle of the room
                player.x = new_x;
                player.y = new_y;
            } else {
                // all rooms after the first one

                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();  // get the center of the latest room

                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);  // make an h tunnel from x of the first room to the x of the second
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);  // make a v tunnel from the y of the first room to the y of the second
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);  // make a v tunnel from the y of the first room to the y of the second
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);  // make an h tunnel from x of the first room to the x of the second
                }
            }

            rooms.push(new_room);  // add the new room onto the rooms list
        }

    }

    map
}

pub fn create_room(room: Rect, map: &mut Map) {
    // go through the tiles in the rectangle and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[(y * MAP_WIDTH + x) as usize] = Tile::empty();
        }
    }
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // h tunnel. 'min()' and 'max()' are used in case 'x1 > x2'
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[(y * MAP_WIDTH + x) as usize] = Tile::empty();
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[(y * MAP_WIDTH + x) as usize] = Tile::empty();
    }
}