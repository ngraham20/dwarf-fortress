use crate::game::Game;
use crate::user_input::get_names_under_mouse;
use tcod::colors::LIGHT_GREY;
use tcod::input::Mouse;
use tcod::input::Key;
use tcod::colors::BLACK;
use tcod::colors::DARKER_RED;
use tcod::colors::LIGHT_RED;
use tcod::colors::Color;
use tcod::colors::WHITE;
use crate::object::Object;
use crate::constants::*;

use tcod::console::*;
use tcod::map::{Map as FovMap};

// ----------------------- PUBLIC STRUCTS -----------------------

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub panel: Offscreen,
    pub fov: FovMap,
    pub key: Key,
    pub mouse: Mouse,
}

pub struct Messages {
    messages: Vec<(String, Color)>,
}

impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    // T can be anything that can be turned into a string
    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }

    // returns something that implements the given trait (an iterator to a vec<(string, color)>)
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
        self.messages.iter()
    }
}

// ----------------------- PUBLIC FUNCTIONS -----------------------

pub fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool) {

    // ----------------------- MAP RENDERING -----------------------

    // compute fov
    if fov_recompute {
        let player = &objects[0];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALG);
    }

    // display colors based on region
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

    // calculate which objects to draw based on fov
    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|o| tcod.fov.is_in_fov(o.x, o.y))
        .collect();

    // sort so that the non-blocking objects come first
    to_draw.sort_by(|o1, o2| { o1.blocks.cmp(&o2.blocks) });

    // draw all objects in the list
    for object in &to_draw {
        object.draw(&mut tcod.con);
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
    // ----------------------- END MAP RENDERING -----------------------

    // ----------------------- STATUS BAR RENDERING -----------------------

    tcod.root.set_default_foreground(WHITE);
    if let Some(fighter) = objects[PLAYER].fighter {
        tcod.root.print_ex(
            1, 
            SCREEN_HEIGHT - 2,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("HP: {}/{} ", fighter.hp, fighter.max_hp),
        );
    }

    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();

    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    let hp = objects[PLAYER].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER].fighter.map_or(0, |f| f.max_hp);
    render_bar(
        &mut tcod.panel,
        1,
        1,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        LIGHT_RED,
        DARKER_RED,
    );

    tcod.panel.set_default_foreground(LIGHT_GREY);
    tcod.panel.print_ex(
        1,
        0,
        BackgroundFlag::None,
        TextAlignment::Left,
        get_names_under_mouse(tcod.mouse, objects, &tcod.fov),
    );

    blit(
        &tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_HEIGHT),
        &mut tcod.root,
        (0, PANEL_Y),
        1.0,
        1.0,
    );
    // ----------------------- END STATUS BAR RENDERING -----------------------
}

// ----------------------- PRIVATE FUNCTIONS -----------------------

fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,
) {
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    panel.set_default_foreground(WHITE);
    panel.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", name, value, maximum),
    );
}