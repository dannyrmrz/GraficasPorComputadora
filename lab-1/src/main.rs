use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Polygon {
    points: Vec<Point>,
    fill_color: Color,
    border_color: Color,
    is_hole: bool,
}

struct FrameBuffer {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl FrameBuffer {
    fn new(width: i32, height: i32) -> Self {
        FrameBuffer {
            width,
            height,
            pixels: vec![Color::BLACK; (width * height) as usize],
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> Color {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index]
        } else {
            Color::BLACK
        }
    }

    fn clear(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }
}

// Point-in-polygon test using ray casting algorithm
fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let pi = polygon[i];
        let pj = polygon[j];
        
        if ((pi.y > point.y) != (pj.y > point.y)) &&
           (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x) {
            inside = !inside;
        }
    }
    
    inside
}

// Check if a point is inside any hole
fn point_in_holes(point: Point, holes: &[&Polygon]) -> bool {
    for hole in holes {
        if point_in_polygon(point, &hole.points) {
            return true;
        }
    }
    false
}

fn scanline_fill_with_holes(polygons: &[Polygon], framebuffer: &mut FrameBuffer) {
    // Separate main polygons from holes
    let mut main_polygons = Vec::new();
    let mut holes = Vec::new();
    
    for polygon in polygons {
        if polygon.is_hole {
            holes.push(polygon);
        } else {
            main_polygons.push(polygon);
        }
    }
    
    // For each main polygon
    for polygon in &main_polygons {
        let points = &polygon.points;
        if points.len() < 3 {
            continue;
        }

        // Find min and max y coordinates
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();

        // For each scanline
        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            // Find intersections with polygon edges
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];

                // Skip horizontal edges
                if p1.y == p2.y {
                    continue;
                }

                // Check if scanline intersects with this edge
                if (p1.y <= y && y < p2.y) || (p2.y <= y && y < p1.y) {
                    // Calculate x intersection
                    let x = if p2.y != p1.y {
                        p1.x + ((y - p1.y) * (p2.x - p1.x)) / (p2.y - p1.y)
                    } else {
                        p1.x
                    };
                    intersections.push(x);
                }
            }

            // Sort intersections
            intersections.sort();

            // Fill between pairs of intersections
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let start_x = intersections[i] as i32;
                    let end_x = intersections[i + 1] as i32;
                    
                    for x in start_x..=end_x {
                        let point = Point { x, y };
                        
                        // Check if this point is inside any hole
                        let mut is_in_hole = false;
                        for hole in &holes {
                            if point_in_polygon(point, &hole.points) {
                                is_in_hole = true;
                                break;
                            }
                        }
                        
                        // Only fill if not in a hole
                        if !is_in_hole {
                            framebuffer.set_pixel(x, y, polygon.fill_color);
                        }
                    }
                }
            }
        }
    }
}

fn draw_polygon_border(polygon: &Polygon, framebuffer: &mut FrameBuffer) {
    let points = &polygon.points;
    if points.len() < 2 {
        return;
    }

    // Draw lines between consecutive points
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        
        // Bresenham line algorithm
        let mut x0 = p1.x;
        let mut y0 = p1.y;
        let x1 = p2.x;
        let y1 = p2.y;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            framebuffer.set_pixel(x0, y0, polygon.border_color);
            
            if x0 == x1 && y0 == y1 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    
    let mut framebuffer = FrameBuffer::new(width, height);
    framebuffer.clear(Color::WHITE);

    // Define polygons
    let polygons = vec![
        Polygon {
            points: vec![
                Point { x: 377, y: 249 },
                Point { x: 411, y: 197 },
                Point { x: 436, y: 249 },
            ],
            fill_color: Color::RED, // Rojo
            border_color: Color::WHITE, // Orilla blanca
            is_hole: false,
        },
    ];

    // Fill polygons with hole detection
    scanline_fill_with_holes(&polygons, &mut framebuffer);
    
    // Draw borders for all polygons
    for polygon in &polygons {
        draw_polygon_border(polygon, &mut framebuffer);
    }

    // Initialize raylib
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Polygon Filler")
        .build();

    rl.set_target_fps(60);

    // Create texture from framebuffer
    let mut image = Image::gen_image_color(width, height, Color::WHITE);
    for y in 0..height {
        for x in 0..width {
            let pixel = framebuffer.get_pixel(x, y);
            image.draw_pixel(x, y, pixel);
        }
    }
    
    let texture = rl.load_texture_from_image(&thread, &image).unwrap();

    // Main game loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        
        d.draw_text("Polygon Filler - Press ESC to exit", 10, 10, 20, Color::BLACK);
    }

    // Save the image
    image.export_image("out.bmp");
} 