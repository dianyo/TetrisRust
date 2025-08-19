# A Vibe Coding Rust Tetris

This is a classic Tetris game built from scratch in Rust using the `macroquad` game engine. It was developed as a fun, "vibe coding" project to explore game development in Rust.

![Tetris Screenshot](Tetris.png)

## Features

*   **Classic Tetris Gameplay:** Move, rotate, and drop tetrominoes to clear lines and score points.
*   **Super Rotation System (SRS):** A fully compliant wall kick and rotation system for fluid, modern piece movement.
*   **Ghost Piece:** A transparent projection shows exactly where the current piece will land.
*   **Soft & Hard Drops:** Speed up the descent with the **down arrow** or instantly drop the piece with the **spacebar**.
*   **Scoring System:** Keep track of your score as you clear lines.
*   **Pause & Restart:** Pause the game at any time with **Escape** or the pause button. Restart from the pause menu or the game over screen.
*   **Custom UI:** A clean interface with a dedicated score panel and intuitive pop-up menus.

## Tested Environment

*   **OS:** macOS 15.3.1
*   **CPU:** Apple M2 Pro
*   **Rust:** rustc 1.91.0-nightly (9eb4a2652 2025-08-18)

## Installation & Running

This project requires a specific **nightly** Rust toolchain to run.

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/your-username/TetrisRust.git
    cd TetrisRust
    ```
    *(Please replace the URL with your actual repository URL.)*

2.  **Install the correct nightly toolchain:**
    If you don't have the exact nightly toolchain installed, run the following command:
    ```sh
    rustup toolchain install nightly-2025-08-18
    ```

3.  **Set the nightly toolchain for this project:**
    You can set the correct nightly version for this specific project by running:
    ```sh
    rustup override set nightly-2025-08-18
    ```

4.  **Run the game:**
    ```sh
    cargo run
    ```
