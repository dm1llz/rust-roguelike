use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub bg: RGB,
    pub fg: RGB,
    pub glyph: rltk::FontCharType,
}

#[derive(Component)]
pub struct Viewshed {
    pub dirty: bool,
    pub range: i32,
    pub visible_tiles: Vec<rltk::Point>,
}
