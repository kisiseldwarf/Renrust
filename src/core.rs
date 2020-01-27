use crate::*;
use sdl2::*;
use std::*;

const DEFAULT_WIDTH : u32 = 200;
const DEFAULT_HEIGHT : u32 = 200;
const DEFAULT_FULLSCREEN : bool = false;
const ELAPSED_STARTER : u128 = 0;

pub struct Core{
    pub width:u32,
    pub height:u32,
    pub fullscreen:bool,
    pub canvas:render::Canvas<video::Window>,
    pub layers:graphics::Layers,
    pub event_pump: EventPump,
    pub update_func:fn(&mut Core),
    pub init_func:fn(&mut Core),
    pub elapsed: u128,
}

pub struct CoreBuilder{
    pub width:Option<u32>,
    pub height:Option<u32>,
    pub fullscreen:Option<bool>,
    pub canvas:Option<render::Canvas<video::Window>>,
    pub layers:Option<graphics::Layers>,
    pub event_pump: Option<EventPump>,
    pub update_func: Option<fn(&mut Core)>,
    pub init_func: Option<fn(&mut Core)>,
}

impl CoreBuilder{
    pub fn fullscreen(mut self:CoreBuilder, is_it:bool) -> CoreBuilder{
        self.fullscreen = Some(is_it);
        self
    }

    pub fn update(mut self, func: fn(&mut Core)) -> CoreBuilder{
        self.update_func = Some(func);
        self
    }

    pub fn init(mut self, func: fn(&mut Core)) -> CoreBuilder{
        self.init_func = Some(func);
        self
    }

    pub fn event_pump(mut self, event_pump: EventPump) -> CoreBuilder{
        self.event_pump = Some(event_pump);
        self
    }

    pub fn width(mut self:CoreBuilder, w:u32) -> CoreBuilder{
        self.width = Some(w);
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

    pub fn layers(mut self:CoreBuilder, l:graphics::Layers) -> CoreBuilder{
        self.layers = Some(l);
        self
    }

    pub fn build(self:CoreBuilder) -> Core{
        let mut width = DEFAULT_WIDTH;
        let mut height = DEFAULT_HEIGHT;
        let mut fullscreen = DEFAULT_FULLSCREEN;
        let elapsed = ELAPSED_STARTER;
        let canvas;
        let layers;
        let event_pump;
        let update_func;
        let init_func;

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
        match self.event_pump{
            None => panic!("Core must have an event_pump"),
            _=> event_pump = self.event_pump.unwrap(),
        }
        match self.update_func{
            None => panic!("No update function was given. Give one to core with this prototype : (&mut Core) => ()"),
            _=> update_func = self.update_func.unwrap(),
        }
        match self.init_func{
            None => panic!("No init function was given. Give one to core with this prototype : (&mut Core) => ()"),
            _=> init_func = self.init_func.unwrap(),
        }
        let res = Core{
            width,
            height,
            fullscreen,
            canvas,
            layers,
            event_pump,
            update_func,
            init_func,
            elapsed,
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
        event_pump: None,
        update_func: None,
        init_func: None,
    };
    res
}
