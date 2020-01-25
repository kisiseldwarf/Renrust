pub mod sprite;
pub mod animated;

use std::*;
use sdl2::*;
use path::*;
use std::boxed::*;


/* TRAITS */


//Any graphics that can draw itself
pub trait Drawable{
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>);
    fn get_path(&self)->&Path;
}

//Any graphics that can position itself
pub trait Positionable{
    fn center(self,viewport: rect::Rect) -> Self;
    fn downcenter(self, viewport: rect::Rect) -> Self;
    fn get_pos(&self)->(i32,i32);
}

//Any graphics that can resize itself
pub trait Sizeable{
    fn resize(self,percentage: f32) -> Self;
    fn width(self,w:u32)->Self;
    fn height(self,h:u32)->Self;
    fn get_width(&self)->u32;
    fn get_height(&self)->u32;
}

//Any graphics that can build itself
pub trait DrawableBuilder{
    fn build(&self) -> Box<dyn Drawable>;
}


/* STRUCTS */


//layers are just a collection of a collection owning an unknown number of Drawable
pub struct Layers{
    pub layers: Vec<Vec<Box<dyn Drawable>>>,
}
