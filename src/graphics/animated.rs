use std::*;
use super::*;

const DEFAULT_FPS : u32 = 5;

#[derive(Clone)]
pub struct Animated{
    frames:Vec<sprite::Sprite>,
    current:usize,
    fps:u32,
}

pub struct AnimatedBuilder{
    frames:Option<Vec<sprite::Sprite>>,
    current:Option<usize>,
    fps:Option<u32>,
}

impl AnimatedBuilder{
    pub fn new() -> AnimatedBuilder{
        let res = AnimatedBuilder{
            frames:None,
            current:Some(0),
            fps:None,
        };
        res
    }
    pub fn frames(mut self,frames:Vec<sprite::Sprite>)->AnimatedBuilder{
        self.frames = Some(frames);
        self
    }
    pub fn fps(mut self, fps:u32)->AnimatedBuilder{
        self.fps = Some(fps);
        self
    }
    pub fn build(mut self)->Animated{
        let mut frames;
        let mut fps = DEFAULT_FPS;

        frames = self.frames.expect("An Animated must have frames");
        if self.fps.is_some(){
            fps = self.fps.unwrap();
        }
        let res = Animated{
            frames,
            current:0,
            fps,
        };
        res
    }
}

impl Drawable for Animated{
    fn draw(self:&mut Animated, canvas: &mut render::Canvas<video::Window>){
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
            fps:None,
        };
        res
    }
}
