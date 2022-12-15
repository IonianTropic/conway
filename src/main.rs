

const SIZE: (usize, usize) = (64, 36);

fn main() {
    
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::Window::new(&event_loop).unwrap();

    let initial_size = window.inner_size();
    
    let surface_texture = pixels::SurfaceTexture::new(initial_size.width, initial_size.height, &window);

    let mut pixels = pixels::Pixels::new(SIZE.0 as u32, SIZE.1 as u32, surface_texture).unwrap();

    let mut game_state = [Cell::Dead; SIZE.0*SIZE.1];

    // tub
    game_state[SIZE.0*1+2] = Cell::Alive;
    game_state[SIZE.0*2+1] = Cell::Alive;
    game_state[SIZE.0*2+3] = Cell::Alive;
    game_state[SIZE.0*3+2] = Cell::Alive;

    // blinker
    game_state[SIZE.0*2+5] = Cell::Alive;
    game_state[SIZE.0*3+5] = Cell::Alive;
    game_state[SIZE.0*4+5] = Cell::Alive;

    let mut paused = true;
    
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait_until(std::time::Instant::now().checked_add(std::time::Duration::from_millis(200)).unwrap());
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(psize) => {
                    pixels.resize_surface(psize.width, psize.height);
                }
                // if P pressed
                winit::event::WindowEvent::KeyboardInput { 
                    input:winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(winit::event::VirtualKeyCode::P), 
                        ..
                    },
                    ..
                } => {
                    paused = !paused;
                }
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            }
            winit::event::Event::MainEventsCleared => {
                if !paused {
                    update_state(&mut game_state);
                }
                window.request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                write_game_state(&game_state, &mut pixels, SIZE);
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn to_rgba(self) -> Rgba {
        match self {
            Cell::Alive => Rgba(0xff, 0xff, 0, 0xff),
            Cell::Dead => Rgba(0, 0, 0xff, 0xff),
        }
    }
}

struct Rgba(u8, u8, u8, u8);

impl Rgba {
    fn write_to(self, pixel: &mut [u8]) {
        pixel[0] = self.0;
        pixel[1] = self.1;
        pixel[2] = self.2;
        pixel[3] = self.3;
    }
}

fn modulo(a: isize, b: isize) -> isize {
    let mut r = a;
    while r < 0 {
        r += b;
    }
    while r >= b {
        r -= b;
    }
    r
}

fn write_display_test(pixels: &mut pixels::Pixels, pixel_size: (u32, u32)) {
    let frame = pixels.get_frame_mut();
    let mut row = 0;
    let mut column = 0;
    for pixel in frame.chunks_exact_mut(4) {
        if column % 2 == row % 2 {
            pixel[0] = 0xff;
            pixel[1] = 0x00;
            pixel[2] = 0xff;
            pixel[3] = 0xff;
        }
        row += 1;
        if row == pixel_size.0 {
            row = 0;
            column += 1;
        }
    }
}

fn write_game_state(game_state: &[Cell], pixels: &mut pixels::Pixels, size: (usize, usize)) {
    let frame = pixels.get_frame_mut();
    for (pixel, state) in frame.chunks_exact_mut(4).zip(game_state) {
        state.to_rgba().write_to(pixel)
    }
}

fn update_state(game_state: &mut [Cell]) {
    let width = SIZE.0 as isize;
    let height = SIZE.1 as isize;
    let mut neighboor_count: [u8; SIZE.0*SIZE.1] = [0; SIZE.0*SIZE.1];
    for row in 0..height {
        for col in 0..width {
            let bp = 0;
            if wrapping_idx(game_state, row-1, col-1) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
            if wrapping_idx(game_state, row-1, col) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
            if wrapping_idx(game_state, row-1, col+1) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
            if wrapping_idx(game_state, row, col-1) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
            if wrapping_idx(game_state, row, col+1) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
            if wrapping_idx(game_state, row+1, col-1) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
            if wrapping_idx(game_state, row+1, col) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
            if wrapping_idx(game_state, row+1, col+1) == &Cell::Alive {
                neighboor_count[SIZE.0*row as usize+col as usize]+=1;
            }
        }
    }
    // println!("state: {:?}\nneighboors: {:?}", game_state, neighboor_count);
    for (cell, neighboors) in game_state.iter_mut().zip(neighboor_count) {
        match cell {
            Cell::Alive => if neighboors < 2 || neighboors > 3 {
                *cell = Cell::Dead;
            }
            Cell::Dead => if neighboors == 3 {
                *cell = Cell::Alive;
            }
        }
    }
}

fn wrapping_idx(game_state: &[Cell], row: isize, col: isize) -> &Cell {
    let width = SIZE.0 as isize;
    let height = SIZE.1 as isize;
    let idx = (width*modulo(row, height)+modulo(col, width)) as usize;
    // println!("row: {}\ncol: {}\nidx: {}", row, col, idx);
    return &game_state[idx]; // -1, 3
}

// fn mut_wrapping_idx(game_state: &mut [Cell], row: isize, col: isize) -> &mut Cell {
//     let width = SIZE.0 as isize;
//     let height = SIZE.1 as isize;
//     return &mut game_state[(width*(row%height)+(col%width)) as usize];
// }
