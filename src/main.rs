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

#[derive(Clone, Copy)]
struct Piece {
    x: i8,
    y: i8,
    shape: &'static [(i8, i8)],
    rotation: u8,
}

impl Piece {
    fn new() -> Self {
        Self {
            x: (BOARD_WIDTH / 2) as i8,
            y: 0,
            shape: SHAPES[rand::gen_range(0, SHAPES.len())],
            rotation: 0,
        }
    }

    fn check_collision(&self, board: &Board) -> bool {
        for (x, y) in self.rotated_shape() {
            let x = self.x + x;
            let y = self.y + y;
            if x < 0 || x >= BOARD_WIDTH as i8 || y >= BOARD_HEIGHT as i8 || (y >= 0 && board[y as usize][x as usize] == 1) {
                return true;
            }
        }
        false
    }

    fn rotated_shape(&self) -> Vec<(i8, i8)> {
        self.shape
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

fn clear_lines(board: &mut Board) {
    let mut y = BOARD_HEIGHT - 1;
    let mut _lines_cleared = 0;

    while y > 0 {
        let mut full = true;
        for x in 0..BOARD_WIDTH {
            if board[y][x] == 0 {
                full = false;
                break;
            }
        }

        if full {
            _lines_cleared += 1;
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
}

#[macroquad::main("Tetris")]
async fn main() {
    let mut board: Board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
    let mut current_piece = Piece::new();
    let mut last_update = get_time();

    loop {
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

        if get_time() - last_update > 0.5 {
            let mut potential_piece = current_piece;
            potential_piece.y += 1;
            if potential_piece.check_collision(&board) {
                for (x, y) in current_piece.rotated_shape() {
                    let x = current_piece.x + x;
                    let y = current_piece.y + y;
                    if y >= 0 {
                        board[y as usize][x as usize] = 1;
                    }
                }
                clear_lines(&mut board);
                current_piece = Piece::new();
            } else {
                current_piece = potential_piece;
            }
            last_update = get_time();
        }

        clear_background(BLACK);

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                draw_rectangle(
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    if board[y][x] == 1 { GRAY } else { WHITE },
                );
                draw_rectangle_lines(
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    2.0,
                    BLACK,
                );
            }
        }

        for (x, y) in current_piece.rotated_shape() {
            draw_rectangle(
                (current_piece.x + x) as f32 * BLOCK_SIZE,
                (current_piece.y + y) as f32 * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                RED,
            );
        }

        next_frame().await
    }
}
