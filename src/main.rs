use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{DrawParam, Image};
use ggez::nalgebra as na;
use ggez::{conf, event, Context, GameResult};
use legion::prelude::*;
use std::iter;
use std::path;

mod systems;
use systems::build_move_system;
mod level;

// Components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Clone, Debug, PartialEq)]
struct Renderable {
    path: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Wall {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Player {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Boxes {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BoxSpot {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Movable {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Immovable {}

// Resources
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub struct Game {
    pub universe: Universe,
    pub world: World,
    pub resources: Resources,
    pub scheduler: Schedule,
}

impl Game {
    fn new() -> Game {
        let universe = Universe::new();
        let world = universe.create_world();
        let mut resources = Resources::default();
        resources.insert(InputQueue {
            keys_pressed: Vec::new(),
        });

        let scheduler = Schedule::builder()
            .add_system(build_move_system())
            .flush()
            .build();

        Game {
            universe,
            world,
            resources,
            scheduler,
        }
    }

    pub fn create_wall(&mut self, position: Position) {
        self.world.insert(
            (),
            iter::once((
                Renderable {
                    path: "/wall.png".to_string(),
                },
                Position { z: 10, ..position },
                Wall {},
                Immovable {},
            )),
        );
    }

    pub fn create_floor(&mut self, position: Position) {
        self.world.insert(
            (),
            iter::once((
                Renderable {
                    path: "/floor.png".to_string(),
                },
                Position { z: 5, ..position },
            )),
        );
    }

    pub fn create_box(&mut self, position: Position) {
        self.world.insert(
            (),
            iter::once((
                Renderable {
                    path: "/box.png".to_string(),
                },
                Position { z: 10, ..position },
                Boxes {},
                Movable {},
            )),
        );
    }

    pub fn create_box_spot(&mut self, position: Position) {
        self.world.insert(
            (),
            iter::once((
                Renderable {
                    path: "/box_spot.png".to_string(),
                },
                Position { z: 9, ..position },
                BoxSpot {},
            )),
        );
    }

    pub fn create_player(&mut self, position: Position) {
        self.world.insert(
            (),
            iter::once((
                Player {},
                Renderable {
                    path: "/player.png".to_string(),
                },
                Position { z: 10, ..position },
                Movable {},
            )),
        );
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.scheduler.execute(&mut self.world, &mut self.resources);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let query = <(Read<Renderable>, Read<Position>)>::query();
        for (renderable, pos) in query.iter(&mut self.world) {
            let image = Image::new(context, renderable.path.clone()).expect("expected path...");
            let x = pos.x as f32 * 32.0;
            let y = pos.y as f32 * 32.0;

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(context, &image, draw_params).expect("expected render");
        }

        graphics::present(context).expect("Expected to present");

        Ok(())
    }

    fn key_down_event(&mut self, _context: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        let mut input_queue = self.resources.get_mut::<InputQueue>().unwrap();
        input_queue.keys_pressed.push(keycode);
    }
}
fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Rust Sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;

    let game = &mut Game::new();

    level::initialize_level(game);

    event::run(context, event_loop, game)
}
