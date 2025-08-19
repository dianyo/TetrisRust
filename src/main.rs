use macroquad::prelude::*;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const BLOCK_SIZE: f32 = 30.0;

type Board = [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

const SHAPES: [&[(i8, i8)]; 7] = [
    &[(0, 0), (1, 0), (0, 1), (1, 1)], // O
    &[(0, 0), (0, 1), (0, 2), (0, 3)], // I
    &[(0, 0), (1, 0), (2, 0), (1, 1)], // T
    &[(0, 0), (1, 0), (1, 1), (2, 1)], // S
    &[(1, 0), (2, 0), (0, 1), (1, 1)], // Z
    &[(0, 0), (0, 1), (0, 2), (1, 2)], // L
    &[(1, 0), (1, 1), (1, 2), (0, 2)], // J
];

const COLORS: [Color; 7] = [
    macroquad::prelude::YELLOW,
    macroquad::prelude::SKYBLUE,
    macroquad::prelude::MAGENTA,
    macroquad::prelude::GREEN,
    macroquad::prelude::RED,
    macroquad::prelude::ORANGE,
    macroquad::prelude::BLUE,
];

#[derive(Clone, Copy)]
struct Piece {
    x: i8,
    y: i8,
    shape_index: usize,
    rotation: u8,
}

impl Piece {
    fn new() -> Self {
        let shape_index = rand::gen_range(0, SHAPES.len());
        Self {
            x: (BOARD_WIDTH / 2) as i8,
            y: 0,
            shape_index,
            rotation: 0,
        }
    }

    fn shape(&self) -> &'static [(i8, i8)] {
        SHAPES[self.shape_index]
    }

    fn color(&self) -> Color {
        COLORS[self.shape_index]
    }

    fn check_collision(&self, board: &Board) -> bool {
        for (x, y) in self.rotated_shape() {
            let x = self.x + x;
            let y = self.y + y;
            if x < 0
                || x >= BOARD_WIDTH as i8
                || y >= BOARD_HEIGHT as i8
                || (y >= 0 && board[y as usize][x as usize] != 0)
            {
                return true;
            }
        }
        false
    }

    fn rotated_shape(&self) -> Vec<(i8, i8)> {
        self.shape()
            .iter()
            .map(|&(x, y)| match self.rotation {
                0 => (x, y),
                1 => (y, -x),
                2 => (-x, -y),
                3 => (-y, x),
                _ => unreachable!(),
            })
            .collect()
    }
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    Paused,
    GameOver,
}

fn clear_lines(board: &mut Board) -> u8 {
    let mut y = BOARD_HEIGHT - 1;
    let mut lines_cleared = 0;

    while y > 0 {
        let mut full = true;
        for x in 0..BOARD_WIDTH {
            if board[y][x] == 0 {
                full = false;
                break;
            }
        }

        if full {
            lines_cleared += 1;
            for y2 in (1..=y).rev() {
                for x in 0..BOARD_WIDTH {
                    board[y2][x] = board[y2 - 1][x];
                }
            }
            for x in 0..BOARD_WIDTH {
                board[0][x] = 0;
            }
        } else {
            y -= 1;
        }
    }
    lines_cleared
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: (BOARD_WIDTH as f32 * BLOCK_SIZE + 200.0) as i32,
        window_height: (BOARD_HEIGHT as f32 * BLOCK_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board: Board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
    let mut current_piece = Piece::new();
    let mut last_update = get_time();
    let mut score = 0;
    let mut game_state = GameState::Playing;
    let board_x_offset = 200.0;

    loop {
        match game_state {
            GameState::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                let mut potential_piece = current_piece;
                if is_key_pressed(KeyCode::Left) {
                    potential_piece.x -= 1;
                }
                if is_key_pressed(KeyCode::Right) {
                    potential_piece.x += 1;
                }
                if is_key_pressed(KeyCode::Up) {
                    potential_piece.rotation = (potential_piece.rotation + 1) % 4;
                }

                if !potential_piece.check_collision(&board) {
                    current_piece = potential_piece;
                }

                if is_key_pressed(KeyCode::Space) {
                    loop {
                        let mut test_piece = current_piece;
                        test_piece.y += 1;
                        if test_piece.check_collision(&board) {
                            break;
                        }
                        current_piece = test_piece;
                    }
                    last_update = get_time() - 1.0;
                }

                if get_time() - last_update > 0.5 {
                    let mut potential_piece = current_piece;
                    potential_piece.y += 1;
                    if potential_piece.check_collision(&board) {
                        for (x, y) in current_piece.rotated_shape() {
                            let x = current_piece.x + x;
                            let y = current_piece.y + y;
                            if y < 0 {
                                game_state = GameState::GameOver;
                                break;
                            }
                            board[y as usize][x as usize] = (current_piece.shape_index + 1) as u8;
                        }
                        if let GameState::GameOver = game_state {
                            continue;
                        }
                        score += clear_lines(&mut board) as u32 * 10;
                        current_piece = Piece::new();
                        if current_piece.check_collision(&board) {
                            game_state = GameState::GameOver;
                        }
                    } else {
                        current_piece = potential_piece;
                    }
                    last_update = get_time();
                }
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Playing;
                }
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    // Restart button
                    if mouse_x > 10.0 && mouse_x < 60.0 && mouse_y > 70.0 && mouse_y < 120.0 {
                        board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
                        current_piece = Piece::new();
                        score = 0;
                        game_state = GameState::Playing;
                    }
                    // Continue button
                    if mouse_x > 70.0 && mouse_x < 120.0 && mouse_y > 70.0 && mouse_y < 120.0 {
                        game_state = GameState::Playing;
                    }
                }
            }
            GameState::GameOver => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    let text = "Restart";
                    let font_size = 30.0;
                    let text_size = measure_text(text, None, font_size as u16, 1.0);
                    let text_x = board_x_offset + (BOARD_WIDTH as f32 * BLOCK_SIZE) / 2.0 - text_size.width / 2.0;
                    let text_y = screen_height() / 2.0 + 50.0;
                    if mouse_x > text_x && mouse_x < text_x + text_size.width && mouse_y > text_y - text_size.height && mouse_y < text_y {
                        board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
                        current_piece = Piece::new();
                        score = 0;
                        game_state = GameState::Playing;
                    }
                }
            }
        }

        clear_background(BLACK);

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let color = if board[y][x] == 0 {
                    WHITE
                } else {
                    COLORS[board[y][x] as usize - 1]
                };
                draw_rectangle(
                    board_x_offset + x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    color,
                );
                draw_rectangle_lines(
                    board_x_offset + x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    2.0,
                    BLACK,
                );
            }
        }

        if game_state != GameState::GameOver {
            for (x, y) in current_piece.rotated_shape() {
                draw_rectangle(
                    board_x_offset + (current_piece.x + x) as f32 * BLOCK_SIZE,
                    (current_piece.y + y) as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    current_piece.color(),
                );
            }
        }

        draw_text(&format!("Score: {}", score), 10.0, 30.0, 30.0, WHITE);

        if game_state == GameState::Playing {
            // Pause button
            draw_rectangle_lines(10.0, 70.0, 50.0, 50.0, 5.0, WHITE);
            draw_line(25.0, 80.0, 25.0, 110.0, 5.0, WHITE);
            draw_line(45.0, 80.0, 45.0, 110.0, 5.0, WHITE);
        }

        if game_state == GameState::Paused {
            // Restart button
            draw_rectangle_lines(10.0, 70.0, 50.0, 50.0, 5.0, WHITE);
            draw_poly_lines(35.0, 95.0, 5, 20.0, 90.0, 5.0, WHITE);
            // Continue button
            draw_rectangle_lines(70.0, 70.0, 50.0, 50.0, 5.0, WHITE);
            draw_triangle_lines(Vec2::new(85.0, 80.0), Vec2::new(85.0, 110.0), Vec2::new(115.0, 95.0), 5.0, WHITE);
        }

        if game_state == GameState::GameOver {
            let text = "Game Over";
            let font_size = 40.0;
            let text_size = measure_text(text, None, font_size as u16, 1.0);
            draw_text(
                text,
                board_x_offset + (BOARD_WIDTH as f32 * BLOCK_SIZE) / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0,
                font_size,
                RED,
            );
            let restart_text = "Restart";
            let restart_font_size = 30.0;
            let restart_text_size = measure_text(restart_text, None, restart_font_size as u16, 1.0);
            draw_text(
                restart_text,
                board_x_offset + (BOARD_WIDTH as f32 * BLOCK_SIZE) / 2.0 - restart_text_size.width / 2.0,
                screen_height() / 2.0 + 50.0,
                restart_font_size,
                WHITE,
            );
        }

        next_frame().await
    }
}
