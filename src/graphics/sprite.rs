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

#[derive()]
pub struct Sprite{
    path:PathBuf,
    width:u32,
    height:u32,
    pos:(i32,i32),
    surface: Surface<'static>,
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
    fn build(&self) -> Box<dyn Drawable>{
        let path = self.path.clone();
        let surface = Surface::load_bmp(&path).unwrap();
        let mut width = surface.width();
        let mut height = surface.height();
        let mut pos = DEFAULT_POS;
        if self.width.is_some(){
            width = self.width.unwrap();
        }
        if self.height.is_some(){
            height = self.height.unwrap();
        }
        if self.pos.is_some(){
            pos = self.pos.unwrap();
        }
        Box::new(Sprite{
            path,
            width,
            height,
            pos,
            surface,
        })
    }
}

impl Drawable for Sprite{
    fn draw(&mut self, canvas: &mut Canvas<Window>){
        let path = self.path.clone();
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
        let texture = texture_creator.create_texture_from_surface(&self.surface).unwrap();
        canvas.copy(&texture,None,rect).unwrap();
    }

    fn get_path(&self)->&path::Path{
        &self.path
    }
}

impl Sizeable for Sprite{
    fn resize(mut self, percentage: f32) -> Sprite{
        let height = self.height as f32;
        let width = self.width as f32;
        let height = (height * percentage) as u32;
        let width = (width * percentage) as u32;
        self.width = width;
        self.height = height;
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

impl Sizeable for SpriteBuilder{
    fn resize(mut self, percentage: f32) -> SpriteBuilder{
        let path = self.path.clone();
        let height;
        let width;
        if self.height.is_none() || self.width.is_none(){
            let surface = Surface::load_bmp(&path).unwrap();
            height = surface.height() as f32;
            width = surface.width() as f32;
        } else {
            height = self.height.unwrap() as f32;
            width = self.width.unwrap() as f32;
        }
        let height = (height * percentage) as u32;
        let width = (width * percentage) as u32;
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    fn width(mut self, width: u32)->SpriteBuilder{
        self.width = Some(width);
        self
    }
    fn height(mut self, height: u32)->SpriteBuilder{
        self.height = Some(height);
        self
    }
    fn get_width(&self)->u32{
        if self.width.is_some(){
            self.width.unwrap()
        } else {
            0
        }
    }
    fn get_height(&self)->u32{
        if self.height.is_some(){
            self.height.unwrap()
        } else {
            0
        }
    }
}

impl Positionable for SpriteBuilder{
    fn center(self, viewport: rect::Rect) -> SpriteBuilder{
        let view_width = viewport.width() as i32;
        let view_height = viewport.height() as i32;
        if self.width.is_none() || self.height.is_none(){
            return self;
        }
        let width = self.width.unwrap() as i32;
        let height = self.height.unwrap() as i32;
        let pos = Some((view_width/2 - width/2,view_height/2 - height/2));
        SpriteBuilder{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        }
    }
    fn downcenter(self, viewport: rect::Rect) -> SpriteBuilder{
        let view_width = viewport.width() as i32;
        let view_height = viewport.height() as i32;
        if self.width.is_none() || self.height.is_none(){
            return self;
        }
        let width = self.width.unwrap() as i32;
        let height = self.height.unwrap() as i32;
        let pos = Some((view_width/2 - width/2,view_height - height));
        SpriteBuilder{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        }
    }
    fn get_pos(&self)->(i32,i32){
        if self.pos.is_none(){
            (0,0)
        } else {
            self.pos.unwrap()
        }
    }
}
