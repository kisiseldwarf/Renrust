extern crate renrust;

use renrust::*;
use renrust::graphics::sprite::SpriteBuilder;
use renrust::graphics::animated::AnimatedBuilder;
use renrust::graphics::{Positionable,Sizeable};
use renrust::text_engine::*;
use renrust::text_engine::text::*;
use std::path::*;

const WIDTH : u32 = 1920;
const HEIGHT : u32 = 1080;
const FULLSCREEN : bool = true;

fn update(core: &mut renrust::core::Core){
    //Do things
}

fn init(core: &mut renrust::core::Core){
    /* declaration & load */
    let vp = core.canvas.viewport();
    let mut animated = renrust::graphics::animated::load(Path::new("/home/kisis/animations"));
    let mut frame = renrust::graphics::sprite::SpriteBuilder::new(Path::new("/home/kisis/renrust/bg.bmp"));
    let mut font = renrust::text_engine::text::FontBuilder::new(Path::new("/home/kisis/renrust/font.TTF"));
    font.create_sentence("Yo");

    /* style */
    animated.resize(0.8);
    let animated = animated.framespeed(10);

    /* diplay */
    renrust::scene(core,&frame);
    renrust::show(core,&animated,2);
}

fn main() {
    let cb = renrust::core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN).update(update).init(init);
    let core = renrust::renderer::renderer_init(cb);
    renrust::start(core);
}
