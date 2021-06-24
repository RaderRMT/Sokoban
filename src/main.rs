mod level;
mod player;
mod replay;

fn main() {
    let mut level_counter = 0;

    let mut level = level::read_level(&level_counter);
    let mut replay = replay::new();

    let mut running: bool = true;
    let mut step_counter: u32 = 0;

    while running {
        cls();

        // if we win, we change level
        if level.box_left == 0 {
            level.draw_level();
            println!("Well played! You completed this level in {} steps!\n", step_counter);
            println!("Press Enter to continue\n");

            input();

            level_counter += 1;
            level = level::read_level(&level_counter);
            step_counter = 0;

            replay.clear();
            continue;
        }

        level.draw_level();
        println!("Steps: {}\n", step_counter);
        
        player::move_player(&mut running, &mut step_counter, &mut level, &mut replay, to_vec(input()));
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