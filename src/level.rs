use super::player;
use std::fs;

pub const WALL             : char = '#';
pub const BOX              : char = '$';
pub const BOX_ON_GOAL      : char = '*';
pub const GOAL             : char = '.';
pub const FLOOR            : char = ' ';

pub struct Level {
    pub width: u32,
    pub height: u32,
    pub box_left: u32,
    pub terrain_size: usize,
    pub terrain: Vec<char>,
    pub player: player::Player,
}

impl Level {
    fn get_terrain_index(&self, x: i32, y: i32) -> usize {
        let mut terrain_x = 0;
        let mut terrain_y = 0;

        for i in 0..self.terrain_size {
            if self.terrain[i] == '\n' {
                terrain_x = 0;
                terrain_y += 1;
                continue;
            }

            if x == terrain_x && y == terrain_y {
                return i
            }

            terrain_x += 1;
        }

        0
    }

    pub fn get_tile_at(&self, x: i32, y: i32) -> char {
        self.terrain[self.get_terrain_index(x, y)]
    }

    pub fn set_tile_at(&mut self, x: i32, y: i32, new_tile: char) {
        let index = self.get_terrain_index(x, y);

        self.terrain[index] = new_tile;
    }

    pub fn draw_level(&self) {
        let mut x = 0;
        let mut y = 0;

        for i in 0..self.terrain_size {
            if self.terrain[i] == '\n' {
                x = 0;
                y += 1;
                print!("\n");
                continue;
            }

            if x == self.player.x && y == self.player.y {
                print!("{}", self.player.player_char);
            } else {
                print!("{}", self.terrain[i]);
            }

            x += 1;
        }

        print!("\n");
    }
}

pub fn read_level(level_id: &u32) -> Level {
    let contents = fs::read_to_string(format!("levels/level_{}", level_id))
        .expect("Something went wrong reading the file");

    let mut player_char: char = player::PLAYER;

    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;

    let mut current_width = 0;
    let mut box_counter = 0;

    let mut width: u32 = 0;
    let mut height: u32 = 0;

    let mut terrain_vec = Vec::new();

    for character in contents.chars() {
        let mut current_char: char = character;

        current_width += 1;

        // count width
        if current_width > width {
            width = current_width;
        }

        match current_char {
            player::PLAYER_ON_GOAL | player::PLAYER => { // setting user char & starting position
                current_char = FLOOR;
                if current_char == player::PLAYER_ON_GOAL {
                    current_char = GOAL;
                    player_char = player::PLAYER_ON_GOAL;
                }

                start_x = (current_width - 1) as i32;
                start_y = height as i32;
            }

            BOX => { // count the number of boxes
                box_counter += 1;
            }

            '\n' => { // count the height of the map
                height += 1;
                current_width = 0;
            }

            _ => {} // ignore any useless chars
        }

        terrain_vec.push(current_char);
    }

    let player = player::Player {
        x: start_x,
        y: start_y,
        player_char: player_char
    };

    let level = Level {
        width: width,
        height: height,
        box_left: box_counter,
        terrain_size: contents.len(),
        terrain: terrain_vec,
        player: player
    };

    level
}