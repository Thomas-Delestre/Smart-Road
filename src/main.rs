use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use vehicules::{Start, CLOSE_CALL};
use intersection::Intersection;
use sprites::Sprite;

mod vehicules;
mod sprites;
mod intersection;

fn main() -> Result<(), String> {
    const BG_SOURCE: &str = "./assets/road.jpg"; // Fichier source du background
    const VEHICULE_SOURCE: &str = "./assets/car.png";
    const VEHICULE_SIZE: (u32, u32) = (45, 40); 
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

    let mut show_stats = false; // Afficher les statistiques

    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if show_stats {
                        println!("Exiting main loop");
                        break 'running;
                    } else {
                        println!("Showing statistics window");
                        show_stats = true;
                    }
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if last_key_event_time.elapsed() >= Duration::from_millis(250) {
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
        
       // Trouver les vitesses minimales et maximales parmi tous les véhicules terminés
        let min_speed = intersection.finished_vehicles.iter().map(|v| v.min_speed).min().unwrap_or(0);
        let max_speed = intersection.finished_vehicles.iter().map(|v| v.max_speed).max().unwrap_or(0);

       // Calculer le temps minimum et maximum des véhicules qui ont terminé
        let min_time = intersection
        .finished_vehicles
        .iter()
        .filter_map(|v| v.total_time) // Assure que seuls les temps définis sont pris en compte
        .min()
        .unwrap_or(Duration::ZERO);

        let max_time = intersection
        .finished_vehicles
        .iter()
        .filter_map(|v| v.total_time) // Assure que seuls les temps définis sont pris en compte
        .max()
        .unwrap_or(Duration::ZERO);


        let count_vehicles = intersection.finished_vehicles.len() as i32;

        // Affiche les statistiques uniquement si `show_stats` est vrai
        if show_stats {
            show_statistics_window(
                //&intersection.finished_vehicles,
                &sdl_context,
                &mut event_pump,
                count_vehicles,
                min_speed,
                max_speed,
                min_time,
                max_time,
            );
            break 'running; // ou mets une autre logique pour rester dans la boucle
        }



        // Update logic here
        intersection.step();
       
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

fn show_statistics_window(
    sdl_context: &sdl2::Sdl,
    event_pump: &mut sdl2::EventPump,
    count_vehicles: i32,
    min_speed: u8,
    max_speed: u8,
    min_time: Duration,
    max_time: Duration,
) {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Statistics", 400, 300)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let font_size = 24;
    let font_context = sdl2::ttf::init().unwrap(); // ttf permet de charger des polices de caractères
    let font = font_context
        .load_font(
            &format!("{}/assets/font.ttf", env!("CARGO_MANIFEST_DIR")),
            font_size,
        )
        .unwrap();
    
    // Convert max_time to string
    let max_time_str = if max_time == Duration::ZERO {
        "N/A".to_string()
    } else {
        format!("{:.2}s", max_time.as_secs_f64()) // Convert Duration to seconds as f64
    };
    // Convert min_time to string
    let min_time_str = if min_time == Duration::MAX {
        "N/A".to_string()
    } else {
        format!("{:.2}s", min_time.as_secs_f64()) // Convert Duration to seconds as f64
    };
    let lines = [
        format!("Vehicles that passed: {}", count_vehicles),
        format!("Max velocity: {}", max_speed),
        format!("Min velocity: {}", min_speed),
        format!("Max time: {}", max_time_str),
        format!("Min time: {}", min_time_str),
        format!("Collisions: 0"),
        format!("Close Calls: {}", unsafe { CLOSE_CALL }),
    ];
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.clear();
    let mut y_offset = 20;
    let line_height = font_size as i32 + 5;
    for line in &lines {
        let surface = font.render(&line).blended(Color::RGB(0, 0, 0)).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let texture_query = texture.query();
        let text_rect = Rect::new(20, y_offset, texture_query.width, texture_query.height);
        canvas.copy(&texture, None, Some(text_rect)).unwrap();
        y_offset += line_height;
    }
    canvas.present();
    'stats_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'stats_loop,
                _ => {}
            }
        }
    }
}