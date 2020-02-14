extern crate renrust;

use renrust::*;
use renrust::graphics::sprite::SpriteBuilder;
use renrust::graphics::animated::AnimatedBuilder;
use renrust::graphics::{Positionable,Sizeable};
use rgb::*;
// use renrust::text_engine::*;
// use renrust::text_engine::text::*;
use std::path::*;

const WIDTH : u32 = 800;
const HEIGHT : u32 = 600;
const FULLSCREEN : bool = false;

fn update(core: &mut renrust::core::Core){
    //Do things
}

fn init(core: &mut renrust::core::Core){
    /* declaration & load */
    let vp = core.canvas.viewport();
    let mut animated = renrust::graphics::animated::load(Path::new("/home/kisis/animations"));
    let mut frame = renrust::graphics::sprite::SpriteBuilder::new(Path::new("/home/kisis/renrust/bg.bmp"));
    let mut textbuilder = renrust::text_engine::text::TextBuilder::new(Path::new("/home/kisis/renrust/Wintersoul.ttf"));

    /* style */
    animated.center(vp);
    let animated = animated.framespeed(20);
    textbuilder.text("Je t'aime Pierre");
    textbuilder.size(1);
    textbuilder.spacing(3);
    textbuilder.pos((30,20));
    textbuilder.color(RGBA::<u8>::new(162,210,223,255));

    /* display */
    renrust::show(core, &textbuilder, 2);
}

fn main() {
    let cb = renrust::core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN).update(update).init(init);
    let core = renrust::renderer::renderer_init(cb);
    renrust::start(core);
}
