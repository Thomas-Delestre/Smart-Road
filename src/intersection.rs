use crate::vehicules::{ generate_path, Direction, Start, Vehicule};
use crate::sprites::{Sprite};
use sdl2::video::Window;
use sdl2::render::Canvas;
extern crate rand;
use rand::Rng;
use sdl2::rect::Rect;

pub struct Intersection<'a> {
    pub cars: Vec<Vehicule<'a>>,
    pub finished_vehicles: Vec<Vehicule<'a>>,
    cross: Vec<Vehicule<'a>>,
    sprite: Sprite<'a>,
    next_id: u16, // Champ pour gérer les identifiants de véhicules
    speeds: Vec<u8>,
    cross_perimeter: Rect,
}


impl <'a> Intersection<'a> {

    pub fn new(sprite: Sprite<'a>) -> Intersection<'a> {
        Intersection {
            cars: Vec::new(),
            finished_vehicles: Vec::new(),
            cross: Vec::new(),
            sprite,
            next_id: 1, // Commence à 1
            speeds: vec![4, 8, 12],
            cross_perimeter:  Rect::new(250, 250, 300, 300),
        }
    }
    pub fn step(&mut self) {

        // let mut to_remove: Vec<usize> = Vec::new();


        // Met à jour la position des voitures déjà dans l'intersection
        if !self.cross.is_empty() {

            for i in 0..self.cross.len() {

                let mut prio: bool = true;

                for j in 0..i {

                    if self.cross[i].path.from != self.cross[j].path.from {
                        prio = false;
                        break;
                    }
                }
                
                if prio {
                    self.cross[i].speed = self.speeds[2]; 
                    self.cross[i].update_speed(self.speeds[2]); 

                }else{
                    
                    self.cross[i].speed = self.speeds[0];
                    self.cross[i].update_speed(self.speeds[0]); 
                }
                
               // Crée une copie temporaire de `self.cross` à utiliser dans `check_col`
                let cross_clone = self.cross.clone();

                // Accède à `self.cross[i]` et effectue la vérification avec la copie
                if !self.cross[i].check_col(&cross_clone) {
                    self.cross[i].step();
                }

                
               

            }
        }

        // Gérer les voitures en dehors de l'intersection
       
        let mut cars_i = 0;
        
        while cars_i < self.cars.len() {
            
            if self.cars[cars_i].is_in_cross(self.cross_perimeter) && self.cars[cars_i].path.dir != Direction::Right {

                
                // Si la voiture est dans l'intersection, la transférer à cross
                if self.cross.len() < 6 {
                    
                    let car = self.cars.remove(cars_i);
                    if car.path.dir == Direction::Left {
                        if self.count_dir_in_cross(Direction::Left) < 3 {
                            self.cross.push(car);
                        }else{
                            cars_i += 1;
                        }
                    }else{
                        self.cross.push(car);
                    }
                    

                   
                    
                }else{
                    cars_i += 1;
                }
            } else {
                cars_i += 1;
            }
        }

        cars_i = 0;
        while cars_i < self.cross.len() {
            if !self.cross[cars_i].is_in_cross(self.cross_perimeter) && self.cross[cars_i].path.dir != Direction::Right {
                // Si la voiture est dans l'intersection, la transférer à cross
                let mut car = self.cross.remove(cars_i);
                car.out_cross = true;
                self.cars.push(car);
            } else {
                cars_i += 1;
            }
        }
            
        let mut to_remove = Vec::new();

            for i in 0..self.cars.len() {
                self.cars[i].speed = self.speeds[1];
                self.cars[i].update_speed(self.speeds[1]);
                if  self.cars[i].path.dir == Direction::Right || self.cars[i].out_cross {
                    self.cars[i].speed = self.speeds[2];
                    self.cars[i].update_speed(self.speeds[2]);
                }
               // Crée des copies temporaires de `self.cross` et `self.cars`
                let cross_clone = self.cross.clone();
                let cars_clone = self.cars.clone();

                // Utilise les copies pour éviter les conflits d'emprunt
                if !self.cars[i].check_col(&cross_clone) && !self.cars[i].check_col(&cars_clone) {
                    if !self.cars[i].is_in_cross(self.cross_perimeter) || self.cars[i].path.dir == Direction::Right {
                        self.cars[i].step();
                    }
                }


                if self.cars[i].path.ended {
                    // Met à jour le temps total que la voiture a pris pour traverser l'intersection
                    self.cars[i].total_time = Some(self.cars[i].car_in.elapsed());
                
                    // Ajoute le véhicule à finished_vehicles avant de le marquer pour suppression
                    self.finished_vehicles.push(self.cars[i].clone());
                    to_remove.push(i);
                }    
            }
            
            for (i, car_index) in to_remove.iter().enumerate() {
                self.cars.remove(*car_index - i);
            }

        // Supprimer les voitures qui ont atteint leur destination
     
    }

    fn count_dir_in_cross(&self, dir: Direction) -> u8 {
        let mut count: u8 = 0;
        for checkc in &self.cross {
            if checkc.path.dir == dir {
                count = count + 1;
            }
        }
        count
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

    pub fn add_car(&mut self, window_size: (u32, u32), sprite: &'a Sprite<'a>, from: Start, vehicule_size: (u32, u32)) {
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
            self.speeds[1], // vitesse
            10, // distance de sécurité
            path,
            (gen * 3.0).round() as u16,
            vehicule_size,
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