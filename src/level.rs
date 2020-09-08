use super::{Game, Position};

pub fn initialize_level(game: &mut Game) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_level(game, MAP.to_string());
}

pub fn load_level(game: &mut Game, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as i32,
                y: y as i32,
                z: 0,
            };

            match *column {
                "." => game.create_floor(position),
                "W" => {
                    game.create_floor(position);
                    game.create_wall(position);
                }
                "P" => {
                    game.create_floor(position);
                    game.create_player(position);
                }
                "B" => {
                    game.create_floor(position);
                    game.create_box(position);
                }
                "S" => {
                    game.create_floor(position);
                    game.create_box_spot(position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
