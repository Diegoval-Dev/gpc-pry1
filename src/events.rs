use minifb::{Window, Key, MouseMode};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use crate::player::Player;
use crate::audio_player::AudioPlayer; 

pub fn process_events(
    window: &Window,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    last_mouse_pos: &mut Option<(f32, f32)>,
    audio_player: &AudioPlayer,  
) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;
    const MOUSE_SENSITIVITY: f32 = 0.015;


    if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Discard) {
        if let Some((last_x, _)) = last_mouse_pos {
            let dx = mouse_x - *last_x;
            player.a += dx * MOUSE_SENSITIVITY;
        }
        *last_mouse_pos = Some((mouse_x, 0.0));
    }

    let mut new_pos = player.pos.clone();

    if window.is_key_down(Key::W) {
        new_pos.x += player.a.cos() * MOVE_SPEED;
        new_pos.y += player.a.sin() * MOVE_SPEED;
    }
    if window.is_key_down(Key::S) {
        new_pos.x -= player.a.cos() * MOVE_SPEED;
        new_pos.y -= player.a.sin() * MOVE_SPEED;
    }

    let new_i = (new_pos.x / block_size as f32) as usize;
    let new_j = (new_pos.y / block_size as f32) as usize;


    if maze[new_j][new_i] == ' ' || maze[new_j][new_i] == 'p' || maze[new_j][new_i] == 'g' {
        player.pos = new_pos;
    }


    if maze[new_j][new_i] == 'g' {
        audio_player.play_sound_effect("win.wav", 2.0);
    }
}

