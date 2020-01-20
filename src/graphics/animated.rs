use std::*;
use super::*;

const DEFAULT_FRAMESPEED : u32 = 2;

#[derive(Clone, Debug)]
pub struct Animated{
    frames:Vec<sprite::Sprite>,
    current:usize,
    framespeed:u32,
    frames_elapsed:u32,
}

pub struct AnimatedBuilder{
    frames:Option<Vec<sprite::Sprite>>,
    current:Option<usize>,
    framespeed:Option<u32>,
}

//Load every *.bmp in a directory and add it to its frames
pub fn load(src: &Path) -> AnimatedBuilder{
    let mut frames = Vec::<sprite::Sprite>::new();
    if src.is_dir(){
        for entries in std::fs::read_dir(src).unwrap(){
            let entry = entries.unwrap();
            let path = entry.path();
            let ext = path.extension();
            if ext.is_some() && ext.unwrap() == "bmp"{
                frames.push(sprite::load(&entry.path()));
            }
        }
    }
    let res = Animated::new().frames(frames);
    res
}

impl AnimatedBuilder{
    pub fn new() -> AnimatedBuilder{
        let res = AnimatedBuilder{
            frames:None,
            current:Some(0),
            framespeed:None,
        };
        res
    }
    pub fn frames(mut self,frames:Vec<sprite::Sprite>)->AnimatedBuilder{
        self.frames = Some(frames);
        self
    }
    pub fn framespeed(mut self, fps:u32)->AnimatedBuilder{
        self.framespeed = Some(fps);
        self
    }
    pub fn build(mut self)->Animated{
        let mut frames;
        let mut framespeed = DEFAULT_FRAMESPEED;

        frames = self.frames.expect("An Animated must have frames");
        if self.framespeed.is_some(){
            framespeed = self.framespeed.unwrap();
        }
        let res = Animated{
            frames,
            current:0,
            framespeed,
            frames_elapsed:0,
        };
        res
    }
}

impl Drawable for Animated{
    fn draw(self:&mut Animated, canvas: &mut render::Canvas<video::Window>){
        if(self.frames_elapsed != self.framespeed){
            self.frames_elapsed += 1;
            self.frames[self.current].draw(canvas);
            return;
        }
        self.frames_elapsed = 0;

        if self.current == self.frames.len()-1 {
            self.current = 0;
        } else {
            self.current += 1;
        }
        self.frames[self.current].draw(canvas);
    }
}

impl Animated{
    pub fn new()->AnimatedBuilder{
        let res = AnimatedBuilder{
            frames:None,
            current:Some(0),
            framespeed:None,
        };
        res
    }
}
