extern crate renrust;

use renrust::*;
use renrust::core::*;
use renrust::graphics::sprite::*;
use std::path::*;

const WIDTH : u32 = 800;
const HEIGHT : u32 = 600;
const FULLSCREEN : bool = false;

fn update(core: &mut Core){
    //Do things
}

fn init(core: &mut Core){
    let frame = SpriteBuilder::new(Path::new("/home/kisis/animations/objection-0043.bmp")).pos((100,200));
    let frame2 = SpriteBuilder::new(Path::new("/home/kisis/renrust/bg.bmp"));
    renrust::show(core,&frame,2);
    let frame = frame.pos((54,202));
    renrust::show(core,&frame,2);
    let frame = frame.pos((54,10));
    renrust::show(core,&frame,2);
    renrust::scene(core,&frame2);
}

fn main() {
    let cb = core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN).update(update).init(init);
    let core = renderer::renderer_init(cb);
    renrust::start(core);
}
