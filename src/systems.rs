use super::{InputQueue, Player, Position, Wall, Boxes};
use legion::prelude::*;
use ggez::event::KeyCode;

pub fn build_move_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("move_system")
    .read_component::<Player>()
    .write_component::<Position>()
    .write_resource::<InputQueue>()
    .with_query(<(Read<Player>, Write<Position>)>::query())
    .with_query(<(Read<Wall>, Read<Position>)>::query())
    .build(move |_, world, keys, (query_player, query_walls)| {
      let destination_x = 0;
      let destination_y = 0;
      for (_player, mut pos) in query_player.iter_mut(&mut *world) {
        if let Some(key) = keys.keys_pressed.pop() {
          match key {
            KeyCode::Up => pos.y -= 1,
            KeyCode::Down => pos.y += 1,
            KeyCode::Left => pos.x -= 1,
            KeyCode::Right => pos.x += 1,
            _ => (),
          }
        }
      }
    })
}
