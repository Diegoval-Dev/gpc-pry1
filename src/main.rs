mod framebuffer;
mod maze;
mod player;
mod cast_ray;
mod color;
mod events;

use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::cast_ray::cast_ray;
use crate::color::Color;
use crate::events::process_events;

const MOVE_SPEED: f32 = 2.0;
const ROTATE_SPEED: f32 = 0.05;

fn find_player_start(maze: &Vec<Vec<char>>) -> Option<Vec2> {
    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                return Some(Vec2::new(x as f32, y as f32));
            }
        }
    }
    None
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '-' | '|' => Color::new(0, 0, 0),      // Negro para paredes
        ' ' => Color::new(87,35,100),            // Blanco para caminos
        'p' => Color::new(0, 255, 0),                // Verde para punto de inicio
        'g' => Color::new(255, 0, 0),                // Rojo para la meta
        _ => Color::new(255, 255, 255),              // Blanco por defecto para cualquier otro carácter
    };

    framebuffer.set_current_color(color);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 30; // Tamaño de bloque ajustado

    // Dibuja el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    // Dibuja al jugador
    framebuffer.set_current_color(Color::new(0, 0, 255)); // Azul brillante para el jugador
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_size);
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 30; // Tamaño de bloque ajustado
    let num_rays = framebuffer.width; // Un rayo por cada columna de píxeles
    let hw = framebuffer.width as f32 / 2.0; // Mitad del ancho del framebuffer
    let hh = framebuffer.height as f32 / 2.0; // Mitad de la altura del framebuffer
    let distance_to_projection_plane = 1.0; // Ajustar según se requiera

    framebuffer.set_current_color(Color::new(255, 255, 255)); // Color blanco para los stakes

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // Rayo actual dividido por el total de rayos
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); // Calcula el ángulo del rayo

        let intersect = cast_ray(framebuffer, &maze, player, a, block_size); // Llamada a cast_ray

        let distance_to_wall = intersect.distance;
        if distance_to_wall < 0.001 { continue; } // Ignorar distancias demasiado pequeñas

        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Calcular la posición para dibujar el stake
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

        // Dibujar el stake directamente en el framebuffer
        for y in stake_top..stake_bottom {
            framebuffer.point(i, y); // i ya es usize
        }
    }
}


fn main() {
    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(Color::new(51, 51, 85)); // Color de fondo

    let maze = load_maze("./maze.txt");
    let player_start = find_player_start(&maze).unwrap_or(Vec2::new(1.0, 1.0));
    let mut player = Player {
        pos: player_start * 42.0, // Ajustar la posición inicial con el tamaño de bloque
        a: PI / 3.0,
        fov: PI / 2.0, // Campo de visión ajustado
    };

    let mut mode = "2D"; // Modo inicial

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        process_events(&window, &mut player, &maze, 30); // Pasar maze y block_size a process_events

        if mode == "2D" {
            render2d(&mut framebuffer, &player);
        } else {
            render3d(&mut framebuffer, &player);
        }

        window
            .update_with_buffer(framebuffer.get_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
