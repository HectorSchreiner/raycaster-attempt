use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub const WHITE: u32 = 0x00000000;
pub const BLACK: u32 = 0xffffffff;

fn main() {
    let mut renderer: Renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut window = Window::new(
        "Hectors Raycaster - Press ESC To EXIT",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let map: [i32; 64] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1,
    ];

    let pos: Position = Position { x: 20, y: 20 };
    let square: Square = Square {
        lenght: 50,
        height: 50,
        position: pos,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.background(WHITE);
        renderer.rect(&square, BLACK);
    }
}

struct Position {
    x: u32,
    y: u32,
}
struct Square {
    lenght: u32,
    height: u32,
    position: Position,
}
struct Renderer {
    buffer: Vec<u32>,
}

impl Renderer {
    fn draw_pixel(&mut self, position: Position, color: u32, default_color: u32) {
        for iter in 0..HEIGHT * WIDTH {
            if self.buffer[iter] == position.x + position.y * HEIGHT as u32 {
                self.buffer[iter] = color;
            } else {
                self.buffer[iter] = default_color;
            }
        }
    }

    fn rect(&mut self, square: &Square, color: u32) {
        let pos_y = square.position.y;
        let pos_x = square.position.x;

        for y in pos_y..square.height + pos_y {
            for x in pos_x..square.lenght + pos_x {
                self.buffer[(y * WIDTH as u32 + x) as usize] = color;
            }
        }
    }

    fn background(&mut self, color: u32) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = color;
        }
    }
}
