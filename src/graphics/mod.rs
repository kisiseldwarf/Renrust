pub mod sprite;
pub mod animated;

use std::*;
use sdl2::*;

//Only 5 layers are available. But for a game, it should be more than enough
const LAYERS_NB : usize = 5;

//Any graphics that can draw itself
pub trait Drawable{
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>);
    fn get_path<'a>(&'a self)->&'a path::PathBuf;
    fn get_height(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_pos(&self) -> (u32,u32);
    fn has_sp_height(&self) -> bool;
    fn has_sp_width(&self) -> bool;
    fn has_sp_pos(&self) -> bool;
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

//Any graphics that can resize itself
pub trait Sizeable{
    //1 = 100%
    fn resize(self,percentage: u32) -> Self;
}

//layers are just a fixed collection (array) of an unknown number of Drawable
pub struct Layers{
    pub layers: [Vec<Box<dyn Drawable>>;LAYERS_NB],
}
