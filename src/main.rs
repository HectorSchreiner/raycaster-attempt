use line_drawing::Bresenham;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1200;
const HEIGHT: usize = 600;
const PI: f32 = 3.1415;

#[derive(Clone, Copy)]
enum Color {
    WHITE = 0xffffff,
    BLACK = 0x000000,
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
    DARK_GREY = 0x202020,
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

    let mut map_grid: [u32; 64] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0,
        0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
        1, 1, 1, 1,
    ];

    let mut map: Map = Map { map: map_grid };

    let mut player = Player {
        pos: Position { x: 80, y: 80 },
        angle: 30.0,
        pdx: 0.0,
        pdy: 0.0,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();

        renderer.clear(Color::DARK_GREY);
        map.draw_map_2d(&mut renderer);
        player.draw_player(&mut renderer);
        //player.vission(&mut renderer);
        map.raycast(&player, &mut renderer);
        player.move_player(&window);
    }
}

struct Map {
    map: [u32; 64],
}
impl Map {
    fn draw_map_2d(&mut self, renderer: &mut Renderer) {
        let mapX: u32 = 8;
        let mapY: u32 = 8;
        let mut col: Color;

        for y in 0..mapY {
            for x in 0..mapX {
                if self.map[(y * mapY + x) as usize] == 1 {
                    col = Color::WHITE;
                } else {
                    col = Color::BLACK;
                }
                let cell: Square = Square::new(
                    WIDTH as u32 / mapX / 2 - 1,
                    HEIGHT as u32 / mapY - 1,
                    Position::from((x * WIDTH as u32 / mapX / 2, y * HEIGHT as u32 / mapY)),
                );
                renderer.rect(&cell, col);
            }
        }
    }

    fn raycast(&mut self, player: &Player, renderer: &mut Renderer) {
        let (mut r, mut mx, mut my, mut mp, mut dof) = (1i32, 2i32, 3i32, 4i32, 5i32);
        let (mut rx, mut ry, mut ra, mut xo, mut yo) = (1f32, 2f32, 3f32, 4f32, 5f32);
        let ra = player.angle;
        let px = player.pos.x as f32;
        let py = player.pos.y as f32;

        for r in 0..1 {
            //--- Check horixontale linjer ---
            dof = 0;
            let aTan = -1.0 / ra.tan();

            // looking up
            if (ra > PI) {
                ry = ((py as i32 >> 6) << 6) as f32 * -0.0001;
                rx = (py - ry) * aTan + px;
                yo = -64.0;
                xo = -yo * aTan;
            }
            // looking down
            if (ra < PI) {
                ry = ((py as i32 >> 6) << 6) as f32 + 64.0;
                rx = (py - ry) * aTan + px;
                yo = 64.0;
                xo = -yo * aTan;
            }
            // looking straight left or right
            if (ra == 0.0 || ra == PI) {
                rx = px;
                ry = py;
                dof = 8;
            }
            while (dof < 8) {
                mx = rx as i32 >> 6;
                my = ry as i32 >> 6;
                mp = my * 8 + mx;

                // muren er ramt
                if mp < self.map.len() as i32 && self.map[mp as usize] == 1 {
                    dof = 8;
                }
                // next line
                else {
                    rx += xo;
                    ry += yo;
                    dof += 1;
                }
            }
            let line = Line::new((player.pos.x, player.pos.y), (rx as u32, ry as u32));
            renderer.line(&line, Color::GREEN);
        }
    }
}

struct Player {
    pos: Position,
    angle: f32,
    pdx: f32,
    pdy: f32,
}
impl Player {
    fn draw_player(&mut self, renderer: &mut Renderer) {
        let player_size = 10;
        let player = Square::new(player_size, player_size, (self.pos.x, self.pos.y));

        renderer.rect(&player, Color::RED);
    }

    fn vission(&mut self, renderer: &mut Renderer) {
        let line: Line = Line::new(
            (self.pos.x + 5, self.pos.y + 5),
            (
                (self.pos.x as f32 + self.pdx * 10.0) as u32,
                (self.pos.y as f32 + self.pdy as f32 * 10.0) as u32,
            ),
        );
        renderer.line(&line, Color::GREEN);
    }

    fn move_player(&mut self, window: &Window) {
        if window.is_key_down(Key::A) {
            self.angle -= 0.1;

            if (self.angle < 0.0) {
                self.angle += 2.0 * PI;
            };
            self.pdx = self.angle.cos() * 5.0;
            self.pdy = self.angle.sin() * 5.0;
        }

        if window.is_key_down(Key::D) {
            self.angle += 0.1;

            if (self.angle > 2.0 * PI) {
                self.angle -= 2.0 * PI;
            };
            self.pdx = self.angle.cos() * 5.0;
            self.pdy = self.angle.sin() * 5.0;
        }

        if window.is_key_down(Key::W) {
            self.pos.x = (self.pos.x as f32 + self.pdx) as u32;
            self.pos.y = (self.pos.y as f32 + self.pdy) as u32;
        }

        if window.is_key_down(Key::S) {
            self.pos.x = (self.pos.x as f32 - self.pdx) as u32;
            self.pos.y = (self.pos.y as f32 - self.pdy) as u32;
        }
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
        for (x, y) in Bresenham::new(
            (line.pos_1.x as i32, line.pos_1.y as i32),
            (line.pos_2.x as i32, line.pos_2.y as i32),
        ) {
            self.draw_pixel(
                Position {
                    x: x as u32,
                    y: y as u32,
                },
                color,
            );
        }
    }

    fn clear(&mut self, color: Color) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = color as _;
        }
    }
}
