pub mod sprite;
pub mod animated;
use std::*;
use sdl2::*;
use path::*;
use std::boxed::*;


////* TRAITS *////
//Any graphics that can draw itself
pub trait Drawable : Sizeable{
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>);
    fn get_path(&self)->&Path;
}

//Any graphics that can position itself
pub trait Positionable{
    fn center(&mut self,viewport: rect::Rect);
    fn downcenter(&mut self, viewport: rect::Rect);
    fn get_pos(&self)->(i32,i32);
}

//Any graphics that can resize itself
pub trait Sizeable{
    fn resize(&mut self,percentage: f32);
    fn width(&mut self,w:u32);
    fn height(&mut self,h:u32);
    fn get_width(&self)->u32;
    fn get_height(&self)->u32;
}

//Any graphics that can build itself
pub trait DrawableBuilder{
    fn build(&self) -> Box<dyn Drawable>;
}


////* STRUCTS *////
pub struct Layers{
    pub layers: Vec<Vec<Box<dyn Drawable>>>,
}
