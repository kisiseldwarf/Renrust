mod character;
mod display;
mod core;
mod graphics;

extern crate sdl2;
use std::option::*;
use graphics::Positionable;
use graphics::Sizeable;
use std::path::Path;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = false;

//Fonction de lancement de l'application
fn run(builder:core::CoreBuilder){
    display::start(builder);
}

//Appellé avant l'entrée en boucle engloabante par ::display
pub fn init(mut core: &mut core::Core){
    let animated = graphics::animated::load(Path::new("/home/kisis/animations")).framespeed(40).center(core.canvas.viewport()).build().resize(0.5);
    display::show(&mut core,&animated,2);
}

//Appellé toutes les frames par ::display
pub fn update(mut core:&mut core::Core){
}

fn main() {
    let cb = core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN);
    run(cb);
}
