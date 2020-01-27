extern crate renrust;

use renrust::*;
use renrust::graphics::sprite::SpriteBuilder;
use std::path::*;

const WIDTH : u32 = 800;
const HEIGHT : u32 = 600;
const FULLSCREEN : bool = false;

fn update(core: &mut renrust::core::Core){
    //Do things
}

fn init(core: &mut renrust::core::Core){
    let frame = SpriteBuilder::new(Path::new("/home/kisis/animations/objection-0043.bmp")).pos((100,200));
    let frame2 = SpriteBuilder::new(Path::new("/home/kisis/renrust/bg.bmp"));
    renrust::show(core,&frame,2);
    let frame = frame.pos((54,202));
    renrust::show(core,&frame,2);
    let frame = frame.pos((-10,-10));
    renrust::show(core,&frame,2);
    renrust::scene(core,&frame2);
}

fn main() {
    let cb = renrust::core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN).update(update).init(init);
    let core = renrust::renderer::renderer_init(cb);
    renrust::start(core);
}
