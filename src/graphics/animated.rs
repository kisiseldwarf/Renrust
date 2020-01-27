use std::*;
use sdl2::*;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::surface::*;
use crate::graphics::sprite::*;
use crate::graphics::*;
use std::boxed::*;
use std::path::*;

const DEFAULT_FRAMESPEED : u32 = 30;
const FRAMES_ELAPSED_STARTER : u128 = 0;
const CURRENT_STARTER : usize = 0;
const STD_FRAME : usize = 0;
const CURRENT_STEP : usize = 1;
const FRAMES_CNT_START : u32 = 0;

/* Shortcut Function */

pub fn load(src: &Path) -> AnimatedBuilder{
    let mut frames = Vec::<SpriteBuilder>::new();
    if src.is_dir(){
        for entries in std::fs::read_dir(src).unwrap(){
            let entry = entries.unwrap();
            let path = entry.path();
            let ext = path.extension();
            if ext.is_some() && ext.unwrap() == "bmp"{
                frames.push(SpriteBuilder::new(&entry.path()));
            }
        }
    }
    let res = AnimatedBuilder::new().frames(frames);
    res
}

/*  Struct(s) Data(s) */

#[derive()]
pub struct Animated{
    frames:Vec<Box<dyn Drawable>>,
    current:usize,
    framespeed:u32,
    frames_elapsed:u128,
}

#[derive(Clone)]
pub struct AnimatedBuilder{
    frames:Option<Vec<SpriteBuilder>>,
    current:Option<usize>,
    framespeed:Option<u32>,
}

/*  Struct(s) Implementation(s) */

impl AnimatedBuilder{
    pub fn new() -> AnimatedBuilder{
        let res = AnimatedBuilder{
            frames:None,
            current:Some(CURRENT_STARTER),
            framespeed:Some(DEFAULT_FRAMESPEED),
        };
        res
    }
    pub fn frames(mut self,frames:Vec<SpriteBuilder>)->AnimatedBuilder{
        self.frames = Some(frames);
        self
    }
    pub fn framespeed(mut self, fps:u32)->AnimatedBuilder{
        self.framespeed = Some(fps);
        self
    }
}

/* Struct(s) Trait(s) */

impl DrawableBuilder for AnimatedBuilder{
    fn build(&self)->Box<dyn Drawable>{
        let frames = self.frames.clone();
        let framespeed = self.framespeed.unwrap();
        let current = self.current.unwrap();
        let frames_elapsed = FRAMES_ELAPSED_STARTER;
        let mut vec = vec![];
        for frame in frames{
            for fram in frame{
                let fram = fram.build();
                vec.push(fram);
            }
        }
        Box::new(Animated{
            frames: vec,
            current,
            framespeed,
            frames_elapsed,
        })
    }
}

impl Sizeable for Animated{
    fn resize(&mut self, percentage: f32){
        for frame in &mut self.frames{
            frame.resize(percentage);
        }
    }
    fn width(&mut self, w: u32){
        for frame in &mut self.frames{
            frame.width(w);
        }
    }
    fn height(&mut self, h: u32){
        for frame in &mut self.frames{
            frame.height(h);
        }
    }
    fn get_width(&self)->u32{
        return self.frames[STD_FRAME].get_width();
    }
    fn get_height(&self)->u32{
        return self.frames[STD_FRAME].get_height();
    }
}

impl Drawable for Animated{
    fn draw(&mut self, canvas: &mut Canvas<Window>, delta: u128){
        let framespeed_in_millis = ((self.framespeed as f32) / (1000 as f32)); //Nombre de tranisitions par secondes
        if self.frames_elapsed as f32 <= ((1 as f32) / framespeed_in_millis) {
            self.frames_elapsed += delta;
            self.frames[self.current].draw(canvas, delta);
            return;
        }
        self.frames_elapsed = FRAMES_ELAPSED_STARTER;
        if self.current == self.frames.len()-1 {
            self.current = CURRENT_STARTER;
        } else {
            self.current += CURRENT_STEP;
        }
        self.frames[self.current].draw(canvas, delta);
    }

    fn get_path(&self) -> &Path{
        return self.frames[0].get_path();
    }
}

// // impl Positionable for AnimatedBuilder{
// //     fn center(self,viewport: rect::Rect) -> AnimatedBuilder{
// //         let vec = Vec::<Sprite>::new();
// //         let mut res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         if self.frames.is_none(){
// //             return res;
// //         }
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames.unwrap(){
// //             vec.push(i.center(viewport));
// //         }
// //         res.frames = Some(vec);
// //         res
// //     }
// //     fn downcenter(self, viewport: rect::Rect) -> AnimatedBuilder{
// //         let vec = Vec::<Sprite>::new();
// //         let mut res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         if self.frames.is_none(){
// //             return res;
// //         }
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames.unwrap(){
// //             vec.push(i.downcenter(viewport));
// //         }
// //         res.frames = Some(vec);
// //         res
// //     }
// //     fn left(self,viewport: rect::Rect) -> AnimatedBuilder{
// //         let vec = Vec::<Sprite>::new();
// //         let mut res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         if self.frames.is_none(){
// //             return res;
// //         }
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames.unwrap(){
// //             vec.push(i.left(viewport));
// //         }
// //         res.frames = Some(vec);
// //         res
// //     }
// //     fn right(self, viewport: rect::Rect) -> AnimatedBuilder{
// //         let mut vec = Vec::<Sprite>::new();
// //         let mut res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         if self.frames.is_none(){
// //             return res;
// //         }
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames.unwrap(){
// //             vec.push(i.right(viewport));
// //         }
// //         res.frames = Some(vec);
// //         res
// //     }
// //     fn downleft(self, viewport: rect::Rect) -> AnimatedBuilder{
// //         let vec = Vec::<Sprite>::new();
// //         let mut res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         if self.frames.is_none(){
// //             return res;
// //         }
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames.unwrap(){
// //             vec.push(i.downleft(viewport));
// //         }
// //         res.frames = Some(vec);
// //         res
// //     }
// //     fn downright(self, viewport: rect::Rect) -> AnimatedBuilder{
// //         let vec = Vec::<Sprite>::new();
// //         let mut res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         if self.frames.is_none(){
// //             return res;
// //         }
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames.unwrap(){
// //             vec.push(i.downright(viewport));
// //         }
// //         res.frames = Some(vec);
// //         res
// //     }
// // }
// //
// // impl Positionable for Animated{
// //     fn center(self,viewport: rect::Rect) -> Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.center(viewport));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// //     fn downcenter(self, viewport: rect::Rect) -> Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.downcenter(viewport));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// //     fn left(self,viewport: rect::Rect) -> Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.left(viewport));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// //     fn right(self, viewport: rect::Rect) -> Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.right(viewport));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// //     fn downleft(self, viewport: rect::Rect) -> Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.downleft(viewport));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// //     fn downright(self, viewport: rect::Rect) -> Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.downright(viewport));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// // }
// //
// // impl Sizeable for AnimatedBuilder{
// //     fn resize(self,percentage: f32) -> AnimatedBuilder{
// //         let mut vec = Vec::<Sprite>::new();
// //         if self.frames.is_some(){
// //             for i in self.frames.unwrap(){
// //                 vec.push(i.resize(percentage));
// //             }
// //         }
// //         let res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         res
// //     }
// //     fn width(self,w:u32)->AnimatedBuilder{
// //         let mut vec = Vec::<Sprite>::new();
// //         if self.frames.is_some(){
// //             for i in self.frames.unwrap(){
// //                 vec.push(i.width(w));
// //             }
// //         }
// //         let res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         res
// //     }
// //     fn height(self,h:u32)->AnimatedBuilder{
// //         let mut vec = Vec::<Sprite>::new();
// //         if self.frames.is_some(){
// //             for i in self.frames.unwrap(){
// //                 vec.push(i.height(h));
// //             }
// //         }
// //         let res = AnimatedBuilder{
// //             frames:Some(vec),
// //             current:self.current,
// //             framespeed:self.framespeed,
// //         };
// //         res
// //     }
// // }
// //
// // impl Sizeable for Animated{
// //     fn resize(self,percentage: f32) -> Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.resize(percentage));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// //     fn width(self,w:u32)->Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.width(w));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
// //     fn height(self,h:u32)->Animated{
// //         let mut vec = Vec::<Sprite>::new();
// //         for i in self.frames{
// //             vec.push(i.width(h));
// //         }
// //         let res = Animated{
// //             frames:vec,
// //             current:self.current,
// //             framespeed:self.framespeed,
// //             frames_elapsed:self.frames_elapsed,
// //         };
// //         res
// //     }
     // }
