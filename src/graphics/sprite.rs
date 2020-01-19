use std::*;
use sdl2::*;
use std::path::PathBuf;
use image::GenericImageView;
use super::Drawable;

//Those are options types to have a none value, since not all Sprites must implements every attributes
#[derive(Clone)]
pub struct Sprite{
    path:Option<PathBuf>,
    pub width:Option<u32>,
    pub height:Option<u32>,
    pub pos:Option<(u32,u32)>,
}

impl super::Drawable for Sprite{
    fn draw(self:&mut Sprite, canvas: &mut render::Canvas<video::Window>){
        let surface = surface::Surface::load_bmp(self.get_path()).unwrap();
        let mut x = 0;
        let mut y = 0;
        let mut width = surface.width();
        let mut height = surface.height();
        if self.has_sp_width()
            { width = self.get_width(); }
        if self.has_sp_height()
            { height = self.get_height(); }
        if self.has_sp_pos(){
             x = self.get_pos().0 as i32;
             y = self.get_pos().1 as i32;
         }
        let rect = rect::Rect::new(
            x,
            y,
            width,
            height,
        );
        let old = canvas.viewport();
        canvas.set_viewport(rect);
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();
        canvas.copy(&texture,None,None).unwrap();
        canvas.set_viewport(old);
    }

    fn get_path<'a>(self:&'a Sprite)->&'a path::PathBuf{
        &self.path.as_ref().expect("All Sprites should have a Path.")
    }
    fn get_width(&self) -> u32{
        let res = self.width.unwrap();
        res
    }
    fn get_height(&self) -> u32{
        let res = self.height.unwrap();
        res
    }
    fn get_pos(&self) -> (u32,u32){
        let res = self.pos.unwrap();
        res
    }
    fn has_sp_width(&self) -> bool{
        self.width.is_some()
    }
    fn has_sp_height(&self) -> bool{
        self.height.is_some()
    }
    fn has_sp_pos(&self) -> bool{
        self.pos.is_some()
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
    pub fn pos(mut self:Sprite,x:u32,y:u32)->Sprite{
        self.pos = Some((x,y));
        self
    }
}

impl super::Sizeable for Sprite{
    fn resize(mut self, percentage: u32) -> Sprite{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.width = Some(width * percentage);
            self.height = Some(height * percentage);
        }
        self
    }
}

impl super::Positionable for Sprite{
    fn center(mut self,viewport: rect::Rect)->Sprite{
        if self.width.is_some() && self.height.is_some(){
            self.pos = Some((viewport.width()/2 - self.width.unwrap()/2,viewport.height()/2 - self.height.unwrap()/2));
        } else {
            if self.path.is_some(){
                let loaded = image::open(self.get_path()).unwrap();
                let dim = loaded.dimensions();
                let height = dim.0;
                let width = dim.1;
                self.pos = Some((viewport.width()/2 - width/2,viewport.height()/2 - height/2));
            }
        }
        self
    }
    fn downcenter(mut self, viewport: rect::Rect)->Sprite{
        if self.width.is_some() && self.height.is_some(){
            self.pos = Some((viewport.width()/2 - self.width.unwrap()/2,viewport.height() - self.height.unwrap()));
        } else {
            if self.path.is_some(){
                let loaded = image::open(self.get_path()).unwrap();
                let dim = loaded.dimensions();
                let height = dim.0;
                let width = dim.1;
                self.pos = Some((viewport.width()/2 - width/2,viewport.height() - height));
            }
        }
        self
    }
    fn left(mut self,viewport: rect::Rect) -> Sprite{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((0,viewport.height()/2 - height/2));
        }
        self
    }
    fn right(mut self, viewport: rect::Rect) -> Sprite{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((viewport.width() - width,viewport.height()/2 - height/2));
        }
        self
    }
    fn downleft(mut self, viewport: rect::Rect) -> Sprite{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((0,viewport.height()/2 - height/2));
        }
        self
    }
    fn downright(mut self, viewport: rect::Rect) -> Sprite{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((viewport.width() - width,viewport.height()/2 - height/2));
        }
        self
    }
    fn margeleft(mut self, marge: u32) -> Sprite{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0+marge , self.pos.unwrap().1));
        }
        self
    }
    fn margeright(mut self, marge: u32) -> Sprite{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0-marge , self.pos.unwrap().1));
        }
        self
    }
    fn margetop(mut self, marge: u32) -> Sprite{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0 , self.pos.unwrap().1+marge));
        }
        self
    }
    fn margedown(mut self, marge: u32) -> Sprite{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0, self.pos.unwrap().1-marge));
        }
        self
    }
}
