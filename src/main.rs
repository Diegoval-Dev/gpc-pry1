mod framebuffer;
mod maze;
mod player;
mod cast_ray;
mod color;
mod events;

use minifb::{Window, WindowOptions, Key, MouseMode};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use std::process::Command;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::cast_ray::cast_ray;
use crate::events::process_events;
use crate::color::Color;

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' | '-' | '|' => Color::new(0, 0, 0),
        ' ' => Color::new(87, 35, 100),
        'p' => Color::new(0, 255, 0),
        'g' => Color::new(255, 0, 0),
        _ => Color::new(255, 255, 255),
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    let color = cell_to_color(cell);
    framebuffer.set_current_color(color);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

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

fn render2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    framebuffer.set_current_color(Color::new(0, 0, 255));
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, player, a, block_size, true);
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;
    let distance_to_projection_plane = 70.0;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);
        let distance_to_wall = intersect.distance;

        if distance_to_wall < 0.001 {
            continue;
        }

        let color = cell_to_color(intersect.impact);
        framebuffer.set_current_color(color);

        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

        for y in stake_top..stake_bottom {
            framebuffer.point(i, y);
        }
    }
}

fn render_fps(framebuffer: &mut Framebuffer, fps: f32) {
    let color = Color::new(255, 255, 0);
    framebuffer.set_current_color(color);

    let digits = format!("{:.2}", fps);
    let start_x = framebuffer.width - digits.len() * 10 - 10;
    let y = 10;

    for (i, digit) in digits.chars().enumerate() {
        draw_digit(framebuffer, start_x + i * 10, y, digit);
    }
}

fn draw_digit(framebuffer: &mut Framebuffer, x: usize, y: usize, digit: char) {
    let digit_map = match digit {
        '0' => [" ### ", "#   #", "#   #", "#   #", " ### "],
        '1' => ["  #  ", " ##  ", "  #  ", "  #  ", " ### "],
        '2' => [" ### ", "#   #", "  ## ", " #   ", "#####"],
        '3' => [" ### ", "#   #", "  ## ", "#   #", " ### "],
        '4' => ["#   #", "#   #", "#####", "    #", "    #"],
        '5' => ["#####", "#    ", "#### ", "    #", "#### "],
        '6' => [" ### ", "#    ", "#### ", "#   #", " ### "],
        '7' => ["#####", "    #", "   # ", "  #  ", " #   "],
        '8' => [" ### ", "#   #", " ### ", "#   #", " ### "],
        '9' => [" ### ", "#   #", " ####", "    #", " ### "],
        '.' => ["     ", "     ", "     ", "  ## ", "  ## "],
        _ => ["     ", "     ", "     ", "     ", "     "],
    };

    for (row, line) in digit_map.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                framebuffer.point(x + col, y + row);
            }
        }
    }
}

fn main() {
  // Ejecutar el script Python para generar el laberinto
  let python_script = "python"; // Asumiendo que se usa python
  let script_path = "maze.py";
  let args = ["10", "10"];

  let output = Command::new(python_script)
      .arg(script_path)
      .args(&args)
      .output()
      .expect("Failed to execute script");

  if !output.status.success() {
      panic!("Failed to generate maze: {:?}", output);
  }

  let window_width = 800;  
  let window_height = 600; 

  let framebuffer_width = 800;
  let framebuffer_height = 600;

  let frame_delay = Duration::from_millis(16);
  let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

  let mut window = Window::new(
      "Rust Graphics - Maze Example",
      window_width,
      window_height,
      WindowOptions::default(),
  ).unwrap();

  framebuffer.set_background_color(Color::new(51, 51, 85));

  let maze = load_maze("./maze.txt");

  let player_start = find_player_start(&maze).unwrap_or(Vec2::new(1.0, 1.0));
  let mut player = Player {
      pos: player_start * 30.0,
      a: PI / 3.0,
      fov: PI / 2.0,
  };

  let block_size = 25;
  let mut mode = "2D";  // Modo inicial
  let mut m_was_down = false;  // Para detectar el cambio de modo
  let mut last_mouse_pos = None;

  let mut last_time = Instant::now();
  let mut frames = 0;
  let mut fps = 0.0;

  while window.is_open() && !window.is_key_down(Key::Escape) {
      framebuffer.clear();

      let m_is_down = window.is_key_down(Key::M);
      if m_is_down && !m_was_down {
          mode = if mode == "2D" { "3D" } else { "2D" };
      }
      m_was_down = m_is_down;

      process_events(&window, &mut player, &maze, block_size, &mut last_mouse_pos);

      if mode == "2D" {
          render2d(&mut framebuffer, &player, &maze, block_size);
      } else {
          render3d(&mut framebuffer, &player, &maze, block_size);
      }

      // Calcular FPS
      frames += 1;
      let now = Instant::now();
      let elapsed = now.duration_since(last_time).as_secs_f32();
      if elapsed >= 1.0 {
          fps = frames as f32 / elapsed; // Calcular FPS exactos
          frames = 0;
          last_time = now;
      }

      render_fps(&mut framebuffer, fps);

      window
          .update_with_buffer(framebuffer.get_buffer(), framebuffer_width, framebuffer_height)
          .unwrap();

      std::thread::sleep(frame_delay);
  }
}
