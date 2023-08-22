use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Render {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Player {}