use super::level;
use super::replay;

pub const PLAYER_ON_GOAL   : char = '+';
pub const PLAYER           : char = '@';

const BOX      : u8 = 0b0001_0000;
const FORWARD  : u8 = 0b0000_1000;
const LEFT     : u8 = 0b0000_0100;
const BACKWARD : u8 = 0b0000_0010;
const RIGHT    : u8 = 0b0000_0001;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub player_char: char
}

pub fn move_player(running: &mut bool, step_counter: &mut u32, level: &mut level::Level, replay: &mut replay::Replay, direction: Vec<char>) {
    let mut new_pos_x = level.player.x as i32;
    let mut new_pos_y = level.player.y as i32;

    for i in 0..direction.len() {
        let mut movement: u8 = 0;

        if level.box_left == 0 {
            return
        }

        match direction[i] {
            'p' => {
                *running = false;
                return
            }

            'a' => {
                if !replay.is_empty() {
                    movement = replay.pop();
    
                    let offset_x = ((movement & LEFT) >> 2) as i32 - (movement & RIGHT) as i32;
                    let offset_y = ((movement & FORWARD) >> 3) as i32 - ((movement & BACKWARD) as i32 >> 1);
    
                    if is_set(movement, BOX) {
                        move_box(level, level.player.x - offset_x, level.player.y - offset_y, offset_x, offset_y);
                    }
    
                    level.player.x += offset_x;
                    level.player.y += offset_y;
    
                    *step_counter -= 1;
    
                    level.player.player_char = PLAYER;
                    if level.get_tile_at(level.player.x, level.player.y) == level::GOAL {
                        level.player.player_char = PLAYER_ON_GOAL;
                    }
                }
                
                continue;
            }

            'z' => {
                new_pos_y -= 1;
                movement |= FORWARD;
            }

            'q' => {
                new_pos_x -= 1;
                movement |= LEFT;
            }

            's' => {
                new_pos_y += 1;
                movement |= BACKWARD;
            }

            'd' => {
                new_pos_x += 1;
                movement |= RIGHT;
            }

            _ => {}
        }

        let tile: char = level.get_tile_at(new_pos_x, new_pos_y);
        match tile {
            level::WALL => {
                new_pos_x = level.player.x;
                new_pos_y = level.player.y;
            }

            level::BOX | level::BOX_ON_GOAL => {
                if !move_box(level, new_pos_x, new_pos_y, new_pos_x - level.player.x, new_pos_y - level.player.y) {
                    new_pos_x = level.player.x;
                    new_pos_y = level.player.y;
                } else {
                    movement |= BOX;
                }
            }

            level::GOAL => {
                level.player.player_char = PLAYER_ON_GOAL;
            }

            level::FLOOR => {
                level.player.player_char = PLAYER;
            }

            _ => {}
        }

        if movement != 0 {
            if level.player.x != new_pos_x || level.player.y != new_pos_y {
                replay.push(movement);
                *step_counter += 1;
            }
        }

        level.player.x = new_pos_x;
        level.player.y = new_pos_y;
    }
}

fn move_box(level: &mut level::Level, box_x: i32, box_y: i32, offset_x: i32, offset_y: i32) -> bool {
    let new_box_x = box_x + offset_x;
    let new_box_y = box_y + offset_y;

    let new_box_pos_tile: char = level.get_tile_at(new_box_x, new_box_y);
    if new_box_pos_tile == level::WALL
            || new_box_pos_tile == level::BOX
            || new_box_pos_tile == level::BOX_ON_GOAL {
        return false
    }

    level.player.player_char = PLAYER;

    let mut box_char: char = level::BOX;
    if new_box_pos_tile == level::GOAL {
        box_char = level::BOX_ON_GOAL;
        level.box_left -= 1;
    }

    let mut old_box_pos_tile = level::FLOOR;
    if level.get_tile_at(box_x, box_y) == level::BOX_ON_GOAL {
        old_box_pos_tile = level::GOAL;
        level.box_left += 1;
        level.player.player_char = PLAYER_ON_GOAL;
    }

    level.set_tile_at(new_box_x, new_box_y, box_char);
    level.set_tile_at(box_x, box_y, old_box_pos_tile);

    true
}

pub fn is_set(value: u8, bit: u8) -> bool {
    value & bit != 0
}