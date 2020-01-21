use std::*;
use sdl2::*;
use std::path::PathBuf;
use image::GenericImageView;
use std::path::Path;
use super::*;

//Those are options types to have a none value, since not all Sprites must implements every attributes
#[derive(Clone,Debug)]
pub struct Sprite{
    path:Option<PathBuf>,
    pub width:Option<u32>,
    pub height:Option<u32>,
    pub pos:Option<(i32,i32)>,
}

pub fn load(src: &Path) -> Sprite{
    let res = Sprite::new().path(src.to_path_buf());
    res
}

impl Drawable for Sprite{
    fn draw(self:&mut Sprite, canvas: &mut render::Canvas<video::Window>){
        let surface = surface::Surface::load_bmp(self.get_path()).unwrap();
        let mut x = 0;
        let mut y = 0;
        let mut width = surface.width();
        let mut height = surface.height();
        if self.width.is_some()
            { width = self.width.unwrap(); }
        if self.height.is_some()
            { height = self.height.unwrap(); }
        if self.pos.is_some(){
             x = self.pos.unwrap().0;
             y = self.pos.unwrap().1;
         }
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
}

impl Sprite{
    pub fn new() -> Sprite{
        let res = Sprite{
            path:None,
            width:None,
            height:None,
            pos:None,
        };
        res
    }
    pub fn path(mut self:Sprite,str:PathBuf)->Sprite{
        self.path = Some(str);
        self
    }
    pub fn width(mut self:Sprite,w:u32)->Sprite{
        self.width = Some(w);
        self
    }
    pub fn height(mut self:Sprite,h:u32)->Sprite{
        self.height = Some(h);
        self
    }
    pub fn pos(mut self:Sprite,x:i32,y:i32)->Sprite{
        self.pos = Some((x,y));
        self
    }
    fn get_path<'a>(self:&'a Sprite)->&'a path::PathBuf{
        &self.path.as_ref().expect("All Sprites should have a Path.")
    }
}

impl Sizeable for Sprite{
    fn resize(mut self, percentage: f32) -> Sprite{
        if self.path.is_some(){
            let surface = surface::Surface::load_bmp(self.get_path()).unwrap();
            let height = surface.height() as f32;
            let width = surface.width() as f32;
            self.width = Some((width * percentage) as u32);
            self.height = Some((height * percentage) as u32);
        }
        self
    }
    fn width(mut self:Sprite,w:u32)->Sprite{
        let a = w;
        self.width = Some(a);
        self
    }
    fn height(mut self:Sprite,h:u32)->Sprite{
        let a = h;
        self.height = Some(a);
        self
    }
}

impl Positionable for Sprite{
    fn center(self, viewport: rect::Rect) -> Sprite{
        let view_width = viewport.width() as i32;
        let view_height = viewport.height() as i32;
        let mut pos = self.pos;
        if self.width.is_some() && self.height.is_some(){
            let width = self.width.unwrap() as i32;
            let height = self.height.unwrap() as i32;
            pos = Some((view_width/2 - width/2,view_height/2 - height/2));
        } else {
            if self.path.is_some(){
                let surface = surface::Surface::load_bmp(self.get_path()).unwrap();
                let height = surface.height() as i32;
                let width = surface.width() as i32;
                pos = Some((view_width/2 - width/2,view_height/2 - height/2));
            }
        }
        let res = Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        };
        res
    }
    fn downcenter(self, viewport: rect::Rect) -> Sprite{
        let mut pos = self.pos;
        let view_width = viewport.width() as i32;
        let view_height = viewport.height() as i32;
        if self.width.is_some() && self.height.is_some(){
            let width = self.width.unwrap() as i32;
            let height = self.height.unwrap() as i32;
            pos = Some((view_width/2 - width/2,view_height - height));
        } else {
            if self.path.is_some(){
                let surface = surface::Surface::load_bmp(self.get_path()).unwrap();
                let height = surface.height() as i32;
                let width = surface.width() as i32;
                pos = Some((view_width/2 - width/2,view_height - height));
            }
        }
        let res = Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        };
        res
    }
    fn left(self, viewport: rect::Rect) -> Sprite{
        let mut pos = self.pos;
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0 as i32;
            let view_height = viewport.height() as i32;
            pos = Some(( 0, view_height/2 - height/2));
        }
        let res = Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        };
        res
    }
    fn right(self, viewport: rect::Rect) -> Sprite{
        let mut pos = self.pos;
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0 as i32;
            let width = dim.1 as i32;
            let view_width = viewport.width() as i32;
            let view_height = viewport.height() as i32;
            pos = Some((view_width - width, view_height/2 - height/2));
        }
        let res = Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        };
        res
    }
    fn downleft(self, viewport: rect::Rect) -> Sprite{
        let mut pos = self.pos;
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0 as i32;
            let view_height = viewport.height() as i32;
            pos = Some((0, view_height/2 - height/2));
        }
        let res = Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        };
        res
    }
    fn downright(self, viewport: rect::Rect) -> Sprite{
        let mut pos = self.pos;
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0 as i32;
            let width = dim.1 as i32;
            let view_width = viewport.width() as i32;
            let view_height = viewport.height() as i32;
            pos = Some((view_width- width,view_height/2 - height/2));
        }
        let res = Sprite{
            path: self.path,
            width: self.width,
            height: self.height,
            pos,
        };
        res
    }
}
