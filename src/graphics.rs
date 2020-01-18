use std::*;
use std::path::PathBuf;


//Only 5 layers are available. But for a game, it should be more than enough
const LAYERS_NB : usize = 5;

//layers are just a fixed collection (array) of an unknown number of Images
pub struct Layers{
    pub layers: [Vec<Image>;LAYERS_NB],
}

//Those are options types to have a none value, since not all images must implements every attributes
pub struct Image{
    pub path:Option<PathBuf>,
    pub width:Option<u32>,
    pub height:Option<u32>,
    pub pos:Option<(u32,u32)>,
}

impl Image{
    pub fn new() -> Image{
        let res = Image{
            path:None,
            width:None,
            height:None,
            pos:None,
        };
        res
    }
    pub fn path(mut self:Image,str:PathBuf)->Image{
        self.path = Some(str);
        self
    }
    pub fn width(mut self:Image,w:u32)->Image{
        self.width = Some(w);
        self
    }
    pub fn height(mut self:Image,h:u32)->Image{
        self.height = Some(h);
        self
    }
    pub fn pos(mut self:Image,x:u32,y:u32)->Image{
        self.pos = Some((x,y));
        self
    }
}
