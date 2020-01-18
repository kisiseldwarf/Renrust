use crate::*;
use sdl2::*;
use std::*;

pub struct Core{
    pub width:u32,
    pub height:u32,
    pub fullscreen:bool,
    pub canvas:render::Canvas<video::Window>,
    pub layers:display::Layers,
}

pub struct CoreBuilder{
    pub width:Option<u32>,
    pub height:Option<u32>,
    pub fullscreen:Option<bool>,
    pub canvas:Option<render::Canvas<video::Window>>,
    pub layers:Option<display::Layers>,
}

impl CoreBuilder{
    pub fn fullscreen(mut self:CoreBuilder) -> CoreBuilder{
        self.fullscreen = Some(true);
        self
    }

    pub fn width(mut self:CoreBuilder, w:u32) -> CoreBuilder{
        self.width = Some(w);
        println!("width : {}",self.width.unwrap());
        self
    }

    pub fn height(mut self:CoreBuilder, h: u32) -> CoreBuilder{
        self.height = Some(h);
        self
    }

    pub fn canvas(mut self:CoreBuilder, c:render::Canvas<video::Window>) -> CoreBuilder{
        self.canvas = Some(c);
        self
    }

    pub fn layers(mut self:CoreBuilder, l:display::Layers) -> CoreBuilder{
        self.layers = Some(l);
        self
    }

    pub fn build(self:CoreBuilder) -> Core{
        let mut width = 800;
        let mut height = 600;
        let mut fullscreen = true;
        let mut canvas;
        let mut layers;

        if self.width.is_some(){
            width = self.width.unwrap();
        }
        if self.height.is_some(){
            height = self.height.unwrap();
        }
        if self.fullscreen.is_some(){
            fullscreen = self.fullscreen.unwrap();
        }
        match self.canvas{
            None => panic!("Core must have a Canvas"),
            _=> canvas = self.canvas.unwrap(),
        }
        match self.layers{
            None => panic!("Core must have layers"),
            _=> layers = self.layers.unwrap(),
        }

        let res = Core{
            width,
            height,
            fullscreen,
            canvas,
            layers,
        };
        res
    }
}

pub fn core_builder() -> CoreBuilder{
    let res = CoreBuilder{
        width: None,
        height: None,
        fullscreen: None,
        canvas: None,
        layers: None,
    };
    res
}
