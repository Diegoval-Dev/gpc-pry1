use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::thread;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::maze::load_maze;
use crate::polygon_impl::fill_polygon;
use nalgebra_glm as glm;

mod line_impl;
mod framebuffer;
mod color;
mod maze;
mod polygon_impl;

const WIDTH: usize = 800; 
const HEIGHT: usize = 540; 
const BLOCK_SIZE: usize = 26; 

fn draw_cell(framebuffer: &mut Framebuffer, x: usize, y: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '-' | '|' => Color::new(0, 0, 0),      
        ' ' => Color::new(255, 255, 255),            
        'p' => Color::new(0, 255, 0),                
        'g' => Color::new(255, 0, 0),                
        _ => Color::new(255, 255, 255),              
    };

    framebuffer.set_current_color(color);

    let x = x as f32;
    let y = y as f32;
    let block_size = block_size as f32;

    let vertices = [
        glm::vec3(x, y, 0.0),
        glm::vec3(x + block_size, y, 0.0),
        glm::vec3(x + block_size, y + block_size, 0.0),
        glm::vec3(x, y + block_size, 0.0),
    ];

    fill_polygon(framebuffer, &vertices, color);
}

fn render(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>) {
    for (row, maze_row) in maze.iter().enumerate() {
        for (col, &cell) in maze_row.iter().enumerate() {
            draw_cell(framebuffer, col * BLOCK_SIZE, row * BLOCK_SIZE, BLOCK_SIZE, cell);
        }
    }
}

fn main() {
    let window_width = WIDTH;
    let window_height = HEIGHT;
    let frame_delay = Duration::from_millis(16); 

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let maze = load_maze("./maze.txt");

    let mut window = Window::new(
        "Maze Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        framebuffer.clear(); 
        render(&mut framebuffer, &maze);

        window
            .update_with_buffer(framebuffer.get_buffer(), WIDTH, HEIGHT)
            .unwrap();

        thread::sleep(frame_delay);
    }
}
