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
    //TO DO : configure position on animated object is much simpler
    let animated = graphics::animated::load(Path::new("/home/kisis/animations")).build();
    let bg = graphics::sprite::load(Path::new("/home/kisis/renrust/bg.bmp"));
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
