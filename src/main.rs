use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[derive(Clone, Copy)]
enum Color {
    WHITE = 0xffffff,
    BLACK = 0x000000,
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
}

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

    let square_1 = Square::new(10, 40, [50, 50]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.background(Color::BLACK);
        renderer.rect(&square_1, Color::RED);
    }
}

struct Position {
    x: u32,
    y: u32,
}
impl From<(u32, u32)> for Position {
    fn from(position: (u32, u32)) -> Self {
        Self {
            x: position.0,
            y: position.1,
        }
    }
}

impl From<[u32; 2]> for Position {
    fn from(position: [u32; 2]) -> Self {
        Self {
            x: position[0],
            y: position[1],
        }
    }
}

struct Square {
    lenght: u32,
    height: u32,
    position: Position,
}

impl Square {
    fn new(lenght: u32, height: u32, position: impl Into<Position>) -> Square {
        Square {
            lenght,
            height,
            position: position.into(),
        }
    }
}
struct Renderer {
    buffer: Vec<u32>,
}

impl Renderer {
    fn draw_pixel(&mut self, position: Position, color: Color, default_color: Color) {
        for iter in 0..HEIGHT * WIDTH {
            if self.buffer[iter] == position.x + position.y * HEIGHT as u32 {
                self.buffer[iter] = color as _;
            } else {
                self.buffer[iter] = default_color as _;
            }
        }
    }

    fn rect(&mut self, square: &Square, color: Color) {
        let pos_y = square.position.y;
        let pos_x = square.position.x;

        for y in pos_y..square.height + pos_y {
            for x in pos_x..square.lenght + pos_x {
                self.buffer[(y * WIDTH as u32 + x) as usize] = color as _;
            }
        }
    }

    fn background(&mut self, color: Color) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = color as _;
        }
    }
}
