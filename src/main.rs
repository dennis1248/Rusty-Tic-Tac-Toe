// Prints play field
fn print_board(cells: &mut [char; 9]) {
    println!("{0}|{1}|{2}\n-----\n{3}|{4}|{5}\n-----\n{6}|{7}|{8}",
            cells[0],
            cells[1],
            cells[2],
            cells[3],
            cells[4],
            cells[5],
            cells[6],
            cells[7],
            cells[8]);
}

// Player game loop
// Player X and O will in turn claim a cell
// TODO: Rewite as loop?
fn player_move(player: bool, cells: &mut [char; 9]) {
    let mut line = String::new();
    let current_player;
    
    if player {
        current_player = 'X';
    } else {
        current_player = 'O';
    }

    println!("Player {0} is now in play...", current_player);
    print_board(cells);
    println!("Choose a cell (1-9)...");
    
    std::io::stdin()
        .read_line(&mut line)
        .unwrap();
    
    let line_out = line.chars().nth(0).unwrap();

    // Write player char to player board
    match line_out {
        '1' => cells[0] = current_player,
        '2' => cells[1] = current_player,
        '3' => cells[2] = current_player,
        '4' => cells[3] = current_player,
        '5' => cells[4] = current_player,
        '6' => cells[5] = current_player,
        '7' => cells[6] = current_player,
        '8' => cells[7] = current_player,
        '9' => cells[8] = current_player,
        _ => println!("Invalid, selection"),
    };
}

// Check if player wins game, if true returns true
// triggering game_state to quit game and announce winner
fn check_for_win_state(player: bool, cells: &mut [char; 9]) -> bool {
    let current_player;
    let mut win_state = false;
    
    if player {
        current_player = 'X';
    } else {
        current_player = 'O';
    }

    // Check for wine states
    // Only 8 possible win states, no reason not to hard-code
    if cells[0] == current_player && cells[1] == current_player && cells[2] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    } else if cells[3] == current_player && cells[4] == current_player && cells[5] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    } else if cells[6] == current_player && cells[7] == current_player && cells[8] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    } else if cells[0] == current_player && cells[3] == current_player && cells[6] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    } else if cells[1] == current_player && cells[4] == current_player && cells[7] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    } else if cells[2] == current_player && cells[5] == current_player && cells[8] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    } else if cells[0] == current_player && cells[4] == current_player && cells[8] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    } else if cells[2] == current_player && cells[4] == current_player && cells[6] == current_player {
        println!("Player {0} has won the game!", current_player);
        win_state = true;
    }

    win_state
}

// Passes current player and gives back other play
// X = true
// O = false
fn give_turn_to_next_player(mut player: bool) -> bool {
    // Give turn to next player
    if player {
        player = false;
    } else {
        player = true;
    }
    
    player
}

fn main() {    
    // X = true
    // O = false
    let mut player = true;
    let mut win_state;
    
    // Define game play field
    let mut cells: [char; 9] = ['1','2','3','4','5','6','7','8','9'];
    
    // Main game loop, will continue until game_state returns true to confirm win
    loop {
        player_move(player, &mut cells);
        win_state = check_for_win_state(player, &mut cells);
        
        // Check for win, announce win and stop game if true
        if win_state {
            print_board(&mut cells);
            break;
        }
        
        player = give_turn_to_next_player(player);
    }
}
