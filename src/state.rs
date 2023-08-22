use bracket_lib::prelude::*;
use specs::prelude::*;
use super::player::*;
use super::{Map, Position, Render};

pub struct State {
    pub ecs: World,
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