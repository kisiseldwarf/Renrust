use std::*;
use sdl2::*;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::surface::*;
use crate::graphics::*;
use std::boxed::*;
use std::path::*;

const DEFAULT_POS : (i32,i32) = (0,0);

/* Struct(s) Data(s) */

#[derive(Clone,Debug)]
pub struct Sprite{
    path:PathBuf,
    width:u32,
    height:u32,
    pos:(i32,i32),
}

#[derive(Clone,Debug)]
pub struct SpriteBuilder{
    path:PathBuf,
    width:Option<u32>,
    height:Option<u32>,
    pos:Option<(i32,i32)>,
}

/* Struct(s) Method(s) */

impl SpriteBuilder{
    pub fn new(src: &Path)->SpriteBuilder{
        let pathbuf = src.to_path_buf();
        SpriteBuilder{
            path: pathbuf,
            width: None,
            height: None,
            pos: None,
        }
    }
    pub fn width(mut self, w: u32)->SpriteBuilder{
        self.width = Some(w);
        self
    }
    pub fn height(mut self, h: u32)->SpriteBuilder{
        self.height = Some(h);
        self
    }
    pub fn pos(mut self, pos: (i32,i32))->SpriteBuilder{
        self.pos = Some(pos);
        self
    }
}

/* Trait(s) Implementation(s) */

impl DrawableBuilder for SpriteBuilder{
    fn build(self) -> Box<dyn Drawable>{
        let path = self.path.clone();
        let surface = Surface::load_bmp(self.path).unwrap();
        let width = surface.width();
        let height = surface.height();
        let pos = DEFAULT_POS;
        if self.width.is_some(){
            let width = self.width.unwrap();
        }
        if self.height.is_some(){
            let height = self.height.unwrap();
        }
        Box::new(Sprite{
            path,
            width,
            height,
            pos,
        })
    }
}

impl Drawable for Sprite{
    fn draw(&mut self, canvas: &mut Canvas<Window>){
        let path = self.path.clone();
        let surface = Surface::load_bmp(path).unwrap();
        let width = self.width;
        let height = self.height;
        let x = self.pos.0;
        let y = self.pos.1;
        let rect = rect::Rect::new(
            x,
            y,
            width,
            height,
        );
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();
        canvas.copy(&texture,None,rect).unwrap();
    }
    fn get_path(&self)->&path::Path{
        &self.path
    }
}

impl Sizeable for Sprite{
    fn resize(mut self, percentage: f32) -> Sprite{
        let path = self.path.clone();
        let surface = surface::Surface::load_bmp(path).unwrap();
        let height = surface.height() as f32;
        let width = surface.width() as f32;
        self.width = (width * percentage) as u32;
        self.height = (height * percentage) as u32;
        self
    }
    fn width(mut self, width: u32)->Sprite{
        self.width = width;
        self
    }
    fn height(mut self, height: u32)->Sprite{
        self.height = height;
        self
    }
    fn get_width(&self)->u32{
        self.width
    }
    fn get_height(&self)->u32{
        self.height
    }
}

impl Positionable for Sprite{
    fn center(self, viewport: rect::Rect) -> Sprite{
        let view_width = viewport.width() as i32;
        let view_height = viewport.height() as i32;
        let width = self.width as i32;
        let height = self.height as i32;
        let pos = (view_width/2 - width/2,view_height/2 - height/2);
        Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        }
    }
    fn downcenter(self, viewport: rect::Rect) -> Sprite{
        let view_width = viewport.width() as i32;
        let view_height = viewport.height() as i32;
        let width = self.width as i32;
        let height = self.height as i32;
        let pos = (view_width/2 - width/2,view_height - height);
        Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        }
    }
    fn get_pos(&self)->(i32,i32){
        self.pos
    }
}
