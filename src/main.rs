mod framebuffer;
mod maze;
mod player;
mod cast_ray;
mod color;
mod events;
mod audio_player;

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
use image::GenericImageView;
use crate::audio_player::AudioPlayer;


fn load_texture(file_path: &str) -> Vec<u32> {
  let img = image::open(file_path).expect("Failed to load texture");
  let (width, height) = img.dimensions();
  let mut texture = Vec::new();

  for y in 0..height {
      for x in 0..width {
          let pixel = img.get_pixel(x, y);
          let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
          texture.push(color);
      }
  }

  texture
}


fn cell_to_color(cell: char) -> Color {
  match cell {
      '+' | '-' | '|' => Color::new(0, 0, 0),
      ' ' => Color::new(87, 35, 100),
      'p' => Color::new(0, 255, 0), 
      'g' => Color::new(255, 0, 0), 
      _ => Color::new(255, 255, 255),
  }
}

fn draw_cell_with_texture(
  framebuffer: &mut Framebuffer,
  xo: usize,
  yo: usize,
  block_size: usize,
  texture: &[u32],
  texture_width: usize,
  texture_height: usize,
) {
  for x in 0..block_size {
      for y in 0..block_size {
          let tx = (x * texture_width / block_size) % texture_width;
          let ty = (y * texture_height / block_size) % texture_height;
          let color = texture[ty * texture_width + tx];
          framebuffer.point(xo + x, yo + y, color);
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

fn render2d(
  framebuffer: &mut Framebuffer,
  player: &Player,
  maze: &Vec<Vec<char>>,
  block_size: usize,
  wall_texture_1: &[u32],
  wall_texture_2: &[u32],
  wall_texture_3: &[u32],
  texture_width: usize,
  texture_height: usize,
) {
  for row in 0..maze.len() {
      for col in 0..maze[row].len() {
          let cell = maze[row][col];
          match cell {
              '+' => draw_cell_with_texture(
                  framebuffer,
                  col * block_size,
                  row * block_size,
                  block_size,
                  wall_texture_1,
                  texture_width,
                  texture_height,
              ),
              '-' => draw_cell_with_texture(
                  framebuffer,
                  col * block_size,
                  row * block_size,
                  block_size,
                  wall_texture_2,
                  texture_width,
                  texture_height,
              ),
              '|' => draw_cell_with_texture(
                  framebuffer,
                  col * block_size,
                  row * block_size,
                  block_size,
                  wall_texture_3,
                  texture_width,
                  texture_height,
              ),
              'p' | 'g' => {
                  let color = cell_to_color(cell);
                  framebuffer.set_current_color(color);
                  for x in 0..block_size {
                      for y in 0..block_size {
                          framebuffer.point(col * block_size + x, row * block_size + y, framebuffer.current_color.to_hex());
                      }
                  }
              }
              _ => {} 
          }
      }
  }


  framebuffer.set_current_color(Color::new(255, 255, 0));
  framebuffer.point(player.pos.x as usize, player.pos.y as usize, Color::new(255, 255, 0).to_hex());


  let num_rays = 5;
  for i in 0..num_rays {
      let current_ray = i as f32 / num_rays as f32;
      let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
      cast_ray(framebuffer, &maze, player, a, block_size, true);
  }
}


fn render3d(
  framebuffer: &mut Framebuffer,
  player: &Player,
  maze: &Vec<Vec<char>>,
  block_size: usize,
  wall_texture_1: &[u32],
  wall_texture_2: &[u32],
  wall_texture_3: &[u32],
  texture_width: usize,
  texture_height: usize,
) {
  let num_rays = framebuffer.width;
  let hh = framebuffer.height as f32 / 2.0;
  let distance_to_projection_plane = 60.0;

  for i in 0..num_rays {
      let current_ray = i as f32 / num_rays as f32;
      let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

      let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);
      let distance_to_wall = intersect.distance;

      if distance_to_wall < 0.001 {
          continue;
      }

      match intersect.impact {
          '+' => apply_texture(framebuffer, i, hh, distance_to_wall, distance_to_projection_plane, wall_texture_1, texture_width, texture_height),
          '-' => apply_texture(framebuffer, i, hh, distance_to_wall, distance_to_projection_plane, wall_texture_2, texture_width, texture_height),
          '|' => apply_texture(framebuffer, i, hh, distance_to_wall, distance_to_projection_plane, wall_texture_3, texture_width, texture_height),
          'p' => {
              framebuffer.set_current_color(Color::new(0, 255, 0));
              draw_stake(framebuffer, i, hh, distance_to_wall, distance_to_projection_plane);
          }
          'g' => {
              framebuffer.set_current_color(Color::new(255, 0, 0));
              draw_stake(framebuffer, i, hh, distance_to_wall, distance_to_projection_plane);
          }
          _ => continue, 
      }
  }
}

fn apply_texture(
    framebuffer: &mut Framebuffer,
    i: usize,
    hh: f32,
    distance_to_wall: f32,
    distance_to_projection_plane: f32,
    texture: &[u32],
    texture_width: usize,
    texture_height: usize,
) {
    // Ensure that the x-coordinate (i) is within the framebuffer bounds
    if i >= framebuffer.width {
        return;
    }

    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
    let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

    for y in stake_top..stake_bottom {
        // Ensure that the y-coordinate is within the framebuffer bounds
        if y >= framebuffer.height {
            continue;
        }

        let tex_x = (i as f32 / framebuffer.width as f32 * texture_width as f32) as usize;
        let tex_y = ((y - stake_top) as f32 / (stake_bottom - stake_top) as f32 * texture_height as f32) as usize;

        // Ensure tex_x and tex_y are within texture bounds
        if tex_x < texture_width && tex_y < texture_height {
            let color_index = tex_y * texture_width + tex_x;
            if color_index < texture.len() {
                let color = texture[color_index];
                framebuffer.point(i, y, color);
            }
        }
    }
}


fn draw_stake(
  framebuffer: &mut Framebuffer,
  i: usize,
  hh: f32,
  distance_to_wall: f32,
  distance_to_projection_plane: f32,
) {
  let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
  let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
  let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

  for y in stake_top..stake_bottom {
      framebuffer.point(i, y, framebuffer.current_color.to_hex());
  }
}

fn render_fps(framebuffer: &mut Framebuffer, fps: f32) {
  let background_color = Color::new(0, 0, 255); 
  let text_color = Color::new(255, 255, 0); 

  let label = "FPS: ";
  let digits = format!("{:.2}", fps);
  let text = format!("{}{}", label, digits);
  
  let width = text.len() * 15 + 20; 
  let height = 40; 
  let start_x = framebuffer.width - width - 10; 
  let start_y = 10; 


  for y in 0..height {
      for x in 0..width {
          framebuffer.point(start_x + x, start_y + y, background_color.to_hex());
      }
  }


  framebuffer.set_current_color(text_color);
  for (i, ch) in text.chars().enumerate() {
      draw_digit(framebuffer, start_x + 10 + i * 15, start_y + 15, ch); 
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
      'F' => ["#####", "#    ", "#####", "#    ", "#    "],
      'P' => ["#### ", "#   #", "#### ", "#    ", "#    "],
      'S' => [" ####", "#    ", " ### ", "    #", "#### "],
      _ => ["     ", "     ", "     ", "     ", "     "],
  };

  let scale = 2; 
  for (row, line) in digit_map.iter().enumerate() {
      for (col, ch) in line.chars().enumerate() {
          if ch == '#' {
              for dy in 0..scale {
                  for dx in 0..scale {
                      framebuffer.point(x + col * scale + dx, y + row * scale + dy, framebuffer.current_color.to_hex());
                  }
              }
          }
      }
  }
}

fn render_minimap(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    framebuffer_width: usize,
    framebuffer_height: usize,
    minimap_size: usize,
    block_size: usize,
) {
    let minimap_scale = minimap_size / maze.len().max(maze[0].len()); 
    let minimap_x = 10; 
    let minimap_y = framebuffer_height - minimap_size - 10; 

    // Dibujar el minimapa
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' | '-' | '|' => Color::new(100, 100, 100), // Color gris para paredes
                'p' => Color::new(255, 0, 0),                 // Verde para el punto de inicio
                'g' => Color::new(0, 255, 0),                 // Rojo para el punto final
                _ => Color::new(200, 200, 200),               // Color claro para el suelo
            };
            
            framebuffer.set_current_color(color);

            let x0 = minimap_x + col_index * minimap_scale;
            let y0 = minimap_y + row_index * minimap_scale;
            for x in 0..minimap_scale {
                for y in 0..minimap_scale {
                    framebuffer.point(x0 + x, y0 + y, framebuffer.current_color.to_hex());
                }
            }
        }
    }


    let player_minimap_x = minimap_x + (player.pos.x / block_size as f32 * minimap_scale as f32) as usize;
    let player_minimap_y = minimap_y + (player.pos.y / block_size as f32 * minimap_scale as f32) as usize;

    framebuffer.set_current_color(Color::new(0, 0, 255)); 
    framebuffer.point(player_minimap_x, player_minimap_y, framebuffer.current_color.to_hex());
}

fn main() {

    let python_script = "python";
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
        "Maze 2D/3D Taylosr’s Version",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(Color::new(51, 51, 85));

    let maze = load_maze("./maze.txt");

    let player_start = find_player_start(&maze).unwrap_or(Vec2::new(1.0, 1.0));
    let mut player = Player {
        pos: player_start * 30.0,
        a: PI / 3.0,
        fov: PI / 2.0,
    };

    let audio_player = AudioPlayer::new("fff.wav");
    audio_player.play(); 

    let wall_texture_1 = load_texture("Brick_20-128x128.png");
    let wall_texture_2 = load_texture("Brick_12-128x128.png");
    let wall_texture_3 = load_texture("Brick_02-128x128.png");
    let floor_texture = load_texture("suelo.png");

    let texture_width = 128;
    let texture_height = 128;

    let block_size = 25;
    let mut mode = "2D";
    let mut m_was_down = false;
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

        process_events(&window, &mut player, &maze, block_size, &mut last_mouse_pos, &audio_player);

        if mode == "2D" {
            render2d(
                &mut framebuffer,
                &player,
                &maze,
                block_size,
                &wall_texture_1,
                &wall_texture_2,
                &wall_texture_3,
                texture_width,
                texture_height,
            );
        } else {
            render3d(
                &mut framebuffer,
                &player,
                &maze,
                block_size,
                &wall_texture_1,
                &wall_texture_2,
                &wall_texture_3,
                texture_width,
                texture_height,
            );
        }


        let framebuffer_width = framebuffer.width;
        let framebuffer_height = framebuffer.height;


        render_minimap(
            &mut framebuffer,
            &player,
            &maze,
            framebuffer_width,
            framebuffer_height,
            200,                 
            block_size,          
        );

        frames += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(last_time).as_secs_f32();
        if elapsed >= 1.0 {
            fps = frames as f32 / elapsed;
            frames = 0;
            last_time = now;
        }

        render_fps(&mut framebuffer, fps);

        window
            .update_with_buffer(framebuffer.get_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
    audio_player.stop();
}
