use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
mod map;
mod player;
mod rect;
mod visibility_system;

pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::*;
pub use visibility_system::VisibilitySystem;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Monster>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        let glyph;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => glyph = rltk::to_cp437('g'),
            _ => glyph = rltk::to_cp437('o'),
        }

        gs.ecs
            .create_entity()
            .with(Monster {})
            .with(Position { x, y })
            .with(Renderable {
                bg: RGB::named(rltk::RED),
                fg: RGB::named(rltk::BLACK),
                glyph,
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    }

    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            bg: RGB::named(rltk::BLACK),
            fg: RGB::named(rltk::YELLOW),
            glyph: rltk::to_cp437('@'),
        })
        .with(Viewshed {
            dirty: true,
            range: 6,
            visible_tiles: Vec::new(),
        })
        .build();

    rltk::main_loop(context, gs)
}
