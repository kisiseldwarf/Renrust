mod character;
mod display;
mod core;
mod graphics;

extern crate sdl2;
use std::path::*;
use std::option::*;
use graphics::Positionable;
use graphics::Sizeable;
use std::path::Path;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = true;

//Fonction de lancement de l'application
fn run(builder:core::CoreBuilder){
    display::start(builder);
}

//Appellé avant l'entrée en boucle engloabante par ::display
pub fn init(mut core: &mut core::Core){
    //configure position on animated object is much simpler
    let frame1 = graphics::load(Path::new("/home/kisis/animations/objection-0043.bmp"));
    let frame2 = graphics::load(Path::new("/home/kisis/animations/objection-0044.bmp"));
    let frame3 = graphics::load(Path::new("/home/kisis/animations/objection-0045.bmp"));
    let frame4 = graphics::load(Path::new("/home/kisis/animations/objection-0046.bmp"));
    let frame5 = graphics::load(Path::new("/home/kisis/animations/objection-0047.bmp"));
    let bg = graphics::load(Path::new("/home/kisis/renrust/bg.bmp"));
    let animated = graphics::animated::Animated::new().frames(vec![frame1,frame2,frame3,frame4,frame5]).build();
    display::scene(&mut core,&bg);
    display::show(&mut core,&animated,1);
}

//Appellé toutes les frames par ::display
pub fn update(mut core:&mut core::Core){
}

fn main() {
    let mut cb = core::core_builder().width(WIDTH).height(HEIGHT);
    run(cb);
}
