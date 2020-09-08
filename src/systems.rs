use super::{InputQueue, Player, Position, Wall, Boxes};
use legion::*;
use legion::world::SubWorld;
use ggez::event::KeyCode;

#[system]
#[read_component(Player)]
#[write_component(Position)]
pub fn player_move(world: &mut SubWorld, #[resource] keys: &mut InputQueue) {
    //Check player destination
    let mut player_query = <(&Player, &mut Position)>::query();
    let (_player, mut player_pos) = player_query.iter_mut(world).next().expect("Expected Player entity");

    let mut destination_pos: Option<(i32, i32)> = None;
    if let Some(key) = keys.keys_pressed.pop() {
        match key {
            KeyCode::Up => destination_pos = Some((player_pos.x, -1)),
            KeyCode::Down => destination_pos = Some((player_pos.x, 1)),
            KeyCode::Left => destination_pos = Some((-1, player_pos.y)),
            KeyCode::Right => destination_pos = Some((1, player_pos.y)),
            _ => (),
        }
    }

    if let Some(position) = destination_pos {
        //Check possible collision with a wall
        let mut wall_query = <(&Wall, &Position)>::query();
        for (_wall, pos) in wall_query.iter_mut(world) {
            if pos.x == position.0 && pos.y == position.1 {
                println!("Collision...");
                break;
            } else {
                println!("No collision. Move is legal.");
                player_pos.x = position.0;
                player_pos.y = position.1;                        
            }
        }
    }
}
