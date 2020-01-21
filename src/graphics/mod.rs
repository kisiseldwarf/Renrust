pub mod sprite;
pub mod animated;

use std::*;
use sdl2::*;
use path::Path;

//Any graphics that can draw itself
pub trait Drawable {
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>);
}

//Any graphics that can position itself
pub trait Positionable{
    fn center(self,viewport: rect::Rect) -> Self;
    fn downcenter(self, viewport: rect::Rect) -> Self;
    fn left(self,viewport: rect::Rect) -> Self;
    fn right(self, viewport: rect::Rect) -> Self;
    fn downleft(self, viewport: rect::Rect) -> Self;
    fn downright(self, viewport: rect::Rect) -> Self;
    // fn margeleft(self, marge: u32) -> Self; //replace by xpos (entre 0 et 1 : pourcentage par rapport à l'écran, sinon pixel absolus) To do
    // fn margeright(self, marge: u32) -> Self; //replace by xpos
    // fn margetop(self, marge: u32) -> Self; //replace by ypos (same que xpos)
    // fn margedown(self, marge: u32) -> Self; //replace by ypos
}

//Any graphics that can resize itself
pub trait Sizeable{
    fn resize(self,percentage: f32) -> Self;
    fn width(self,w:u32)->Self;
    fn height(self,h:u32)->Self;
}

//layers are just a collection of a collection owning an unknown number of Drawable
pub struct Layers{
    pub layers: Vec<Vec<Box<dyn Drawable>>>,
}
