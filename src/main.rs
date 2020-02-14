extern crate renrust;
extern crate lipsum;

use renrust::*;
use renrust::graphics::sprite::SpriteBuilder;
use renrust::graphics::animated::AnimatedBuilder;
use renrust::graphics::{Positionable,Sizeable};
use rgb::*;
use lipsum::lipsum;
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
    let mut dialogbox = renrust::graphics::sprite::SpriteBuilder::new(Path::new("/home/kisis/renrust/dialogbox.bmp"));
    let mut textbuilder = renrust::text_engine::text::TextBuilder::new(Path::new("/home/kisis/renrust/optimus/OptimusPrincepsSemiBold.ttf"));

    /* style */
    animated.center(vp);
    let animated = animated.framespeed(20);
    dialogbox.resize(2.0);
    dialogbox.downcenter(vp);
    let rect = sdl2::rect::Rect::new(60,450,630,400);
    textbuilder.display(rect);
    textbuilder.scale(26.0);
    textbuilder.text(&lipsum(25));
    textbuilder.color(RGBA::<u8>::new(0,0,0,255));

    /* display */
    renrust::show(core, &animated, 1);
    renrust::show(core, &dialogbox, 2);
    renrust::show(core, &textbuilder, 3);
}

fn main() {
    let cb = renrust::core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN).update(update).init(init);
    let core = renrust::renderer::renderer_init(cb);
    renrust::start(core);
}
