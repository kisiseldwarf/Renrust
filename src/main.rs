mod character;
mod display;
mod core;
mod graphics;

extern crate sdl2;
use std::option::*;
use graphics::sprite::*;
use display::*;
use std::path::*;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = false;

//Fonction de lancement de l'application
fn run(builder:core::CoreBuilder){
    display::start(builder);
}

//Appellé avant l'entrée en boucle engloabante par ::display
pub fn init(core: &mut core::Core){
    let frame = SpriteBuilder::new(Path::new("/home/kisis/animations/objection-0043.bmp"));
    show(core,&frame,2);
}

//Appellé toutes les frames par ::display
pub fn update(core:&mut core::Core){
}

fn main() {
    let cb = core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN);
    run(cb);
}
