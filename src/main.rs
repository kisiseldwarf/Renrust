mod character;
mod display;
mod core;

extern crate sdl2;
use std::path::*;
use std::option::*;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = true;

//Fonction de lancement de l'application
fn run(builder:core::CoreBuilder){
    display::start(builder);
}

//Appellé toutes les frames par ::display
pub fn update(mut core:&mut core::Core){
    display::show(&mut core,Path::new("/home/kisis/test.bmp"),1);
    display::show(&mut core,Path::new("/home/kisis/0-1.bmp"),1);
    display::show(&mut core,Path::new("/home/kisis/0-0.bmp"),1);
    println!("{:#?}",core.layers.layers);
}

fn main() {
    let mut cb = core::core_builder().width(WIDTH).height(HEIGHT);
    if FULLSCREEN {
        cb = cb.fullscreen();
    }
    run(cb);
}
