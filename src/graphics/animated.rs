// use std::*;
// use super::*;
// use super::sprite::*;
// use super::Sizeable;
//
// const DEFAULT_FRAMESPEED : u32 = 1;
//
// #[derive(Clone, Debug)]
// pub struct Animated{
//     frames:Vec<sprite::Sprite>,
//     current:usize,
//     framespeed:u32,
//     frames_elapsed:u32,
// }
//
// pub struct AnimatedBuilder{
//     frames:Option<Vec<sprite::Sprite>>,
//     current:Option<usize>,
//     framespeed:Option<u32>,
// }
//
// //Load every *.bmp in a directory and add it to its frames
// pub fn load(src: &Path) -> AnimatedBuilder{
//     let mut frames = Vec::<sprite::Sprite>::new();
//     if src.is_dir(){
//         for entries in std::fs::read_dir(src).unwrap(){
//             let entry = entries.unwrap();
//             let path = entry.path();
//             let ext = path.extension();
//             if ext.is_some() && ext.unwrap() == "bmp"{
//                 frames.push(sprite::load(&entry.path()));
//             }
//         }
//     }
//     let res = Animated::new().frames(frames);
//     res
// }
//
// impl AnimatedBuilder{
//     pub fn new() -> AnimatedBuilder{
//         let res = AnimatedBuilder{
//             frames:None,
//             current:Some(0),
//             framespeed:None,
//         };
//         res
//     }
//     pub fn frames(mut self,frames:Vec<sprite::Sprite>)->AnimatedBuilder{
//         self.frames = Some(frames);
//         self
//     }
//     pub fn framespeed(mut self, fps:u32)->AnimatedBuilder{
//         self.framespeed = Some(fps);
//         self
//     }
//     pub fn build(self)->Animated{
//         let mut framespeed = DEFAULT_FRAMESPEED;
//         let frames = self.frames.expect("An Animated must have frames");
//
//         if self.framespeed.is_some(){
//             framespeed = self.framespeed.unwrap();
//         }
//         let res = Animated{
//             frames,
//             current:0,
//             framespeed,
//             frames_elapsed:0,
//         };
//         res
//     }
// }
//
// impl Drawable for Animated{
//     fn draw(self:&mut Animated, canvas: &mut render::Canvas<video::Window>){
//         if self.frames_elapsed != self.framespeed {
//             self.frames_elapsed += 1;
//             self.frames[self.current].draw(canvas);
//             return;
//         }
//         self.frames_elapsed = 0;
//
//         if self.current == self.frames.len()-1 {
//             self.current = 0;
//         } else {
//             self.current += 1;
//         }
//         self.frames[self.current].draw(canvas);
//     }
// }
//
// impl Positionable for AnimatedBuilder{
//     fn center(self,viewport: rect::Rect) -> AnimatedBuilder{
//         let vec = Vec::<Sprite>::new();
//         let mut res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         if self.frames.is_none(){
//             return res;
//         }
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames.unwrap(){
//             vec.push(i.center(viewport));
//         }
//         res.frames = Some(vec);
//         res
//     }
//     fn downcenter(self, viewport: rect::Rect) -> AnimatedBuilder{
//         let vec = Vec::<Sprite>::new();
//         let mut res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         if self.frames.is_none(){
//             return res;
//         }
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames.unwrap(){
//             vec.push(i.downcenter(viewport));
//         }
//         res.frames = Some(vec);
//         res
//     }
//     fn left(self,viewport: rect::Rect) -> AnimatedBuilder{
//         let vec = Vec::<Sprite>::new();
//         let mut res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         if self.frames.is_none(){
//             return res;
//         }
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames.unwrap(){
//             vec.push(i.left(viewport));
//         }
//         res.frames = Some(vec);
//         res
//     }
//     fn right(self, viewport: rect::Rect) -> AnimatedBuilder{
//         let mut vec = Vec::<Sprite>::new();
//         let mut res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         if self.frames.is_none(){
//             return res;
//         }
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames.unwrap(){
//             vec.push(i.right(viewport));
//         }
//         res.frames = Some(vec);
//         res
//     }
//     fn downleft(self, viewport: rect::Rect) -> AnimatedBuilder{
//         let vec = Vec::<Sprite>::new();
//         let mut res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         if self.frames.is_none(){
//             return res;
//         }
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames.unwrap(){
//             vec.push(i.downleft(viewport));
//         }
//         res.frames = Some(vec);
//         res
//     }
//     fn downright(self, viewport: rect::Rect) -> AnimatedBuilder{
//         let vec = Vec::<Sprite>::new();
//         let mut res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         if self.frames.is_none(){
//             return res;
//         }
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames.unwrap(){
//             vec.push(i.downright(viewport));
//         }
//         res.frames = Some(vec);
//         res
//     }
// }
//
// impl Positionable for Animated{
//     fn center(self,viewport: rect::Rect) -> Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.center(viewport));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
//     fn downcenter(self, viewport: rect::Rect) -> Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.downcenter(viewport));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
//     fn left(self,viewport: rect::Rect) -> Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.left(viewport));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
//     fn right(self, viewport: rect::Rect) -> Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.right(viewport));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
//     fn downleft(self, viewport: rect::Rect) -> Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.downleft(viewport));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
//     fn downright(self, viewport: rect::Rect) -> Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.downright(viewport));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
// }
//
// impl Sizeable for AnimatedBuilder{
//     fn resize(self,percentage: f32) -> AnimatedBuilder{
//         let mut vec = Vec::<Sprite>::new();
//         if self.frames.is_some(){
//             for i in self.frames.unwrap(){
//                 vec.push(i.resize(percentage));
//             }
//         }
//         let res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         res
//     }
//     fn width(self,w:u32)->AnimatedBuilder{
//         let mut vec = Vec::<Sprite>::new();
//         if self.frames.is_some(){
//             for i in self.frames.unwrap(){
//                 vec.push(i.width(w));
//             }
//         }
//         let res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         res
//     }
//     fn height(self,h:u32)->AnimatedBuilder{
//         let mut vec = Vec::<Sprite>::new();
//         if self.frames.is_some(){
//             for i in self.frames.unwrap(){
//                 vec.push(i.height(h));
//             }
//         }
//         let res = AnimatedBuilder{
//             frames:Some(vec),
//             current:self.current,
//             framespeed:self.framespeed,
//         };
//         res
//     }
// }
//
// impl Sizeable for Animated{
//     fn resize(self,percentage: f32) -> Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.resize(percentage));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
//     fn width(self,w:u32)->Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.width(w));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
//     fn height(self,h:u32)->Animated{
//         let mut vec = Vec::<Sprite>::new();
//         for i in self.frames{
//             vec.push(i.width(h));
//         }
//         let res = Animated{
//             frames:vec,
//             current:self.current,
//             framespeed:self.framespeed,
//             frames_elapsed:self.frames_elapsed,
//         };
//         res
//     }
// }
//
// impl Animated{
//     pub fn new()->AnimatedBuilder{
//         let res = AnimatedBuilder{
//             frames:None,
//             current:Some(0),
//             framespeed:None,
//         };
//         res
//     }
// }
