

fn main() {
    
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::Window::new(&event_loop).unwrap();

    let initial_size = window.inner_size();
    
    let surface_texture = pixels::SurfaceTexture::new(initial_size.width, initial_size.height, &window);

    let pixel_size = (16, 12);

    let mut pixels = pixels::Pixels::new(pixel_size.0, pixel_size.1, surface_texture).unwrap();

    let game_array = [[0; 16]; 12];

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    pixels.resize_surface(size.width, size.height);
                }
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            }
            winit::event::Event::MainEventsCleared => {
                write_display_test(&mut pixels, pixel_size);
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
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
