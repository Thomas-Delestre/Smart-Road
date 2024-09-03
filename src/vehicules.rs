use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{WindowContext, Window};
use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};

use crate::sprites::Sprite;



pub enum Start {
    North,
    South,
    East,
    West,
}

pub enum Direction {
    Straigth,
    Left,
    Right,
}


pub struct Path {
    pub steps: Vec<Point>,
    pub current_index: usize,
}

impl Path {
    pub fn new(steps: Vec<Point>) -> Path {
        Path {
            steps,
            current_index: 0,
        }
    }
    pub fn current_target(&self) -> Option<Point> {
        self.steps.get(self.current_index).copied()
    }

    pub fn advance(&mut self) {
        if self.current_index < self.steps.len() - 1 {
            self.current_index += 1;
        }
    }
}


pub struct Vehicule<'a> {
    pub id: u16,
    pub frame_id: usize,
    pub x: i64, 
    pub y: i64,
    pub sprite: &'a  Sprite<'a>, 
    speed: u8, // vitesse en pixels / frames
    security_distance: u32,
    // current: Pos,
    cross_passed: bool,
    path: Path,
    angle: f64
   
} 

impl<'a> Vehicule<'a>  {
    pub fn new(x: i64, y: i64, sprite: &'a  Sprite<'a> , speed: u8, security_distance: u32, path: Path, frame_id: u16) -> Vehicule<'a> {
        Vehicule {
            id: 0,
            frame_id: frame_id as usize,
            x, 
            y,
            sprite,
            speed : speed/2,
            security_distance,
            path, 
            cross_passed: false,
            angle: 0.0,
            
        }
    }

    pub fn step(&mut self) { // Logique de la voiture frame par frame
     
        if let Some(target) = self.path.current_target() {
            // Calculer la direction vers le point de passage suivant
            let dx = target.x - self.x as i32;
            let dy = target.y - self.y as i32;

                // Check des directions pour update les angles
            if dy > 0  {
                self.angle = 180.0;
            }
            if dy < 0 {
                self.angle = 0.0;
            }

            if dx > 0  {
                self.angle = 90.0;
            }
            if dx < 0 {
                self.angle = -90.0;
            }
            // update les derniers pixel avant le points a atteindre
            if dx.abs() <= self.speed as i32 {
                self.x += dx as i64;
            }
            if dy.abs() <= self.speed as i32 {
                self.y += dy as i64;
            }

            if dx.abs() <= self.speed as i32 && dy.abs() <= self.speed as i32 {
                // Si le véhicule a atteint le point cible, avancer au point suivant
                self.path.advance();
            } else {
                // Calculer le pas de déplacement vers le point cible
                let distance = ((dx * dx + dy * dy) as f64).sqrt();
                let step_x = (dx as f64 / distance * self.speed as f64) as i64;
                let step_y = (dy as f64 / distance * self.speed as f64) as i64;
                                    
                self.x += step_x;
                self.y += step_y;
            }
        }
        
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>, xscale: f32, yscale: f32) -> Result<(), String> { // draw de la voiture 
        if let Some(texture) = &self.sprite.loaded {
            let src_rect = self.sprite.get_frame(self.frame_id);
            let fsize = self.sprite.get_frame_size();
            let dest_rect = Rect::new(self.x as i32, self.y as i32, (fsize.0 as f32 * xscale).round() as u32, (fsize.1 as f32 * yscale).round() as u32); // Ajustez la taille selon votre texture
            canvas.copy_ex(texture, Some(src_rect), Some(dest_rect), self.angle, None, false, false)?;
        }else {

        }
        Ok(())
    }
    pub fn check_center() -> bool { // WIP
        true
    }

}


pub fn generate_path(start: Start, dir: Direction, window_size: (u32, u32), vehicule_height: u32) -> Path {
    match start {
        Start::West => {
            match dir {
                Direction::Left => Path::new(vec![
                    Point::new(0, (window_size.1 / 2) as i32),
                    Point::new((window_size.0 / 2 + 7)  as i32, (window_size.1 / 2) as i32),
                    Point::new((window_size.0 / 2 + 7) as i32, 0),
                ]),
                Direction::Straigth => Path::new(vec![
                    Point::new(0, (window_size.1 / 2 + vehicule_height ) as i32),
                    Point::new(window_size.0 as i32, (window_size.1 / 2 + vehicule_height) as i32),
                ]),
                Direction::Right => Path::new(vec![
                    Point::new(0, (window_size.1 / 2 + 80) as i32),
                    Point::new((window_size.0 / 3)  as i32, (window_size.1 / 2 + 80) as i32),
                    Point::new((window_size.0 / 3 + 7) as i32, (window_size.1) as i32),
                ])
            }
           },
           Start::South => {
            match dir {
                Direction::Left => Path::new(vec![
                    Point::new((window_size.0 / 2 + 7) as i32, window_size.1 as i32),
                    Point::new((window_size.0 / 2 + 7) as i32, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                    Point::new(0, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                ]),
                Direction::Straigth => Path::new(vec![
                    Point::new((window_size.0 / 2 + vehicule_height + 10) as i32, window_size.1 as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 10) as i32, 0),
                ]),
                Direction::Right => Path::new(vec![
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, window_size.1 as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, (window_size.1 / 2 + vehicule_height + 45) as i32),
                    Point::new(window_size.0 as i32, (window_size.1 / 2 + vehicule_height + 45) as i32),
                ])
            }
           },
           Start::East => {
            match dir {
                Direction::Left => Path::new(vec![
                    Point::new(window_size.0 as i32, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, window_size.1 as i32),
                ]),
                Direction::Straigth => Path::new(vec![
                    Point::new(window_size.0 as i32, (window_size.1 / 2 - (vehicule_height + 57)) as i32),
                    Point::new(0 as i32, (window_size.1 / 2 - (vehicule_height + 57)) as i32),
                ]),
                Direction::Right => Path::new(vec![
                    Point::new(window_size.0 as i32, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, 0 as i32),
                ])
            }
           },
           Start::North => {
            match dir {
                Direction::Left => Path::new(vec![
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, 0),
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, (window_size.1 / 2) as i32),
                    Point::new(window_size.0 as i32, (window_size.1 / 2) as i32),
                ]),
                Direction::Straigth => Path::new(vec![
                    Point::new((window_size.0 / 2 - (vehicule_height + 50)) as i32, 0 as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 50)) as i32, window_size.1 as i32),
                ]),
                Direction::Right => Path::new(vec![
                    Point::new((window_size.0 / 2 - (vehicule_height + 95)) as i32, 0 as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 95)) as i32, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                    Point::new(0, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                ])
            }
           },
           
    }
}