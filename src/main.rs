use ansi_term::Color::{self, Green, Red, White, Yellow, RGB};
use std::io::{stdin, Write};

fn main() {
    // Init board
    let board = [[Tile::Empty; 3]; 3];
    println!(
        "{} {}",
        Green.bold().paint(">>>"),
        Green.bold().paint("Welcome to Tic-Tac-Toe!")
    );
    arrow_print("You will be randomly assigned a letter, X or O.", Green);
    arrow_print("The computer will play with the other letter.", Green);
    arrow_print("Input is taken as such: <LETTER><NUMBER>", Green);
    arrow_print("Examples: a3, B1, c2", Green);
    arrow_print("Input 'exit' to quit anytime.", Green);
    arrow_print("The board is as follows:\n", Green);
    draw_board(&board);

    loop {
        // Init board
        let mut board = [[Tile::Empty; 3]; 3];
        let (player_tile, computer_tile) = assign_hands();
        println!(
            "\n{} {}",
            Green.bold().paint(">>>"),
            Green.bold().paint("NEW GAME"),
        );
        println!(
            "{} {} {}",
            Green.bold().paint(">>>"),
            Green.bold().paint("You are playing as: "),
            player_tile.to_string()
        );
        print!("{} Press enter to continue...", Green.bold().paint(">>>"));
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut String::new()).unwrap();
        clear_screen();
        draw_board(&board);

        // Declare "global" variables
        let mut ai_picked = false;
        let mut time_end = std::time::Duration::new(0, 0);
        let ai = [0, 0];
        let mut iterations = 0;

        loop {
            // Depending on who is X, X will go first
            if player_tile == Tile::X {
                let input = take_input(&board);
                board[input[1]][input[0]] = player_tile;
            } else if computer_tile == Tile::X {
                let time_start = std::time::Instant::now();
                iterations = 0;
                let ai = computer_move(&mut board, player_tile, computer_tile, &mut iterations);
                time_end = time_start.elapsed();
                board[ai[1]][ai[0]] = computer_tile;
                ai_picked = true;
                ai_print(board, ai, iterations, time_end);
            }

            // Check if game is over
            let state = check_board(&board, player_tile, computer_tile);
            match state {
                BoardState::Win(tile) => {
                    if tile == player_tile {
                        clear_screen();
                        draw_board(&board);
                        println!("{}", Green.bold().paint(">>> You win!"));
                    } else {
                        clear_screen();
                        draw_board(&board);
                        println!("{}", Red.bold().paint(">>> You lose!"));
                    }
                    break;
                }

                BoardState::Tie => {
                    clear_screen();
                    draw_board(&board);
                    println!("{}", Yellow.bold().paint(">>> Tie!"));
                    break;
                }

                BoardState::Continue => (),
            }

            if player_tile == Tile::O {
                let input = take_input(&board);
                board[input[1]][input[0]] = player_tile;
            } else if computer_tile == Tile::O {
                let time_start = std::time::Instant::now();
                iterations = 0;
                let ai = computer_move(&mut board, player_tile, computer_tile, &mut iterations);
                time_end = time_start.elapsed();
                board[ai[1]][ai[0]] = computer_tile;
                ai_picked = true;
            }

            let state = check_board(&board, player_tile, computer_tile);
            match state {
                BoardState::Win(tile) => {
                    if tile == player_tile {
                        clear_screen();
                        draw_board(&board);
                        println!("{}", Green.bold().paint(">>> You win!"));
                    } else {
                        clear_screen();
                        draw_board(&board);
                        println!("{}", Red.bold().paint(">>> You lose!"));
                    }
                    break;
                }

                BoardState::Tie => {
                    draw_board(&board);
                    println!("{}", Yellow.bold().paint(">>> Tie!"));
                    break;
                }

                BoardState::Continue => (),
            }

            if !ai_picked {
                clear_screen();
                draw_board(&board);
            } else {
                clear_screen();
                draw_board(&board);
                ai_print(board, ai, iterations, time_end);
            }
        }
    }
}

fn ai_print(board: [[Tile; 3]; 3], ai: [usize; 2], iterations: i32, time_end: std::time::Duration) {
    clear_screen();
    draw_board(&board);
    println!(
        "{} {} {} {} {:?} {} {} {}",
        Red.bold().paint(">>>"),
        Red.paint("Computer picked:"),
        index_to_string(ai),
        Red.paint("in"),
        time_end,
        Red.paint("after checking"),
        // Divied by 2 because it checks both player and computer
        White.bold().paint((iterations / 2).to_string()),
        Red.paint("permutations.")
    );
}

fn arrow_print(text: &str, color: Color) {
    let arrows = color.bold().paint(">>>");
    println!("{arrows} {text}");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    X,
    O,
}

impl Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Empty => " ".to_string(),
            Tile::X => "X".to_string(),
            Tile::O => "O".to_string(),
        }
    }
}

fn assign_hands() -> (Tile, Tile) {
    let player_tile = match fastrand::bool() {
        true => Tile::X,
        false => Tile::O,
    };

    let computer_tile = match player_tile {
        Tile::X => Tile::O,
        Tile::O => Tile::X,
        _ => unreachable!(),
    };

    return (player_tile, computer_tile);
}

#[derive(Clone, Copy, PartialEq)]
enum BoardState {
    Win(Tile),
    Tie,
    Continue,
}

fn draw_board(board: &[[Tile; 3]; 3]) {
    println!("      A   B   C");
    for (row_id, row) in board.iter().enumerate() {
        print!("{}\n {}  ", Green.paint("    +---+---+---+"), row_id + 1);
        for &tile in row {
            print!("{}", Green.paint("| "));
            match tile {
                Tile::Empty => print!("  "),
                Tile::X => print!("{}", RGB(255, 165, 0).bold().paint("X ")),
                Tile::O => print!("{}", Red.bold().paint("O ")),
            }
        }
        println!("{}", Green.paint("|"));
    }
    println!("{}", Green.paint("    +---+---+---+"));
}

fn return_error(error: &str, input: &String, board: &[[Tile; 3]; 3]) {
    clear_screen();
    draw_board(board);
    println!("{} {}", Green.bold().paint(">>>"), input);
    match error {
        "length" => {
            arrow_print("Input was not 2 chars long!", Red);
        }

        "non_alpha" => {
            arrow_print("Inputs first index wasn't alphabetic!", Red);
        }

        "non_numeric" => {
            arrow_print("Inputs second index wasn't numeric!", Red);
        }

        "out_of_bounds" => {
            arrow_print("Input was out of bounds!", Red);
        }

        "taken" => {
            arrow_print("Input is occupied!", Red);
        }

        _ => unreachable!(),
    }
}

fn take_input(board: &[[Tile; 3]; 3]) -> [usize; 2] {
    'outer: loop {
        // Get user input
        print!("{} ", Green.bold().paint(">>>"));
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase().to_string();

        if input == "exit" {
            std::process::exit(0);
        }

        // Check for invalid input
        if input.len() != 2 {
            return_error("length", &input, board);
            continue;
        }

        // Look for errors in "valid" input
        for (index, char) in input.chars().enumerate() {
            // Check for non-alphabetic first index
            if index == 0 && !char.is_alphabetic() {
                return_error("non_alpha", &input, board);
                continue 'outer;
            }

            // Check for non-numeric second index
            if index == 1 && !char.is_digit(10) {
                return_error("non_numeric", &input, board);
                continue 'outer;
            }

            // Check for out of bounds
            if index == 0 && char > 'c' {
                return_error("out_of_bounds", &input, board);
                continue 'outer;
            } else if index == 1 && char > '3' {
                return_error("out_of_bounds", &input, board);
                continue 'outer;
            }
        }

        // Check for taken tile
        let coords = convert_input(&input);
        if board[coords[1]][coords[0]] != Tile::Empty {
            return_error("taken", &input, board);
            continue;
        }

        return convert_input(&input);
    }
}

fn convert_input(input: &String) -> [usize; 2] {
    let mut coords = [0; 2];
    for (index, char) in input.chars().enumerate() {
        if index == 0 {
            match char {
                'a' => coords[0] = 0,
                'b' => coords[0] = 1,
                'c' => coords[0] = 2,
                _ => unreachable!(),
            }
        } else if index == 1 {
            match char {
                '1' => coords[1] = 0,
                '2' => coords[1] = 1,
                '3' => coords[1] = 2,
                _ => unreachable!(),
            }
        }
    }
    return coords;
}

fn index_to_string(index: [usize; 2]) -> String {
    let mut string = String::new();
    match index[0] {
        0 => string.push('A'),
        1 => string.push('B'),
        2 => string.push('C'),
        _ => unreachable!(),
    }

    match index[1] {
        0 => string.push('1'),
        1 => string.push('2'),
        2 => string.push('3'),
        _ => unreachable!(),
    }

    return string;
}

fn check_board(board: &[[Tile; 3]; 3], player_tile: Tile, computer_tile: Tile) -> BoardState {
    // Check for rows
    for row in board {
        if row[0] == row[1] && row[1] == row[2] {
            if row[0] == player_tile {
                return BoardState::Win(player_tile);
            } else if row[0] == computer_tile {
                return BoardState::Win(computer_tile);
            }
        }
    }

    // Check for columns
    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            if board[0][col] == player_tile {
                return BoardState::Win(player_tile);
            } else if board[0][col] == computer_tile {
                return BoardState::Win(computer_tile);
            }
        }
    }

    // Check for diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        if board[0][0] == player_tile {
            return BoardState::Win(player_tile);
        } else if board[0][0] == computer_tile {
            return BoardState::Win(computer_tile);
        }
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        if board[0][2] == player_tile {
            return BoardState::Win(player_tile);
        } else if board[0][2] == computer_tile {
            return BoardState::Win(computer_tile);
        }
    }

    // Check for a tie
    let mut full = true;
    for row in board {
        for tile in row {
            if *tile == Tile::Empty {
                full = false;
                break;
            }
        }
    }
    if full {
        return BoardState::Tie;
    } else {
        return BoardState::Continue;
    }
}

fn empty_tiles(board: &[[Tile; 3]; 3]) -> Vec<[usize; 2]> {
    let mut spots = Vec::new();
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            if *tile == Tile::Empty {
                spots.push([col_index, row_index]);
            }
        }
    }

    return spots;
}

fn minimax(
    board: &[[Tile; 3]; 3],
    player_tile: Tile,
    computer_tile: Tile,
    maximizing: bool,
    iterations: &mut i32,
) -> i32 {
    *iterations += 1;
    // AI is minimizer

    let state = check_board(board, player_tile, computer_tile);

    // Terminal endpoints
    if state == BoardState::Win(player_tile) {
        return -1;
    } else if state == BoardState::Win(computer_tile) {
        return 1;
    } else if state == BoardState::Tie {
        return 0;
    }

    // Recursion
    if maximizing {
        let mut best_score = -2;
        for tile in empty_tiles(board) {
            let mut new_board = board.clone();
            new_board[tile[1]][tile[0]] = computer_tile;
            let score = minimax(&new_board, player_tile, computer_tile, false, iterations);
            if score > best_score {
                best_score = score;
            }
        }
        return best_score;
    } else {
        let mut best_score = 2;
        for tile in empty_tiles(board) {
            let mut new_board = board.clone();
            new_board[tile[1]][tile[0]] = player_tile;
            let score = minimax(&new_board, player_tile, computer_tile, true, iterations);
            if score < best_score {
                best_score = score;
            }
        }
        return best_score;
    }
}

fn computer_move(
    board: &mut [[Tile; 3]; 3],
    player_tile: Tile,
    computer_tile: Tile,
    mut iterations: &mut i32,
) -> [usize; 2] {
    let mut best_score = -2;
    let mut best_move = [0; 2];
    for tile in empty_tiles(board) {
        let mut new_board = board.clone();
        new_board[tile[1]][tile[0]] = computer_tile;
        let score = minimax(
            &new_board,
            player_tile,
            computer_tile,
            false,
            &mut iterations,
        );
        if score > best_score {
            best_score = score;
            best_move = tile;
        }
    }
    return best_move;
}
