use crate::vehicules::{generate_path, Direction, Start, Vehicule};
use crate::sprites::{Sprite};
use sdl2::video::{WindowContext, Window};
use sdl2::render::{Canvas, Texture, TextureCreator};
extern crate rand;
use rand::Rng;
pub struct Intersection<'a> {
    cars: Vec<Vehicule<'a>>,
    cross: Vec<Vehicule<'a>>,
    sprite: Sprite<'a>,
}

impl <'a> Intersection<'a> {
    pub fn new(sprite: Sprite<'a>) -> Intersection<'a> {
        Intersection {
            cars: Vec::new(),
            cross: Vec::new(),
            sprite,
        }
    }
    pub fn step(&mut self) {
        // Met à jour la position des voitures
        for car in &mut self.cross {
            car.step();
        }
        for car in &mut self.cars {
            car.step();
        }

        // Supprime les voitures qui ont atteint leur destination
        self.cars.retain(|car| !car.has_reached_destination());
        self.cross.retain(|car| !car.has_reached_destination());
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if let Some(texture) = &self.sprite.loaded {
            canvas.copy(texture, None, None)?;
        }
        for car in self.cars.iter() {
            car.draw(canvas, 0.2, 0.2)?;
        }
        for car in self.cross.iter() {
            car.draw(canvas, 0.2, 0.2)?;
        }

        Ok(())
    }
    pub fn add_car(&mut self, window_size: (u32, u32) ,sprite: &'a Sprite<'a>, from: Start) { // Ajoute une voiture à l'intersection
        let mut rng = rand::thread_rng();
        let gen: f64 = rng.gen();
     
        let path = match (gen * 3.0).round() as u8 {
            0 => {
                generate_path(from, Direction::Left, window_size, 35)
            },
            1 => {
                generate_path(from, Direction::Right, window_size, 35)
            },
            _ => {
                generate_path(from, Direction::Straigth, window_size, 35)
            },
        };
     
       self.cars.push(Vehicule::new(path.steps[0].x as i64, path.steps[0].y as i64, sprite, 5, 10, path, (gen * 3.0).round() as u16));
    }

    pub fn list_vehicles(&self) -> String {
        self.cars.iter()
            .map(|v| format!("ID: {}, Position: ({}, {})", v.id, v.x, v.y))
            .collect::<Vec<String>>()
            .join("\n")
    }
}