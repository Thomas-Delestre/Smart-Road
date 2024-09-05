use crate::vehicules::{self, generate_path, Direction, Start, Vehicule};
use crate::sprites::{Sprite};
use sdl2::video::{WindowContext, Window};
use sdl2::render::{Canvas, Texture, TextureCreator};
extern crate rand;
use rand::Rng;
use sdl2::rect::Rect;

pub struct Intersection<'a> {
    cars: Vec<Vehicule<'a>>,
    cross: Vec<Vehicule<'a>>,
    sprite: Sprite<'a>,
    next_id: u16, // Champ pour gérer les identifiants de véhicules
}



impl <'a> Intersection<'a> {

    pub fn new(sprite: Sprite<'a>) -> Intersection<'a> {
        Intersection {
            cars: Vec::new(),
            cross: Vec::new(),
            sprite,
            next_id: 1, // Commence à 1
        }
    }

    pub fn cross_perimeter(&self) -> Rect {
        // Définir et retourner la zone de détection
        Rect::new(265, 265, 270, 270)
    }

    fn is_in_cross(&self, car: &Vehicule, cross: &Rect) -> bool {
        let car_rect = Rect::new(car.x as i32, car.y as i32, 40, 91);
        cross.has_intersection(car_rect)
    }

    pub fn step(&mut self) {
    // Met à jour la position des voitures déjà dans l'intersection
    for car in &mut self.cross {
        car.step();
    }

    // Obtenir la zone de détection
    let detection_zone = self.cross_perimeter();

    // Créer un index pour itérer manuellement
    let mut cars_i = 0;
    while cars_i < self.cars.len() {
        if self.is_in_cross(&self.cars[cars_i], &detection_zone) {
            // Si la voiture est dans l'intersection, la transférer à cross
            let car = self.cars.remove(cars_i); // Supprime la voiture de cars
            self.cross.push(car);       
            for car in &self.cross {
                println!("Car ID: {}, Position: ({}, {})", car.id, car.x, car.y);
            }
        } else {
            cars_i += 1; 
        }
    }

    // Met à jour la position des voitures restantes dans cars
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

    pub fn add_car(&mut self, window_size: (u32, u32), sprite: &'a Sprite<'a>, from: Start) {
        let mut rng = rand::thread_rng();
        let gen: f64 = rng.gen();
     
        let path = match (gen * 3.0).round() as u8 {
            0 => generate_path(from, Direction::Left, window_size, 35),
            1 => generate_path(from, Direction::Right, window_size, 35),
            _ => generate_path(from, Direction::Straigth, window_size, 35),
        };

        // Créer une nouvelle voiture avec l'identifiant actuel
        let mut new_car = Vehicule::new(
            path.steps[0].x as i64,
            path.steps[0].y as i64,
            sprite,
            5, // vitesse
            10, // distance de sécurité
            path,
            (gen * 3.0).round() as u16,
        );

        // Attribuer l'identifiant unique
        new_car.id = self.next_id;
        self.next_id += 1; // Incrémenter l'identifiant pour le prochain véhicule

        // Ajouter la voiture à la liste des voitures
        self.cars.push(new_car);
    }

    pub fn list_vehicles(&self) -> String {
        self.cars.iter()
            .map(|v| format!("ID: {}, Position: ({}, {})", v.id, v.x, v.y))
            .collect::<Vec<String>>()
            .join("\n")
    }
}