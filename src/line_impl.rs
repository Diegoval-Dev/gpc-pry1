use crate::framebuffer::Framebuffer;
use crate::color::Color;
use nalgebra_glm::Vec3;

pub fn draw_line(fb: &mut Framebuffer, v1: &Vec3, v2: &Vec3, color: Color) {
    let x1 = v1.x as isize;
    let y1 = v1.y as isize;
    let x2 = v2.x as isize;
    let y2 = v2.y as isize;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        fb.point(x, y);
        if x == x2 && y == y2 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
