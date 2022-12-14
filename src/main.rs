

fn main() {
    
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::Window::new(&event_loop).unwrap();

    let initial_size = window.inner_size();
    
    let surface_texture = pixels::SurfaceTexture::new(initial_size.width, initial_size.height, &window);

    const SIZE: (usize, usize) = (16, 12);

    let mut pixels = pixels::Pixels::new(SIZE.0 as u32, SIZE.1 as u32, surface_texture).unwrap();

    let mut game_state = [Cell::Dead; SIZE.0*SIZE.1];

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(psize) => {
                    pixels.resize_surface(psize.width, psize.height);
                }
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            }
            winit::event::Event::MainEventsCleared => {
                get_next_state(&mut game_state, SIZE);
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

fn get_next_state(game_state: &mut [Cell], size: (usize, usize)) {
    let mut next_state = Vec::new();
    let width = size.0 as isize;
    let height = size.1 as isize;
    let get_idx = |i: isize, j: isize| ((i % height)*width+(j % height)) as usize;
    println!("0, 0: {:?}", get_idx(0, 0));
    println!("1, 1: {:?}", get_idx(1, 1));
    for i in 0..height {
        for j in 0..width {
            let mut count = 0;
            // check neighborhood
            // current cell game_state[i*width+j] or game_state[get_idx(i, j)]
            if game_state[get_idx(i-1,j-1)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i-1,j)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i-1,j+1)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i,j-1)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i,j+1)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i+1,j-1)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i+1,j)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i+1,j+1)] == Cell::Alive {
                count += 1;
            }
            if game_state[get_idx(i,j)] == Cell::Alive {
                if count < 2 {
                    next_state.push(Cell::Dead);
                } else if count > 3 {
                    next_state.push(Cell::Dead);
                } else {
                    next_state.push(Cell::Alive);
                }
            } else {
                if count == 3 {
                    next_state.push(Cell::Alive);
                } else {
                    next_state.push(Cell::Dead);
                }
            }
        }
        for i in 0..width*height {
            game_state[i as usize] = next_state[i as usize];
        } 
    }
}

fn write_game_state(game_state: &[Cell], pixels: &mut pixels::Pixels, size: (usize, usize)) {
    let frame = pixels.get_frame_mut();
    for (pixel, state) in frame.chunks_exact_mut(4).zip(game_state) {
        state.to_rgba().write_to(pixel)
    }
}
