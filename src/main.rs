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

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    // Pause button
                    if mouse_x > 10.0 && mouse_x < 60.0 && mouse_y > 70.0 && mouse_y < 120.0 {
                        game_state = GameState::Paused;
                    }
                }

                let mut potential_piece = current_piece;
                if is_key_pressed(KeyCode::Left) {
                    potential_piece.x -= 1;
                }
                if is_key_pressed(KeyCode::Right) {
                    potential_piece.x += 1;
                }
                if is_key_pressed(KeyCode::Up) {
                    let mut rotated_piece = potential_piece;
                    rotated_piece.rotation = (rotated_piece.rotation + 1) % 4;
                    
                    let mut moved = false;
                    for &dx in &[0, 1, -1, 2, -2] {
                        let mut test_piece = rotated_piece;
                        test_piece.x += dx;
                        if !test_piece.check_collision(&board) {
                            potential_piece = test_piece;
                            moved = true;
                            break;
                        }
                    }
                    if !moved {
                        // could not find a valid rotation
                    }
                }

                if !potential_piece.check_collision(&board) {
                    current_piece = potential_piece;
                }

                let drop_speed = if is_key_down(KeyCode::Down) { 0.1 } else { 0.5 };

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

                if get_time() - last_update > drop_speed {
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

                    let popup_width = 200.0;
                    let popup_height = 150.0;
                    let popup_x =
                        board_x_offset + ((BOARD_WIDTH as f32 * BLOCK_SIZE) - popup_width) / 2.0;
                    let popup_y = ((BOARD_HEIGHT as f32 * BLOCK_SIZE) - popup_height) / 2.0;

                    // Restart button area
                    let restart_x = popup_x + 25.0;
                    let restart_y = popup_y + 50.0;
                    if mouse_x > restart_x
                        && mouse_x < restart_x + 50.0
                        && mouse_y > restart_y
                        && mouse_y < restart_y + 50.0
                    {
                        board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
                        current_piece = Piece::new();
                        score = 0;
                        game_state = GameState::Playing;
                    }
                    // Continue button area
                    let continue_x = popup_x + 125.0;
                    let continue_y = popup_y + 50.0;
                    if mouse_x > continue_x
                        && mouse_x < continue_x + 50.0
                        && mouse_y > continue_y
                        && mouse_y < continue_y + 50.0
                    {
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

                    let popup_width = 200.0;
                    let popup_height = 150.0;
                    let popup_x =
                        board_x_offset + ((BOARD_WIDTH as f32 * BLOCK_SIZE) - popup_width) / 2.0;
                    let popup_y = ((BOARD_HEIGHT as f32 * BLOCK_SIZE) - popup_height) / 2.0;

                    let text_x = popup_x + (popup_width - text_size.width) / 2.0;
                    let text_y = popup_y + 100.0;
                    if mouse_x > text_x
                        && mouse_x < text_x + text_size.width
                        && mouse_y > text_y - text_size.height
                        && mouse_y < text_y
                    {
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
            let mut ghost_piece = current_piece;
            loop {
                let mut test_piece = ghost_piece;
                test_piece.y += 1;
                if test_piece.check_collision(&board) {
                    break;
                }
                ghost_piece = test_piece;
            }
            for (x, y) in ghost_piece.rotated_shape() {
                draw_rectangle(
                    board_x_offset + (ghost_piece.x + x) as f32 * BLOCK_SIZE,
                    (ghost_piece.y + y) as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    Color::new(
                        ghost_piece.color().r,
                        ghost_piece.color().g,
                        ghost_piece.color().b,
                        0.2,
                    ),
                );
            }

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
            let popup_width = 200.0;
            let popup_height = 150.0;
            let popup_x = board_x_offset + ((BOARD_WIDTH as f32 * BLOCK_SIZE) - popup_width) / 2.0;
            let popup_y = ((BOARD_HEIGHT as f32 * BLOCK_SIZE) - popup_height) / 2.0;

            draw_rectangle(popup_x, popup_y, popup_width, popup_height, WHITE);
            draw_rectangle_lines(popup_x, popup_y, popup_width, popup_height, 5.0, BLACK);

            // Restart button - "return arrow"
            let cx = popup_x + 50.0;
            let cy = popup_y + 75.0;
            let r = 15.0;
            let thickness = 5.0;
            let angle_start = 0.0f32.to_radians();
            let angle_end = 270.0f32.to_radians();
            let segments = 20;
            for i in 0..segments {
                let angle1 = angle_start + (angle_end - angle_start) * (i as f32 / segments as f32);
                let angle2 =
                    angle_start + (angle_end - angle_start) * ((i + 1) as f32 / segments as f32);
                let x1 = cx + r * angle1.cos();
                let y1 = cy + r * angle1.sin();
                let x2 = cx + r * angle2.cos();
                let y2 = cy + r * angle2.sin();
                draw_line(x1, y1, x2, y2, thickness, BLACK);
            }
            let arrow_tip = Vec2::new(cx, cy - r);
            draw_triangle_lines(
                arrow_tip,
                Vec2::new(arrow_tip.x + 10.0, arrow_tip.y - 10.0),
                Vec2::new(arrow_tip.x + 10.0, arrow_tip.y + 10.0),
                thickness,
                BLACK,
            );
            // Continue button
            draw_triangle_lines(
                Vec2::new(popup_x + 135.0, popup_y + 60.0),
                Vec2::new(popup_x + 135.0, popup_y + 90.0),
                Vec2::new(popup_x + 165.0, popup_y + 75.0),
                5.0,
                BLACK,
            );
        }

        if game_state == GameState::GameOver {
            let popup_width = 200.0;
            let popup_height = 150.0;
            let popup_x = board_x_offset + ((BOARD_WIDTH as f32 * BLOCK_SIZE) - popup_width) / 2.0;
            let popup_y = ((BOARD_HEIGHT as f32 * BLOCK_SIZE) - popup_height) / 2.0;

            draw_rectangle(popup_x, popup_y, popup_width, popup_height, WHITE);
            draw_rectangle_lines(popup_x, popup_y, popup_width, popup_height, 5.0, BLACK);

            let text = "Game Over";
            let font_size = 40.0;
            let text_size = measure_text(text, None, font_size as u16, 1.0);
            let x = popup_x + (popup_width - text_size.width) / 2.0;
            let y = popup_y + 40.0;
            let offset = 1.0;
            // Draw text with a slight offset to create a bold effect
            draw_text(text, x, y, font_size, RED);
            draw_text(text, x + offset, y, font_size, RED);

            let restart_text = "Restart";
            let restart_font_size = 30.0;
            let restart_text_size = measure_text(restart_text, None, restart_font_size as u16, 1.0);
            draw_text(
                restart_text,
                popup_x + (popup_width - restart_text_size.width) / 2.0,
                popup_y + 100.0,
                restart_font_size,
                BLACK,
            );
        }

        next_frame().await
    }
}
