// A terminal-only Snake and Ladders game using the Rust standard library.
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const LADDERS: [(u8, u8); 7] = [
    (4, 14),
    (9, 31),
    (20, 38),
    (28, 84),
    (40, 59),
    (51, 67),
    (71, 91),
];

const SNAKES: [(u8, u8); 6] = [(17, 7), (54, 34), (62, 19), (87, 24), (95, 75), (99, 78)];

struct Player {
    name: String,
    symbol: char,
    position: u8,
}

fn main() {
    println!("====================================");
    println!("       SNAKE AND LADDERS GAME");
    println!("====================================\n");
    println!("Reach square 100 first. You need an exact roll to win.\n");

    let mut players = [
        Player {
            name: read_name("Enter Red player's name (or press Enter): ", "Red"),
            symbol: 'R',
            position: 0,
        },
        Player {
            name: read_name("Enter Blue player's name (or press Enter): ", "Blue"),
            symbol: 'B',
            position: 0,
        },
    ];

    let mut current_player = 0;

    loop {
        draw_board(&players);
        show_positions(&players);

        println!(
            "\n{} ({})'s turn.",
            players[current_player].name, players[current_player].symbol
        );
        if !wait_for_roll() {
            println!("\nThanks for playing!");
            break;
        }

        let dice = roll_dice();
        println!("{} rolled a {dice}.", players[current_player].name);
        move_player(&mut players[current_player], dice);

        if players[current_player].position == 100 {
            draw_board(&players);
            println!("\n*** {} wins the game! ***", players[current_player].name);
            break;
        }

        current_player = (current_player + 1) % players.len();
    }
}

fn read_name(prompt: &str, default_name: &str) -> String {
    let name = read_input(prompt);

    if name.is_empty() {
        default_name.to_string()
    } else {
        name
    }
}

fn wait_for_roll() -> bool {
    let input = read_input("Press Enter to roll the dice, or type q to quit: ");
    input.to_lowercase() != "q"
}

fn roll_dice() -> u8 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is invalid");

    (time.subsec_nanos() % 6 + 1) as u8
}

fn move_player(player: &mut Player, dice: u8) {
    let next_position = player.position + dice;

    if next_position > 100 {
        println!(
            "You need an exact roll to reach square 100, so you stay on {}.",
            player.position
        );
        return;
    }

    player.position = next_position;
    println!("{} moves to square {}.", player.name, player.position);

    let landing_square = player.position;
    player.position = check_snake_or_ladder(landing_square);

    if player.position > landing_square {
        println!(
            "Ladder! {} climbs to square {}.",
            player.name, player.position
        );
    } else if player.position < landing_square {
        println!(
            "Snake! {} slides to square {}.",
            player.name, player.position
        );
    }
}

fn check_snake_or_ladder(position: u8) -> u8 {
    for (start, end) in LADDERS {
        if position == start {
            return end;
        }
    }

    for (start, end) in SNAKES {
        if position == start {
            return end;
        }
    }

    position
}

fn draw_board(players: &[Player; 2]) {
    println!("\n                       BOARD");
    println!(
        "{} = {}     {} = {}",
        red_token(),
        players[0].name,
        blue_token(),
        players[1].name
    );

    for row in (0..10).rev() {
        for column in 0..10 {
            let square = board_square(row, column);
            let label = cell_label(square, players);
            print!("[{label}]");
        }
        println!();
    }

    println!("Ladders: 4→14, 9→31, 20→38, 28→84, 40→59, 51→67, 71→91");
    println!("Snakes:  17→7, 54→34, 62→19, 87→24, 95→75, 99→78");
}

fn board_square(row_from_bottom: u8, column: u8) -> u8 {
    if row_from_bottom % 2 == 0 {
        row_from_bottom * 10 + column + 1
    } else {
        row_from_bottom * 10 + 10 - column
    }
}

fn cell_label(square: u8, players: &[Player; 2]) -> String {
    if players[0].position == square && players[1].position == square {
        format!("{}{} ", red_token(), blue_token())
    } else if players[0].position == square {
        format!(" {} ", red_token())
    } else if players[1].position == square {
        format!(" {} ", blue_token())
    } else {
        format!("{square:>3}")
    }
}

fn red_token() -> &'static str {
    "\x1b[31m●\x1b[0m"
}

fn blue_token() -> &'static str {
    "\x1b[34m●\x1b[0m"
}

fn show_positions(players: &[Player; 2]) {
    for player in players {
        if player.position == 0 {
            println!(
                "{} ({}) is waiting to enter the board.",
                player.name, player.symbol
            );
        } else {
            let token = if player.symbol == 'R' {
                red_token()
            } else {
                blue_token()
            };
            println!(
                "{} ({token}) is on square {}.",
                player.name, player.position
            );
        }
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
