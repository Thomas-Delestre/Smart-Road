use std::future;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Point, Rect};

use crate::sprites::Sprite;

#[derive(PartialEq, Debug)]
pub enum Start {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Straigth,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Path {
    pub steps: Vec<Point>,
    pub current_index: usize,
    pub dir: Direction,
    pub from: Start,
    pub ended: bool,
}

impl Path {
    pub fn new(steps: Vec<Point>, dir:Direction, from:Start) -> Path {
        Path {
            steps,
            current_index: 0,
            dir,
            from,
            ended: false,
        }
    }
    pub fn current_target(&self) -> Option<Point> {
        self.steps.get(self.current_index).copied()
    }

    pub fn advance(&mut self) {
        if self.current_index < self.steps.len() - 1 {
            self.current_index += 1;
        }else{
            self.ended = true;
        }
    }
}


pub struct Vehicule<'a> {
    pub id: u16,
    pub frame_id: usize,
    pub x: i64, 
    pub y: i64,
    pub vehicule_size: (u32, u32),
    pub sprite: &'a  Sprite<'a>, 
    pub speed: u8, // vitesse en pixels / frames
    security_distance: u32,
    pub path: Path,
    angle: f64,
    pub out_cross: bool
    
} 

impl<'a> Vehicule<'a>  {
    pub fn new(x: i64, y: i64, sprite: &'a  Sprite<'a> , speed: u8, security_distance: u32, path: Path, frame_id: u16, vehicule_size: (u32, u32)) -> Vehicule<'a> {
        Vehicule {
            id: 0,
            frame_id: frame_id as usize,
            x,
            y,
            vehicule_size,
            sprite,
            speed : speed,
            security_distance,
            path, 
            angle: 0.0,
            out_cross: false
        }
    }

    pub fn check_col(&self, sector: &Vec<Vehicule>) -> bool { // à réécrire prcq de la merde
        
        let mut future_cb: Rect =  self.get_collide_box();

        let dir_target: Point = self.path.current_target().unwrap();
        let dx = (dir_target.x - self.x as i32).signum();
        let dy = (dir_target.y - self.y as i32).signum();
        
        future_cb.x = future_cb.x + dx * self.security_distance as i32 +  dx * self.speed as i32;
        
        future_cb.y = future_cb.y + dy * self.security_distance as i32 +  dy * self.speed as i32;

        println!("test {:?}",  dx * self.security_distance as i32);

        for other in sector {
            if other.id != self.id {
                let obox: Rect = other.get_collide_box();

                if obox.has_intersection(future_cb) {
                   
                        return true;
                    
                }
            }
            
        }
        false
    }
    
    pub fn check_security_distance(&self, other: &Vehicule) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = ((dx * dx + dy * dy) as f64).sqrt();

        distance > self.security_distance as f64
    }

    pub fn step(&mut self) { // Logique de la voiture frame par frame
    
        if let Some(target) = self.path.current_target() {
            // Calculer la direction vers le point de passage suivant
            let dx = target.x - self.x as i32;
            let dy = target.y - self.y as i32;

                // Check des directions pour update les angles
            if dy > 0  { // bas 
                self.angle = 180.0; 
            }
            if dy < 0 { // haut
                self.angle = 0.0;
            }
            if dx > 0  { // droite
                self.angle = 90.0;
            }
            if dx < 0 { // gauche
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
            // println!("TAILLE DE MON SPRITE CAR : {} , {}",(fsize.0 as f32) * xscale, (fsize.1 as f32) * xscale);
            let dest_rect = Rect::new(self.x as i32, self.y as i32, (fsize.0 as f32 * xscale).round() as u32, (fsize.1 as f32 * yscale).round() as u32); // Ajustez la taille selon votre texture
            canvas.copy_ex(texture, Some(src_rect), Some(dest_rect), self.angle, None, false, false)?;


            let mut future_cb: Rect =  self.get_collide_box();

            let dir_target: Point = self.path.current_target().unwrap();
            let dx = (dir_target.x - self.x as i32).signum();
            let dy = (dir_target.y - self.y as i32).signum();
            
            println!("{}, {}", dx, dy);
            future_cb.x = future_cb.x + dx * self.security_distance as i32 +  dx * self.speed as i32;
            
            future_cb.y = future_cb.y + dy * self.security_distance as i32 +  dy * self.speed as i32;


            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(future_cb)?;
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.draw_rect(self.get_collide_box())?;
            
        }else {

        }
        Ok(())
    }


    pub fn get_collide_box(&self) -> Rect {
       
        let mut coll_size: (u32, u32) = self.vehicule_size; 
        let mut coll_pos: (i32, i32) = (self.x as i32, self.y as i32); 

        
        if self.angle == 180.0 || self.angle == 0.0 { // si le vehicule est à l'horizontal je change la width et la height
            coll_size.0 = self.vehicule_size.1;
            coll_size.1 = self.vehicule_size.0;
        }
       
      

        Rect::new(coll_pos.0, coll_pos.1, coll_size.0, coll_size.1)
    }
    pub fn is_in_cross(&self, cross: Rect) -> bool { // Check si le vehicule est au centre de l'intersection
        self.get_collide_box().has_intersection(cross)
    } 

    pub fn distance_to_destiation(&self) -> f64 {
        // Récupérer le point de destination actuel (le dernier point du chemin)
     
            // Calculer la distance entre la position actuelle et la destination
            
            let dx = (self.path.steps[self.path.steps.len() - 1].x - self.x as i32) as f64;
            let dy = (self.path.steps[self.path.steps.len() - 1].y - self.y as i32) as f64;
            let distance = (dx * dx + dy * dy).sqrt();
            distance
    
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
                ],dir, start),
                Direction::Straigth => Path::new(vec![
                    Point::new(0, (window_size.1 / 2 + vehicule_height ) as i32),
                    Point::new(window_size.0 as i32, (window_size.1 / 2 + vehicule_height) as i32),
                ],dir, start),
                Direction::Right => Path::new(vec![
                    Point::new(0, (window_size.1 / 2 + 80) as i32),
                    Point::new((window_size.0 / 3)  as i32, (window_size.1 / 2 + 80) as i32),
                    Point::new((window_size.0 / 3 + 7) as i32, (window_size.1) as i32),
                ],dir, start)
            }
        },
           Start::South => {
            match dir {
                Direction::Left => Path::new(vec![
                    Point::new((window_size.0 / 2 + 7) as i32, window_size.1 as i32),
                    Point::new((window_size.0 / 2 + 7) as i32, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                    Point::new(0, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                ],dir, start),
                Direction::Straigth => Path::new(vec![
                    Point::new((window_size.0 / 2 + vehicule_height + 10) as i32, window_size.1 as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 10) as i32, 0),
                ],dir, start),
                Direction::Right => Path::new(vec![
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, window_size.1 as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, (window_size.1 / 2 + vehicule_height + 45) as i32),
                    Point::new(window_size.0 as i32, (window_size.1 / 2 + vehicule_height + 45) as i32),
                ],dir, start)
            }
           },
           Start::East => {
            match dir {
                Direction::Left => Path::new(vec![
                    Point::new(window_size.0 as i32, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, (window_size.1 / 2 - (vehicule_height + 15)) as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, window_size.1 as i32),
                ],dir, start),
                Direction::Straigth => Path::new(vec![
                    Point::new(window_size.0 as i32, (window_size.1 / 2 - (vehicule_height + 57)) as i32),
                    Point::new(0 as i32, (window_size.1 / 2 - (vehicule_height + 57)) as i32),
                ],dir, start),
                Direction::Right => Path::new(vec![
                    Point::new(window_size.0 as i32, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                    Point::new((window_size.0 / 2 + vehicule_height + 55) as i32, 0 as i32),
                ],dir, start)
            }
           },
           Start::North => {
            match dir {
                Direction::Left => Path::new(vec![
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, 0),
                    Point::new((window_size.0 / 2 - (vehicule_height + 10)) as i32, (window_size.1 / 2) as i32),
                    Point::new(window_size.0 as i32, (window_size.1 / 2) as i32),
                ],dir, start),
                Direction::Straigth => Path::new(vec![
                    Point::new((window_size.0 / 2 - (vehicule_height + 50)) as i32, 0 as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 50)) as i32, window_size.1 as i32),
                ],dir, start),
                Direction::Right => Path::new(vec![
                    Point::new((window_size.0 / 2 - (vehicule_height + 95)) as i32, 0 as i32),
                    Point::new((window_size.0 / 2 - (vehicule_height + 95)) as i32, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                    Point::new(0, (window_size.1 / 2 - (vehicule_height + 100)) as i32),
                ],dir, start)
            }
        },   
    }
}