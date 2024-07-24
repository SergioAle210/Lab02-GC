use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

// Define los colores
const COLOR_FONDO: u32 = 0x000000; // Negro
const COLOR_CUADRO: u32 = 0xFFFFFF; // Blanco
const COLOR_COLISION: u32 = 0xFF0000; // Rojo para colisiones

fn add_glider(framebuffer: &mut Framebuffer, x_offset: usize, y_offset: usize) {
    let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    for (x, y) in glider {
        framebuffer.point(x + x_offset, y + y_offset);
    }
}

fn add_blinker(framebuffer: &mut Framebuffer, x_offset: usize, y_offset: usize) {
    let blinker = vec![(10, 10), (10, 11), (10, 12)];
    for (x, y) in blinker {
        framebuffer.point(x + x_offset, y + y_offset);
    }
}

fn add_toad(framebuffer: &mut Framebuffer, x_offset: usize, y_offset: usize) {
    let toad = vec![(15, 15), (16, 15), (17, 15), (16, 16), (17, 16), (18, 16)];
    for (x, y) in toad {
        framebuffer.point(x + x_offset, y + y_offset);
    }
}

fn add_beacon(framebuffer: &mut Framebuffer) {
    let beacon = vec![
        (20, 20),
        (21, 20),
        (20, 21),
        (21, 21),
        (22, 22),
        (23, 22),
        (22, 23),
        (23, 23),
    ];
    for (x, y) in beacon {
        framebuffer.point(x, y);
    }
}

fn add_block(framebuffer: &mut Framebuffer, x_offset: usize, y_offset: usize) {
    let block = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
    for (x, y) in block {
        framebuffer.point(x + x_offset, y + y_offset);
    }
}

fn add_lwss(framebuffer: &mut Framebuffer) {
    let lwss = vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 1),
        (0, 1),
        (0, 2),
        (3, 3),
        (1, 4),
    ];
    for (x, y) in lwss {
        framebuffer.point(x, y);
    }
}

fn add_mwss(framebuffer: &mut Framebuffer) {
    let mwss = vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 1),
        (0, 1),
        (0, 2),
        (4, 3),
        (1, 4),
    ];
    for (x, y) in mwss {
        framebuffer.point(x, y);
    }
}

fn add_hwss(framebuffer: &mut Framebuffer, x_offset: usize, y_offset: usize) {
    let hwss = vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (5, 1),
        (0, 1),
        (0, 2),
        (4, 3),
        (1, 4),
    ];
    for (x, y) in hwss {
        framebuffer.point(x - x_offset, y - y_offset);
    }
}

fn add_pulsar(framebuffer: &mut Framebuffer, x_offset: usize, y_offset: usize) {
    let pulsar_coords = [
        (2, 0),
        (3, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (0, 2),
        (5, 2),
        (7, 2),
        (12, 2),
        (0, 3),
        (5, 3),
        (7, 3),
        (12, 3),
        (0, 4),
        (5, 4),
        (7, 4),
        (12, 4),
        (2, 5),
        (3, 5),
        (4, 5),
        (8, 5),
        (9, 5),
        (10, 5),
        (2, 7),
        (3, 7),
        (4, 7),
        (8, 7),
        (9, 7),
        (10, 7),
        (2, 8),
        (3, 8),
        (4, 8),
        (8, 8),
        (9, 8),
        (10, 8),
        (0, 9),
        (5, 9),
        (7, 9),
        (12, 9),
        (0, 10),
        (5, 10),
        (7, 10),
        (12, 10),
        (0, 11),
        (5, 11),
        (7, 11),
        (12, 11),
        (2, 12),
        (3, 12),
        (4, 12),
        (8, 12),
        (9, 12),
        (10, 12),
    ];

    for &(x, y) in pulsar_coords.iter() {
        framebuffer.point(x + x_offset, y + y_offset);
    }
}

fn add_weekender(framebuffer: &mut Framebuffer, x_offset: usize, y_offset: usize) {
    let weekender = vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (0, 1),
        (3, 1),
        (0, 2),
        (3, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
        (5, 1),
        (6, 1),
        (4, 2),
        (5, 2),
        (6, 2),
        (7, 2),
        (4, 3),
        (7, 3),
        (4, 4),
        (7, 4),
        (4, 5),
        (5, 5),
        (6, 5),
        (7, 5),
        (5, 7),
        (6, 7),
        (5, 8),
        (6, 8),
        (5, 9),
        (6, 9),
        (9, 1),
        (10, 1),
        (8, 2),
        (9, 2),
        (10, 2),
        (11, 2),
        (8, 3),
        (11, 3),
        (8, 4),
        (11, 4),
        (8, 5),
        (9, 5),
        (10, 5),
        (11, 5),
        (13, 0),
        (14, 0),
        (15, 0),
        (12, 1),
        (15, 1),
        (12, 2),
        (15, 2),
        (12, 3),
        (13, 3),
        (14, 3),
        (15, 3),
    ];
    for (x, y) in weekender {
        framebuffer.point(x + x_offset, y + y_offset);
    }
}

fn count_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dx in [-1, 0, 1].iter().cloned() {
        for dy in [-1, 0, 1].iter().cloned() {
            if dx != 0 || dy != 0 {
                let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(HEIGHT as isize) as usize;
                if framebuffer.is_point_set(nx, ny) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn update(framebuffer: &Framebuffer) -> Framebuffer {
    let mut new_framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbors = count_neighbors(framebuffer, x, y);
            if framebuffer.is_point_set(x, y) {
                if neighbors == 2 || neighbors == 3 {
                    new_framebuffer.point(x, y);
                }
            } else {
                if neighbors == 3 {
                    new_framebuffer.point(x, y);
                }
            }
        }
    }
    new_framebuffer
}

fn initialize_pattern(framebuffer: &mut Framebuffer) {
    // Toad
    add_toad(framebuffer, 0, 0);

    // Beacon
    add_beacon(framebuffer);
    // Pulsars
    add_pulsar(framebuffer, 30, 30);
    add_pulsar(framebuffer, 20, 50);
    add_pulsar(framebuffer, 20, 50);
    add_pulsar(framebuffer, 70, 50);
    add_pulsar(framebuffer, 70, 20);

    //Weekender
    add_weekender(framebuffer, 20, 20);
    // Blinker
    add_blinker(framebuffer, 20, 40);
    // Lwss
    add_lwss(framebuffer);
    // Mwss
    add_mwss(framebuffer);
    // hwss
    add_hwss(framebuffer, 0, 0);
    // Blocks
    add_block(framebuffer, 50, 50);
    add_block(framebuffer, 80, 20);

    //Gliders
    add_glider(framebuffer, 0, 0);
    add_glider(framebuffer, 50, 2);
    add_glider(framebuffer, 60, 7);
    add_glider(framebuffer, 7, 59);

    add_glider(framebuffer, 0, 80);
}

fn main() {
    let window_width = WIDTH * 8;
    let window_height = HEIGHT * 8;
    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    initialize_pattern(&mut framebuffer);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Manejo de eventos de teclado para agregar patrones
        if window.is_key_down(Key::B) {
            add_block(&mut framebuffer, 40, 40);
        }
        if window.is_key_down(Key::G) {
            add_glider(&mut framebuffer, 0, 0);
        }

        framebuffer = update(&framebuffer);

        let mut display_buffer = vec![COLOR_FONDO; window_width * window_height]; // Usa el color de fondo
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = count_neighbors(&framebuffer, x, y);
                if framebuffer.is_point_set(x, y) {
                    let color = if neighbors > 3 {
                        COLOR_COLISION
                    } else {
                        COLOR_CUADRO
                    };
                    for dy in 0..8 {
                        for dx in 0..8 {
                            display_buffer[(y * 8 + dy) * window_width + (x * 8 + dx)] = color;
                        }
                    }
                }
            }
        }

        window
            .update_with_buffer(&display_buffer, window_width, window_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
