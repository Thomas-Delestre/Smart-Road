use std::collections::HashMap;

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
    speeds: Vec<u8>,
    cross_perimeter: Rect,
}

impl <'a> Intersection<'a> {

    pub fn new(sprite: Sprite<'a>) -> Intersection<'a> {
        Intersection {
            cars: Vec::new(),
            cross: Vec::new(),
            sprite,
            next_id: 1, // Commence à 1
            speeds: vec![4, 8, 12],
            cross_perimeter:  Rect::new(250, 250, 300, 300),
        }
    }

    pub fn cross_perimeter(&self) -> Rect {
        // Définir et retourner la zone de détection
        Rect::new(250, 250, 300, 300)
    }

    

    pub fn step(&mut self) {

        let mut to_remove: Vec<usize> = Vec::new();


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
                }else{
                    
                    self.cross[i].speed = self.speeds[0]
                }
                if !self.cross[i].check_col(&self.cross) {
                    self.cross[i].step();
                }
               

            }

            // Accélérer la première voiture dans l'intersection
            // self.cross[0].speed = self.speeds[2];
            // self.cross[0].step();
        
            // // Accélérer les voitures venant de la même direction
            // for i in 1..self.cross.len() {
            //     if self.cross[i].path.from == self.cross[0].path.from {
            //         self.cross[i].speed = self.speeds[2];
            //     } else {
            //         self.cross[i].speed = self.speeds[0]; // Les autres voitures dans l'intersection ralentissent
            //     }
            //     self.cross[i].step();
            // }
        }

        // Gérer les voitures en dehors de l'intersection
       
        let mut cars_i = 0;
        
        while cars_i < self.cars.len() {
            
            if self.cars[cars_i].is_in_cross(self.cross_perimeter) && self.cars[cars_i].path.dir != Direction::Right {

                
                // Si la voiture est dans l'intersection, la transférer à cross
                if self.cross.len() < 6 {
                    let car = self.cars.remove(cars_i);
                    self.cross.push(car);
                    
                }else{
                    cars_i += 1;
                }

                
                
                print!("Switch");
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
            
                print!("Switch");
            } else {
                cars_i += 1;
            }
        }

        // print!("{} len cross",self.cross.len());
        // Gérer les voitures à l'extérieur de l'intersection
        // for car in &mut self.cars {
        //     if car.path.dir == Direction::Right {
        //         car.speed = self.speeds[2]; // Vitesse normale pour les voitures allant à droite
        //     }
        // }

        // Si l'intersection n'est pas vide
        // if !self.cross.is_empty() {
        //     if !self.cars.is_empty() {
        //         // Gérer la première voiture en dehors de l'intersection
        //         self.cars[0].speed = self.speeds[1];
        //         self.cars[0].step();

        //         // Ralentir les autres voitures en attente
        //         for i in 1..self.cars.len() {
        //             self.cars[i].speed = self.speeds[0];
        //             self.cars[i].step();
        //         }
        //     }
        // } else {
            //toutes les voitures avancent normalement
            
            for i in 0..self.cars.len() {
                self.cars[i].speed = self.speeds[1];
                if  self.cars[i].path.dir == Direction::Right || self.cars[i].out_cross {
                    self.cars[i].speed = self.speeds[2];
                }
                if !self.cars[i].check_col(&self.cross) &&  !self.cars[i].check_col(&self.cars) {
                    if !self.cars[i].is_in_cross(self.cross_perimeter) || self.cars[i].path.dir == Direction::Right {
                        self.cars[i].step();
                    }
                    
                }
                
                if  self.cars[i].path.ended {
                    to_remove.push(i);
                }
            }

            for (i, car)  in to_remove.iter().enumerate(){
                self.cars.remove(car - i);
            }
        // }

        // Supprimer les voitures qui ont atteint leur destination
     
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