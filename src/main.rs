use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

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
    let line = Line::new((590, 300), (10, 400));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.clear(Color::BLACK);

        renderer.line(&line, Color::WHITE);
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

struct Line {
    pos_1: Position,
    pos_2: Position,
}

impl Line {
    fn new(pos_1: impl Into<Position>, pos_2: impl Into<Position>) -> Line {
        Line {
            pos_1: pos_1.into(),
            pos_2: pos_2.into(),
        }
    }
}
struct Renderer {
    buffer: Vec<u32>,
}

impl Renderer {
    fn draw_pixel(&mut self, position: Position, color: Color) {
        self.buffer[(position.x + position.y * WIDTH as u32) as usize] = color as _;
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

    fn line(&mut self, line: &Line, color: Color) {
        let dx: f32 = line.pos_1.x as f32 - line.pos_2.x as f32;
        let dy: f32 = line.pos_1.y as f32 - line.pos_2.y as f32;

        let mut slope = dy / dx;

        if line.pos_2.x < line.pos_1.x {
            for x in line.pos_1.x..line.pos_2.x {
                self.draw_pixel(
                    Position {
                        x: x,
                        y: (((x as f32 * slope) + line.pos_1.y as f32) as u32),
                    },
                    color,
                );
            }
        } else {
            slope *= -1.0;

            for x in line.pos_2.x..line.pos_1.x {
                self.draw_pixel(
                    Position {
                        x: x,
                        y: (((x as f32 * slope) + line.pos_1.y as f32) as u32),
                    },
                    color,
                );
            }
        }
    }

    fn clear(&mut self, color: Color) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = color as _;
        }
    }
}
