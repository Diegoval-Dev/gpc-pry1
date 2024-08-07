use minifb::{Window, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use crate::player::Player;

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED; // Rotar a la izquierda
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED; // Rotar a la derecha
    }

    let mut new_pos = player.pos.clone();

    if window.is_key_down(Key::Up) {
        new_pos.x += player.a.cos() * MOVE_SPEED; // Mover hacia adelante
        new_pos.y += player.a.sin() * MOVE_SPEED;
    }
    if window.is_key_down(Key::Down) {
        new_pos.x -= player.a.cos() * MOVE_SPEED; // Mover hacia atrás
        new_pos.y -= player.a.sin() * MOVE_SPEED;
    }

    let new_i = (new_pos.x / block_size as f32) as usize;
    let new_j = (new_pos.y / block_size as f32) as usize;

    if maze[new_j][new_i] == ' ' || maze[new_j][new_i] == 'p' || maze[new_j][new_i] == 'g' {
        player.pos = new_pos;
    }
}
