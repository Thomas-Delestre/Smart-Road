use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use vehicules::Start;
use intersection::Intersection;
use sprites::Sprite;

mod vehicules;
mod sprites;
mod intersection;

fn main() -> Result<(), String> {
    const BG_SOURCE: &str = "./assets/road.jpg"; // Fichier source du background
    const VEHICULE_SOURCE: &str = "./assets/car.png";
    const VEHICULE_SIZE: (u32, u32) = (50, 40); 
    const WINDOW_SIZE: (u32, u32) = (800, 800);


    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Smart Intersection", WINDOW_SIZE.0, WINDOW_SIZE.1) // 800 et 800 c'est la taille de ma fenetre
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // Set a black background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    let mut intersection_sprite = Sprite::new(BG_SOURCE, 1, 1);
    intersection_sprite.load(&texture_creator)?;
    let mut voitures_sprite = Sprite::new(VEHICULE_SOURCE, 2, 3);
    voitures_sprite.load(&texture_creator)?;
    
    let mut intersection = Intersection::new(intersection_sprite);
    
    let mut last_key_event_time = Instant::now();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running; // Quitter la boucle lorsque l'événement de fermeture est reçu
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running; // Quitter la boucle lorsque la touche Échap est enfoncée
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if last_key_event_time.elapsed() >= Duration::from_millis(300) {
                        last_key_event_time = Instant::now(); // Met à jour le dernier événement de temps
                        
                        // println!("Key down: {:?}", keycode);
                        match keycode {
                            Keycode::Up => {
                                intersection.add_car(WINDOW_SIZE, &voitures_sprite, Start::South, VEHICULE_SIZE);
                                // println!("Moving Up");
                            }
                            Keycode::Down => {
                                intersection.add_car(WINDOW_SIZE, &voitures_sprite, Start::North, VEHICULE_SIZE);
                                // println!("Moving Down");
                            }
                            Keycode::Left => {
                                intersection.add_car(WINDOW_SIZE, &voitures_sprite, Start::East, VEHICULE_SIZE);
                                // println!("Moving Left");
                            }
                            Keycode::Right => {
                                intersection.add_car(WINDOW_SIZE, &voitures_sprite, Start::West, VEHICULE_SIZE);
                                // println!("Moving Right");
                            }
                            Keycode::R => {
                                
                                // println!("R for RANDOM");
                                // Générer un entier aléatoire entre 0 et 3
                                let mut rng = rand::thread_rng();
                                let random_direction = match rng.gen_range(0, 4) {
                                    0 => Start::South,
                                    1 => Start::North,
                                    2 => Start::East,
                                    _ => Start::West,
                                };

                                intersection.add_car(WINDOW_SIZE, &voitures_sprite, random_direction, VEHICULE_SIZE);
                          
                            }
                            Keycode::L => {
                                println!("List of vehicles in intersection:\n{}", intersection.list_vehicles());
                            }
                            _ => {}
                        }
                   
                }
            }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    println!("Key up: {:?}", keycode); // Affiche la touche relâchée
                    // println!("List of vehicles in intersection: {:?}", intersection.cars);
                }
                _ => {}
                
            }
        }
        // Update logic here
        intersection.step();
        // Draw vehicles here
       
        canvas.clear(); 
        // Draw vehicles   
        intersection.draw(&mut canvas)?; 
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let _ = canvas.draw_rect(Rect::new(250, 250, 300, 300));
        canvas.present();
        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
