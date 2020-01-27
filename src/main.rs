extern crate renrust;

use renrust::*;
use renrust::graphics::sprite::SpriteBuilder;
use renrust::graphics::animated::AnimatedBuilder;
use std::path::*;

const WIDTH : u32 = 800;
const HEIGHT : u32 = 600;
const FULLSCREEN : bool = false;

fn update(core: &mut renrust::core::Core){
    //Do things
}

fn init(core: &mut renrust::core::Core){
    let animated = renrust::graphics::animated::load(Path::new("/home/kisis/animations"));
    renrust::show(core,&animated,2);
}

fn main() {
    let cb = renrust::core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN).update(update).init(init);
    let core = renrust::renderer::renderer_init(cb);
    renrust::start(core);
}
