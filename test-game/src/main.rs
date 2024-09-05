use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    // Init player.
    let mut player = Player { x: 0, y: 0 };
    // Enter loop.
    loop {
        // Make background black.
        clear_background(BLACK);
        // Get player movement.
        let inside_player = match player_movement(&player) {
            // If player moved update thier position.
            Some(p) => p,
            // If not players position = the stored position.
            None => player,
        };
        // Draw player.
        draw_circle(inside_player.x as f32, inside_player.y as f32, 30.0, BLUE);
        // Store players current position.
        player = inside_player;
        // Load next frame.
        next_frame().await;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Holds information about the player.
struct Player {
    x: i32,
    y: i32,
}

/// Basic player movement.
fn player_movement(player: &Player) -> Option<Player> {
    // Create a new player struct.
    let mut p = Player { x: 0, y: 0 };
    // If (UP) is pressed.
    if is_key_down(KeyCode::Up) {
        // Reduce players y value by 1.
        p.y = player.y - 1;
    // If (DOWN) is pressed.
    } else if is_key_down(KeyCode::Down) {
        // Increment players y value by 1.
        p.y = player.y + 1;
    // If (RIGHT) is pressed.
    } else if is_key_down(KeyCode::Right) {
        // Increment players x value by 1.
        p.x = player.x + 1;
    // If (LEFT) is pressed.
    } else if is_key_down(KeyCode::Left) {
        // Reduce players x value by 1.
        p.x = player.x - 1;
    }
    // If player hasen't moved.
    if p.x == 0 && p.y == 0 {
        // Return the player fuc argument.
        None
        // If player moved up or down.
    } else if p.y >= 1 {
        // Copy fuc argument to players x value.
        p.x = player.x;
        // Return player.
        Some(p)
        // If player moved left or right.
    } else if p.x >= 1 {
        // Copy fuc argument to players y value.
        p.y = player.y;
        // Return player.
        Some(p)
        // Else return none for the compiler.
    } else {
        None
    }
}
