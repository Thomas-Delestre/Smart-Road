use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

pub struct Sprite <'a> {
    pub source: &'a str, 
    rows: u8,
    cols: u8,
    nof: u16,
    steps: Vec<Rect>,
    pub loaded: Option<Texture<'a>>,  
}

impl<'a> Sprite<'a> {
    pub fn new(src: &'a str, rows: u8, cols: u8) -> Sprite<'a> {
        Sprite {
            source: src,
            rows,
            cols,
            nof: (rows * cols) as u16, // nof = number of frames
            steps: Vec::with_capacity((rows*cols) as usize),
            loaded: None,
        }
    }
    pub fn load(&mut self, texture_creator: &'a TextureCreator<WindowContext>) -> Result<(), String> {
        
        self.steps = Vec::with_capacity(self.nof as usize); // setups le tableaux avec la bonne taille pour les frames.


        let texture = texture_creator.load_texture(self.source)?; // charge la texture à partir de la source
        self.loaded = Some(texture); // Stocke directement l'objet Texture
        let frame_size = self.get_frame_size(); 
        for i in 0..self.nof {
            self.steps.push(
                Rect::new(
                    ((i % self.cols as u16) as u32 * frame_size.0) as i32,
                    ((i / self.rows as u16) as u32 * frame_size.1) as i32,
                    frame_size.0,
                    frame_size.1,
                )
            );
        }
        
       
        Ok(())
    } 
    
    pub fn get_frame(&self, id: usize) -> Rect {
        self.steps[id]
    }
    
    pub fn get_frame_size(&self) -> (u32, u32) {
        let size: (u32, u32) = {  
            let query = self.loaded.as_ref().unwrap().query();
            (query.width, query.height) 
        }; // recuperer la taille de la texture

     (size.0 / self.cols as u32, size.1 / self.rows as u32) // recupere la taille d'une frame

    }
}