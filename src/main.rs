mod level;
mod player;
mod replay;

static mut RUNNING: bool = true;
static mut STEP_COUNTER: u32 = 0;

fn main() {
    let mut level_counter = 0;

    let mut level = level::read_level(&level_counter);
    let mut replay = replay::new();

    unsafe {
        while RUNNING {
            cls();
            
            // if we win, we change level
            if level.box_left == 0 {
                level.draw_level();
                println!("Well played! You completed this level in {} steps!\n", STEP_COUNTER);
                println!("Press Enter to continue\n");

                input();

                level_counter += 1;
                level = level::read_level(&level_counter);
                STEP_COUNTER = 0;

                replay.clear();
                continue;
            }

            level.draw_level();
            println!("Steps: {}\n", STEP_COUNTER);
            
            player::move_player(&mut level, &mut replay, to_vec(input()));
        }
    }
}

// get user input
fn input() -> String {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn to_vec(string: String) -> Vec<char> {
    string.chars().collect::<Vec<_>>()
}

fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn stop() {
    unsafe {
        RUNNING = false;
    }
}