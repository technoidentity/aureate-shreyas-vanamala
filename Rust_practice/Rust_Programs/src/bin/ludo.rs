use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const BOARD_SIZE: usize = 15;
const TRACK_SIZE: usize = 52;
const HOME_POSITION: i32 = 58;

struct Player {
    name: String,
    color: &'static str,
    symbol: char,
    start: usize,
    tokens: [i32; 4],
}

fn main() {
    println!("=== TRADITIONAL LUDO ===");
    println!("Roll 6 to leave base. A 6 gives another turn.");
    println!("Travel around the white track, then enter your coloured home lane.");
    println!("The first player to bring all four tokens to the centre wins.\n");

    let mut players = [
        make_player("Red", '●', 39),
        make_player("Green", '▲', 13),
        make_player("Yellow", '◆', 0),
        make_player("Blue", '■', 26),
    ];

    for player in &mut players {
        let prompt = format!("Enter {} player's name: ", player.color);
        let name = read_input(&prompt);
        if name.is_empty() {
            player.name = player.color.to_string();
        } else {
            player.name = name;
        }
    }

    let track = build_track();
    let mut current_player = 0;
    let mut game_over = false;

    while !game_over {
        draw_board(&players, &track);
        show_players(&players);

        println!(
            "\n{} {} ({})'s turn.",
            players[current_player].symbol,
            players[current_player].name,
            players[current_player].color
        );

        let input = read_input("Press Enter to roll, or q to quit: ");
        if input.to_lowercase() == "q" {
            println!("Game ended.");
            break;
        }
        if !input.is_empty() {
            println!("Press Enter without typing a number. The game rolls the dice.");
            continue;
        }

        let dice = roll_dice();
        println!("{} rolled a {}.", players[current_player].name, dice);

        if !player_can_move(&players[current_player], dice) {
            println!("No token can move.");
            if dice != 6 {
                current_player = (current_player + 1) % 4;
            } else {
                println!("You rolled a 6, so you get another turn.");
            }
            continue;
        }

        let token_number = choose_token(&players[current_player], dice);
        let old_position = players[current_player].tokens[token_number];
        let new_position;

        if old_position == 0 {
            new_position = 1;
            println!(
                "Token {} leaves the base and enters its start square.",
                token_number + 1
            );
        } else {
            new_position = old_position + dice;
            println!("Token {} moves {} spaces.", token_number + 1, dice);
        }

        players[current_player].tokens[token_number] = new_position;

        if new_position == HOME_POSITION {
            println!("Token {} reached the centre!", token_number + 1);
        } else if new_position <= TRACK_SIZE as i32 {
            capture_tokens(&mut players, current_player, new_position);
        }

        if player_has_won(&players[current_player]) {
            draw_board(&players, &track);
            println!(
                "\n{} {} wins! All four tokens reached the centre.",
                players[current_player].symbol, players[current_player].name
            );
            game_over = true;
        } else if dice != 6 {
            current_player = (current_player + 1) % 4;
        } else {
            println!("You rolled a 6, so you get another turn.");
        }
    }
}

fn make_player(color: &'static str, symbol: char, start: usize) -> Player {
    Player {
        name: String::new(),
        color,
        symbol,
        start,
        tokens: [0; 4],
    }
}

fn build_track() -> [(usize, usize); TRACK_SIZE] {
    [
        (6, 1),
        (6, 2),
        (6, 3),
        (6, 4),
        (6, 5),
        (5, 6),
        (4, 6),
        (3, 6),
        (2, 6),
        (1, 6),
        (0, 6),
        (0, 7),
        (0, 8),
        (1, 8),
        (2, 8),
        (3, 8),
        (4, 8),
        (5, 8),
        (6, 9),
        (6, 10),
        (6, 11),
        (6, 12),
        (6, 13),
        (6, 14),
        (7, 14),
        (8, 14),
        (8, 13),
        (8, 12),
        (8, 11),
        (8, 10),
        (8, 9),
        (9, 8),
        (10, 8),
        (11, 8),
        (12, 8),
        (13, 8),
        (14, 8),
        (14, 7),
        (14, 6),
        (13, 6),
        (12, 6),
        (11, 6),
        (10, 6),
        (9, 6),
        (8, 5),
        (8, 4),
        (8, 3),
        (8, 2),
        (8, 1),
        (8, 0),
        (7, 0),
        (6, 0),
    ]
}

fn roll_dice() -> i32 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is before the Unix epoch");
    (time.subsec_nanos() % 6 + 1) as i32
}

fn player_can_move(player: &Player, dice: i32) -> bool {
    for token in 0..4 {
        if token_can_move(player, token, dice) {
            return true;
        }
    }
    false
}

fn token_can_move(player: &Player, token: usize, dice: i32) -> bool {
    let position = player.tokens[token];

    if position == 0 {
        return dice == 6;
    }
    if position == HOME_POSITION {
        return false;
    }

    position + dice <= HOME_POSITION
}

fn choose_token(player: &Player, dice: i32) -> usize {
    loop {
        println!("Choose a token to move:");
        for token in 0..4 {
            if token_can_move(player, token, dice) {
                println!("{}. Token {}", token + 1, token + 1);
            }
        }

        let choice = read_input("Token number: ");
        if choice == "1" && token_can_move(player, 0, dice) {
            return 0;
        } else if choice == "2" && token_can_move(player, 1, dice) {
            return 1;
        } else if choice == "3" && token_can_move(player, 2, dice) {
            return 2;
        } else if choice == "4" && token_can_move(player, 3, dice) {
            return 3;
        }

        println!("Choose one of the token numbers shown.");
    }
}

fn capture_tokens(players: &mut [Player; 4], current_player: usize, position: i32) {
    let track_position = (players[current_player].start + position as usize - 1) % TRACK_SIZE;

    if is_safe_square(players, track_position) {
        return;
    }

    for other_player in 0..4 {
        if other_player == current_player {
            continue;
        }

        let other_start = players[other_player].start;
        for token in 0..4 {
            let other_position = players[other_player].tokens[token];
            if other_position > 0 && other_position <= TRACK_SIZE as i32 {
                let other_track_position = (other_start + other_position as usize - 1) % TRACK_SIZE;
                if other_track_position == track_position {
                    players[other_player].tokens[token] = 0;
                    println!(
                        "{} captured {}'s token {}! It returns to base.",
                        players[current_player].name,
                        players[other_player].name,
                        token + 1
                    );
                }
            }
        }
    }
}

fn is_safe_square(players: &[Player; 4], track_position: usize) -> bool {
    if track_position == 8 || track_position == 21 || track_position == 34 || track_position == 47 {
        return true;
    }

    for player in players {
        if player.start == track_position {
            return true;
        }
    }
    false
}

fn player_has_won(player: &Player) -> bool {
    for token in 0..4 {
        if player.tokens[token] != HOME_POSITION {
            return false;
        }
    }
    true
}

fn home_lane_cell(player: usize, progress: i32) -> (usize, usize) {
    let step = (progress - 53) as usize;

    if player == 0 {
        (13 - step, 7)
    } else if player == 1 {
        (step + 1, 7)
    } else if player == 2 {
        (7, step + 1)
    } else {
        (7, 13 - step)
    }
}

fn final_home_cell(player: usize) -> (usize, usize) {
    if player == 0 {
        (8, 7)
    } else if player == 1 {
        (6, 7)
    } else if player == 2 {
        (7, 6)
    } else {
        (7, 8)
    }
}

fn base_cell(player: usize, token: usize) -> (usize, usize) {
    let base_cells = [
        [(10, 2), (10, 4), (12, 2), (12, 4)],
        [(2, 10), (2, 12), (4, 10), (4, 12)],
        [(2, 2), (2, 4), (4, 2), (4, 4)],
        [(10, 10), (10, 12), (12, 10), (12, 12)],
    ];

    base_cells[player][token]
}

fn base_token_symbol(player: usize) -> char {
    if player == 0 {
        '●'
    } else if player == 1 {
        '▲'
    } else if player == 2 {
        '◆'
    } else {
        '■'
    }
}

fn draw_board(players: &[Player; 4], track: &[(usize, usize); TRACK_SIZE]) {
    let mut board = [[' '; BOARD_SIZE]; BOARD_SIZE];

    draw_base_outline(&mut board, 0, 0);
    draw_base_outline(&mut board, 0, 9);
    draw_base_outline(&mut board, 9, 0);
    draw_base_outline(&mut board, 9, 9);

    for (row, col) in track {
        board[*row][*col] = '.';
    }

    for safe_square in [8, 21, 34, 47] {
        let (row, col) = track[safe_square];
        board[row][col] = '*';
    }

    let start_symbols = ['R', 'G', 'Y', 'B'];
    for player in 0..4 {
        let (row, col) = track[players[player].start];
        board[row][col] = start_symbols[player];
    }

    for step in 0..5 {
        board[13 - step][7] = 'r';
        board[step + 1][7] = 'g';
        board[7][step + 1] = 'y';
        board[7][13 - step] = 'b';
    }

    for row in 6..9 {
        for col in 6..9 {
            board[row][col] = 'H';
        }
    }

    for player in 0..4 {
        for token in 0..4 {
            let progress = players[player].tokens[token];

            if progress == 0 {
                let (row, col) = base_cell(player, token);
                board[row][col] = base_token_symbol(player);
            } else if progress <= TRACK_SIZE as i32 {
                let track_position = (players[player].start + progress as usize - 1) % TRACK_SIZE;
                let (row, col) = track[track_position];
                board[row][col] = players[player].symbol;
            } else if progress < HOME_POSITION {
                let (row, col) = home_lane_cell(player, progress);
                board[row][col] = players[player].symbol;
            } else {
                let (row, col) = final_home_cell(player);
                board[row][col] = players[player].symbol;
            }
        }
    }

    println!("\n                         LUDO BOARD");
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            print_cell(board[row][col]);
        }
        println!();
    }
    println!("\nTokens: Red ●   Green ▲   Yellow ◆   Blue ■");
    println!("Starts: Red ↑   Green ↓   Yellow →   Blue ←");
    println!("Board:  ✦ safe square   coloured □ home lane   ★ centre");
}

fn draw_base_outline(
    board: &mut [[char; BOARD_SIZE]; BOARD_SIZE],
    row_start: usize,
    col_start: usize,
) {
    board[row_start][col_start] = 'q';
    board[row_start][col_start + 5] = 'w';
    board[row_start + 5][col_start] = 'x';
    board[row_start + 5][col_start + 5] = 'z';

    for col in col_start + 1..col_start + 5 {
        board[row_start][col] = '-';
        board[row_start + 5][col] = '-';
    }
    for row in row_start + 1..row_start + 5 {
        board[row][col_start] = '|';
        board[row][col_start + 5] = '|';
    }
}

fn print_cell(cell: char) {
    if cell == '●' {
        print!("\x1b[31m●\x1b[0m ");
    } else if cell == '▲' {
        print!("\x1b[32m▲\x1b[0m ");
    } else if cell == '◆' {
        print!("\x1b[33m◆\x1b[0m ");
    } else if cell == '■' {
        print!("\x1b[34m■\x1b[0m ");
    } else if cell == 'q' {
        print!("┌ ");
    } else if cell == 'w' {
        print!("┐ ");
    } else if cell == 'x' {
        print!("└ ");
    } else if cell == 'z' {
        print!("┘ ");
    } else if cell == '-' {
        print!("──");
    } else if cell == '|' {
        print!("│ ");
    } else if cell == 'r' {
        print!("\x1b[31m□\x1b[0m ");
    } else if cell == 'g' {
        print!("\x1b[32m□\x1b[0m ");
    } else if cell == 'y' {
        print!("\x1b[33m□\x1b[0m ");
    } else if cell == 'b' {
        print!("\x1b[34m□\x1b[0m ");
    } else if cell == 'R' {
        print!("\x1b[31m↑\x1b[0m ");
    } else if cell == 'G' {
        print!("\x1b[32m↓\x1b[0m ");
    } else if cell == 'Y' {
        print!("\x1b[33m→\x1b[0m ");
    } else if cell == 'B' {
        print!("\x1b[34m←\x1b[0m ");
    } else if cell == '*' {
        print!("✦ ");
    } else if cell == 'H' {
        print!("★ ");
    } else if cell == '.' {
        print!("\x1b[37m□\x1b[0m ");
    } else {
        print!("  ");
    }
}

fn show_players(players: &[Player; 4]) {
    for player in players {
        let mut in_base = 0;
        let mut at_home = 0;

        for token in 0..4 {
            if player.tokens[token] == 0 {
                in_base += 1;
            } else if player.tokens[token] == HOME_POSITION {
                at_home += 1;
            }
        }

        println!(
            "{} {} ({}) - in base: {}, home: {}",
            player.symbol, player.name, player.color, in_base, at_home
        );
    }
}

fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    input.trim().to_string()
}
