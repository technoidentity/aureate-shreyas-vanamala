use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::f32::consts::PI;

const BOARD_LEFT: f32 = 35.0;
const BOARD_TOP: f32 = 92.0;
const CELL_SIZE: f32 = 60.0;
const PANEL_LEFT: f32 = 675.0;

const LADDERS: [(i32, i32); 7] = [
    (4, 14),
    (9, 31),
    (20, 38),
    (28, 84),
    (40, 59),
    (51, 67),
    (71, 91),
];

const SNAKES: [(i32, i32); 6] = [(17, 7), (54, 34), (62, 19), (87, 24), (95, 75), (99, 78)];

struct Player {
    name: &'static str,
    letter: &'static str,
    color: Color,
    position: i32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake and Ladders".to_owned(),
        window_width: 1100,
        window_height: 740,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut players = [
        Player {
            name: "Red",
            letter: "R",
            color: Color::from_hex(0xe85665),
            position: 1,
        },
        Player {
            name: "Blue",
            letter: "B",
            color: Color::from_hex(0x3b82d0),
            position: 1,
        },
    ];

    let mut current_player = 0;
    let mut dice = 0;
    let mut message = "Click ROLL DICE to begin. Red plays first.".to_string();
    let mut winner = None;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let roll_button = Rect::new(PANEL_LEFT + 32.0, 500.0, 325.0, 60.0);
        let restart_button = Rect::new(PANEL_LEFT + 32.0, 575.0, 325.0, 52.0);
        let clicked = is_mouse_button_pressed(MouseButton::Left);
        let space_pressed = is_key_pressed(KeyCode::Space);
        let mouse = vec2(mouse_position().0, mouse_position().1);

        if winner.is_some() {
            if clicked && restart_button.contains(mouse) {
                restart_game(
                    &mut players,
                    &mut current_player,
                    &mut dice,
                    &mut message,
                    &mut winner,
                );
            }
        } else if (clicked && roll_button.contains(mouse)) || space_pressed {
            take_turn(
                &mut players,
                &mut current_player,
                &mut dice,
                &mut message,
                &mut winner,
            );
        }

        draw_game(
            &players,
            current_player,
            dice,
            &message,
            winner,
            roll_button,
            restart_button,
        );
        next_frame().await;
    }
}

fn take_turn(
    players: &mut [Player; 2],
    current_player: &mut usize,
    dice: &mut i32,
    message: &mut String,
    winner: &mut Option<usize>,
) {
    *dice = gen_range(1, 7);
    let player_index = *current_player;
    let player_name = players[player_index].name;
    let next_position = players[player_index].position + *dice;

    if next_position > 100 {
        *message = format!(
            "{player_name} rolled {}. An exact roll is needed to reach square 100.",
            *dice
        );
        *current_player = (player_index + 1) % players.len();
        return;
    }

    players[player_index].position = next_position;
    let final_position = check_snake_or_ladder(next_position);
    players[player_index].position = final_position;

    if final_position > next_position {
        *message = format!(
            "{player_name} rolled {} and climbed a ladder from {next_position} to {final_position}!",
            *dice
        );
    } else if final_position < next_position {
        *message = format!(
            "{player_name} rolled {} and slid down a snake from {next_position} to {final_position}.",
            *dice
        );
    } else {
        *message = format!(
            "{player_name} rolled {} and moved to square {final_position}.",
            *dice
        );
    }

    if final_position == 100 {
        *winner = Some(player_index);
        *message = format!("{player_name} reached square 100 and wins the game!");
    } else {
        *current_player = (player_index + 1) % players.len();
    }
}

fn check_snake_or_ladder(position: i32) -> i32 {
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

fn restart_game(
    players: &mut [Player; 2],
    current_player: &mut usize,
    dice: &mut i32,
    message: &mut String,
    winner: &mut Option<usize>,
) {
    for player in players {
        player.position = 1;
    }
    *current_player = 0;
    *dice = 0;
    *winner = None;
    *message = "New game started. Red plays first.".to_string();
}

fn draw_game(
    players: &[Player; 2],
    current_player: usize,
    dice: i32,
    message: &str,
    winner: Option<usize>,
    roll_button: Rect,
    restart_button: Rect,
) {
    clear_background(Color::from_hex(0x14213d));
    draw_text("SNAKE & LADDERS", BOARD_LEFT, 46.0, 34.0, WHITE);
    draw_text(
        "A simple Rust graphical game",
        BOARD_LEFT,
        72.0,
        19.0,
        Color::from_hex(0xb9c7e3),
    );

    draw_board(players);
    draw_panel(
        players,
        current_player,
        dice,
        message,
        winner,
        roll_button,
        restart_button,
    );
}

fn draw_board(players: &[Player; 2]) {
    draw_rectangle(
        BOARD_LEFT - 5.0,
        BOARD_TOP - 5.0,
        CELL_SIZE * 10.0 + 10.0,
        CELL_SIZE * 10.0 + 10.0,
        WHITE,
    );

    for square in 1..=100 {
        let cell = square_rect(square);
        let (row, column) = board_coordinates(square);
        let color = if (row + column) % 2 == 0 {
            Color::from_hex(0xfaf3dd)
        } else {
            Color::from_hex(0xdcebc9)
        };

        draw_rectangle(cell.x, cell.y, cell.w, cell.h, color);
        draw_rectangle_lines(
            cell.x,
            cell.y,
            cell.w,
            cell.h,
            1.0,
            Color::from_hex(0x8ea688),
        );
    }

    for (start, end) in LADDERS {
        draw_ladder(start, end);
    }
    for (start, end) in SNAKES {
        draw_snake(start, end);
    }

    for square in 1..=100 {
        let cell = square_rect(square);
        draw_text(
            &square.to_string(),
            cell.x + 5.0,
            cell.y + 17.0,
            15.0,
            Color::from_hex(0x43555d),
        );
    }

    draw_tokens(players);
}

fn board_coordinates(square: i32) -> (i32, i32) {
    let row_from_bottom = (square - 1) / 10;
    let column = if row_from_bottom % 2 == 0 {
        (square - 1) % 10
    } else {
        9 - ((square - 1) % 10)
    };
    (row_from_bottom, column)
}

fn square_rect(square: i32) -> Rect {
    let (row_from_bottom, column) = board_coordinates(square);
    Rect::new(
        BOARD_LEFT + column as f32 * CELL_SIZE,
        BOARD_TOP + (9 - row_from_bottom) as f32 * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE,
    )
}

fn square_center(square: i32) -> Vec2 {
    let cell = square_rect(square);
    vec2(cell.x + cell.w / 2.0, cell.y + cell.h / 2.0)
}

fn draw_ladder(start: i32, end: i32) {
    let bottom = square_center(start);
    let top = square_center(end);
    let direction = top - bottom;
    let side = vec2(
        -direction.y / direction.length() * 8.0,
        direction.x / direction.length() * 8.0,
    );
    let color = Color::from_hex(0xa86d2d);

    draw_line(
        bottom.x + side.x,
        bottom.y + side.y,
        top.x + side.x,
        top.y + side.y,
        4.0,
        color,
    );
    draw_line(
        bottom.x - side.x,
        bottom.y - side.y,
        top.x - side.x,
        top.y - side.y,
        4.0,
        color,
    );

    for step in 1..5 {
        let middle = bottom + direction * (step as f32 / 5.0);
        draw_line(
            middle.x + side.x,
            middle.y + side.y,
            middle.x - side.x,
            middle.y - side.y,
            3.0,
            color,
        );
    }
}

fn draw_snake(start: i32, end: i32) {
    let head = square_center(start);
    let tail = square_center(end);
    let direction = tail - head;
    let side = vec2(
        -direction.y / direction.length(),
        direction.x / direction.length(),
    );
    let color = Color::from_hex(0x865d8c);
    let mut previous = head;

    for part in 1..=24 {
        let progress = part as f32 / 24.0;
        let wave = (progress * PI * 5.0).sin() * 12.0;
        let point = head + direction * progress + side * wave;
        draw_line(previous.x, previous.y, point.x, point.y, 7.0, color);
        previous = point;
    }

    draw_circle(head.x, head.y, 13.0, color);
    draw_circle_lines(head.x, head.y, 13.0, 2.0, Color::from_hex(0x4d3152));
    draw_circle(head.x + side.x * 5.0, head.y + side.y * 5.0, 2.5, WHITE);
    draw_circle(head.x - side.x * 5.0, head.y - side.y * 5.0, 2.5, WHITE);
    draw_circle(tail.x, tail.y, 5.0, color);
}

fn draw_tokens(players: &[Player; 2]) {
    let shared_square = players[0].position == players[1].position;

    for (index, player) in players.iter().enumerate() {
        let offset = if shared_square {
            if index == 0 {
                vec2(-13.0, 0.0)
            } else {
                vec2(13.0, 0.0)
            }
        } else {
            vec2(0.0, 0.0)
        };

        let center = square_center(player.position) + offset;
        draw_circle(center.x, center.y, 16.0, player.color);
        draw_circle_lines(center.x, center.y, 16.0, 3.0, WHITE);
        let width = measure_text(player.letter, None, 19, 1.0).width;
        draw_text(
            player.letter,
            center.x - width / 2.0,
            center.y + 7.0,
            19.0,
            WHITE,
        );
    }
}

fn draw_panel(
    players: &[Player; 2],
    current_player: usize,
    dice: i32,
    message: &str,
    winner: Option<usize>,
    roll_button: Rect,
    restart_button: Rect,
) {
    draw_rectangle(PANEL_LEFT, 92.0, 390.0, 600.0, Color::from_hex(0x22375e));
    draw_rectangle_lines(
        PANEL_LEFT,
        92.0,
        390.0,
        600.0,
        2.0,
        Color::from_hex(0x7794c6),
    );
    draw_text("GAME PANEL", PANEL_LEFT + 28.0, 130.0, 27.0, WHITE);

    let (heading, heading_color) = if let Some(index) = winner {
        (
            format!("{} wins!", players[index].name),
            players[index].color,
        )
    } else {
        (
            format!("{}'s turn", players[current_player].name),
            players[current_player].color,
        )
    };
    draw_text(&heading, PANEL_LEFT + 28.0, 170.0, 29.0, heading_color);

    draw_player_status(players, 0, PANEL_LEFT + 30.0, 210.0);
    draw_player_status(players, 1, PANEL_LEFT + 210.0, 210.0);
    draw_text(
        "DICE",
        PANEL_LEFT + 145.0,
        312.0,
        20.0,
        Color::from_hex(0xcbd8f3),
    );
    draw_dice(PANEL_LEFT + 137.0, 330.0, dice);

    draw_rectangle(
        PANEL_LEFT + 28.0,
        430.0,
        334.0,
        52.0,
        Color::from_hex(0x182949),
    );
    draw_wrapped_text(message, PANEL_LEFT + 42.0, 451.0, 304.0, 17, WHITE);

    if winner.is_none() {
        draw_button(roll_button, "ROLL DICE  (SPACE)", Color::from_hex(0x2d9c6d));
    } else {
        draw_button(roll_button, "GAME COMPLETE", Color::from_hex(0x536683));
        draw_button(restart_button, "PLAY AGAIN", Color::from_hex(0x416fb6));
    }

    draw_text(
        "Ladders move up  •  Snakes slide down  •  Exact 100 wins",
        PANEL_LEFT + 28.0,
        665.0,
        14.0,
        Color::from_hex(0xb9c7e3),
    );
}

fn draw_player_status(players: &[Player; 2], index: usize, x: f32, y: f32) {
    let player = &players[index];
    draw_circle(x + 13.0, y - 7.0, 13.0, player.color);
    draw_text(player.letter, x + 7.0, y, 16.0, WHITE);
    draw_text(player.name, x + 35.0, y, 20.0, WHITE);
    draw_text(
        &format!("Square {}", player.position),
        x + 35.0,
        y + 23.0,
        16.0,
        Color::from_hex(0xcbd8f3),
    );
}

fn draw_dice(x: f32, y: f32, value: i32) {
    draw_rectangle(x, y, 110.0, 80.0, Color::from_hex(0xfffdf7));
    draw_rectangle_lines(x, y, 110.0, 80.0, 3.0, Color::from_hex(0x94a5c9));

    if value == 0 {
        draw_text("?", x + 40.0, y + 58.0, 50.0, Color::from_hex(0x52698f));
        return;
    }

    let color = Color::from_hex(0x23395d);
    let left = x + 25.0;
    let middle = x + 55.0;
    let right = x + 85.0;
    let top = y + 20.0;
    let center = y + 40.0;
    let bottom = y + 60.0;

    match value {
        1 => die_dot(middle, center, color),
        2 => {
            die_dot(left, top, color);
            die_dot(right, bottom, color);
        }
        3 => {
            die_dot(left, top, color);
            die_dot(middle, center, color);
            die_dot(right, bottom, color);
        }
        4 => {
            die_dot(left, top, color);
            die_dot(right, top, color);
            die_dot(left, bottom, color);
            die_dot(right, bottom, color);
        }
        5 => {
            die_dot(left, top, color);
            die_dot(right, top, color);
            die_dot(middle, center, color);
            die_dot(left, bottom, color);
            die_dot(right, bottom, color);
        }
        6 => {
            die_dot(left, top, color);
            die_dot(right, top, color);
            die_dot(left, center, color);
            die_dot(right, center, color);
            die_dot(left, bottom, color);
            die_dot(right, bottom, color);
        }
        _ => {}
    }
}

fn die_dot(x: f32, y: f32, color: Color) {
    draw_circle(x, y, 6.0, color);
}

fn draw_button(button: Rect, label: &str, color: Color) {
    draw_rectangle(button.x, button.y, button.w, button.h, color);
    draw_rectangle_lines(button.x, button.y, button.w, button.h, 2.0, WHITE);
    let text = measure_text(label, None, 21, 1.0);
    draw_text(
        label,
        button.x + (button.w - text.width) / 2.0,
        button.y + (button.h + text.height) / 2.0 - 3.0,
        21.0,
        WHITE,
    );
}

fn draw_wrapped_text(text: &str, x: f32, mut y: f32, width: f32, size: u16, color: Color) {
    let mut line = String::new();

    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };

        if !line.is_empty() && measure_text(&candidate, None, size, 1.0).width > width {
            draw_text(&line, x, y, size as f32, color);
            y += size as f32 + 4.0;
            line = word.to_string();
        } else {
            line = candidate;
        }
    }

    if !line.is_empty() {
        draw_text(&line, x, y, size as f32, color);
    }
}
