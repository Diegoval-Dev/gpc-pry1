use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::color::Color;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool, // Parámetro adicional para controlar el dibujo de la línea
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(Color::new(255, 221, 221)); // Color de la línea de rayos

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        // Si el rayo golpea una pared (cualquier celda que no sea ' '), devolvemos la intersección
        if maze[j][i] != ' ' {
            return Intersect {
                distance: d,
                impact: maze[j][i],
            };
        }

        // Dibuja la línea solo si draw_line es true
        if draw_line {
            framebuffer.point(x, y, framebuffer.current_color.to_hex());
        }

        d += 10.0;
    }
}
