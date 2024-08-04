use nalgebra_glm as glm;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::line_impl::draw_line;

pub fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[glm::Vec3], color: Color) {
    if vertices.len() < 3 {
        return; 
    }

    for i in 0..vertices.len() {
        let v1 = &vertices[i];
        let v2 = &vertices[(i + 1) % vertices.len()]; 
        draw_line(framebuffer, v1, v2, color);
    }
}

pub fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[glm::Vec3], color: Color) {
    if vertices.len() < 3 {
        return; 
    }

    let mut y_min = vertices[0].y as isize;
    let mut y_max = vertices[0].y as isize;
    for vertex in vertices.iter() {
        y_min = y_min.min(vertex.y as isize);
        y_max = y_max.max(vertex.y as isize);
    }

    for y in y_min..=y_max {
        let mut nodes = vec![];
        let mut j = vertices.len() - 1;
        for i in 0..vertices.len() {
            let vi = &vertices[i];
            let vj = &vertices[j];

            if (vi.y as isize > y && vj.y as isize <= y) || (vj.y as isize > y && vi.y as isize <= y) {
                let node_x = (vj.x as isize + (y - vj.y as isize) * (vi.x as isize - vj.x as isize) / (vi.y as isize - vj.y as isize)) as isize;
                nodes.push(node_x);
            }
            j = i;
        }
        nodes.sort();

        for pair in nodes.chunks(2) {
            if let [start, end] = pair {
                for x in *start..=*end {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}

