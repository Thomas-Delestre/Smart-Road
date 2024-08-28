extern crate sdl2;
mod mapping;
use sdl2::event::Event;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    // Initialize SDL2 system
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // Initialize SDL2 image with the necessary formats (e.g., PNG, JPG)
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    // Set up window and canvas
    let window = video_subsystem.window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // Load texture (image file)
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("/home/student/Documents/Zone01/Rust/projects/smart-road-perso/new-road.jpg").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background (optional)
        canvas.clear();

        // Copy texture to the canvas
        canvas.copy(&texture, None, None).unwrap();  // None means the whole texture is drawn

        // Call the mapping function to draw lines on top of the background
        mapping::display(&mut canvas);

        // Present the canvas
        canvas.present();

        // Sleep to maintain ~60 FPS
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}