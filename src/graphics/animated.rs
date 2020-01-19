use std::*;
use super::*;

#[derive(Clone)]
pub struct Animated{
    frames:Vec<sprite::Sprite>,
    current:usize,
    number:usize,
    fps:u32,
}

// impl Drawable for Animated{
//     fn draw(self:&mut Animated, canvas: &mut render::Canvas<video::Window>){
//         if self.current == self.number-1 {
//             self.current = 0;
//         } else {
//             self.current += 1;
//         }
//         self.frames[self.current].draw(canvas);
//     }
// }
