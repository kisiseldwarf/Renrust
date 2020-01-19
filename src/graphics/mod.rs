use std::*;
use sdl2::*;
use std::path::PathBuf;
use image::GenericImageView;

//Only 5 layers are available. But for a game, it should be more than enough
const LAYERS_NB : usize = 5;

//Any graphics that can draw itself
pub trait Drawable{
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>);
}

//Any graphics that can position itself
pub trait Positionable{
    fn center(self,viewport: rect::Rect)->Self;
    fn downcenter(self, viewport: rect::Rect)->Self;
    fn left(self,viewport: rect::Rect) -> Self;
    fn right(self, viewport: rect::Rect) -> Self;
    fn downleft(self, viewport: rect::Rect) -> Self;
    fn downright(self, viewport: rect::Rect) -> Self;
    fn margeleft(self, marge: u32) -> Self;
    fn margeright(self, marge: u32) -> Self;
    fn margetop(self, marge: u32) -> Self;
    fn margedown(self, marge: u32) -> Self;
}

//Any graphcis that can resize itself
pub trait Sizeable{
    //1 = 100%
    fn resize(self,percentage: u32) -> Self;
}

//layers are just a fixed collection (array) of an unknown number of Images
pub struct Layers{
    pub layers: [Vec<Image>;LAYERS_NB],
}

//Those are options types to have a none value, since not all images must implements every attributes
#[derive(Clone)]
pub struct Image{
    path:Option<PathBuf>,
    pub width:Option<u32>,
    pub height:Option<u32>,
    pub pos:Option<(u32,u32)>,
}

#[derive(Clone)]
pub struct Animated{
    frames:Vec<Image>,
    current:usize,
    number:usize,
    fps:u32,
}

impl Sizeable for Image{
    fn resize(mut self, percentage: u32) -> Image{
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

impl Positionable for Image{
    fn center(mut self,viewport: rect::Rect)->Image{
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
    fn downcenter(mut self, viewport: rect::Rect)->Image{
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
    fn left(mut self,viewport: rect::Rect) -> Image{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((0,viewport.height()/2 - height/2));
        }
        self
    }
    fn right(mut self, viewport: rect::Rect) -> Image{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((viewport.width() - width,viewport.height()/2 - height/2));
        }
        self
    }
    fn downleft(mut self, viewport: rect::Rect) -> Image{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((0,viewport.height()/2 - height/2));
        }
        self
    }
    fn downright(mut self, viewport: rect::Rect) -> Image{
        if self.path.is_some(){
            let loaded = image::open(self.get_path()).unwrap();
            let dim = loaded.dimensions();
            let height = dim.0;
            let width = dim.1;
            self.pos = Some((viewport.width() - width,viewport.height()/2 - height/2));
        }
        self
    }
    fn margeleft(mut self, marge: u32) -> Image{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0+marge , self.pos.unwrap().1));
        }
        self
    }
    fn margeright(mut self, marge: u32) -> Image{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0-marge , self.pos.unwrap().1));
        }
        self
    }
    fn margetop(mut self, marge: u32) -> Image{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0 , self.pos.unwrap().1+marge));
        }
        self
    }
    fn margedown(mut self, marge: u32) -> Image{
        if self.pos.is_some(){
            self.pos = Some((self.pos.unwrap().0, self.pos.unwrap().1-marge));
        }
        self
    }
}

impl Drawable for Animated{
    fn draw(self:&mut Animated, canvas: &mut render::Canvas<video::Window>){
        if self.current == self.number-1 {
            self.current = 0;
        } else {
            self.current += 1;
        }
        self.frames[self.current].draw(canvas);
    }
}

impl Drawable for Image{
    fn draw(self:&mut Image, canvas: &mut render::Canvas<video::Window>){
        let surface = surface::Surface::load_bmp(self.path.as_ref().expect("All Images should have a path")).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();
        canvas.copy(&texture,None,None).unwrap();
    }
}

impl Image{
    pub fn new() -> Image{
        let res = Image{
            path:None,
            width:None,
            height:None,
            pos:None,
        };
        res
    }
    pub fn path(mut self:Image,str:PathBuf)->Image{
        self.path = Some(str);
        self
    }
    pub fn width(mut self:Image,w:u32)->Image{
        self.width = Some(w);
        self
    }
    pub fn height(mut self:Image,h:u32)->Image{
        self.height = Some(h);
        self
    }
    pub fn pos(mut self:Image,x:u32,y:u32)->Image{
        self.pos = Some((x,y));
        self
    }
    pub fn get_path<'a>(self:&'a Image)->&'a path::PathBuf{
        &self.path.as_ref().expect("All images should have a Path.")
    }
}
