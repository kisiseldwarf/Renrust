mod character;
mod display;
mod core;
mod graphics;

extern crate sdl2;
use std::path::*;
use std::option::*;
use graphics::Positionable;
use graphics::Sizeable;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = true;

//Fonction de lancement de l'application
fn run(builder:core::CoreBuilder){
    display::start(builder);
}

//Appellé avant l'entrée en boucle engloabante par ::display
pub fn init(mut core: &mut core::Core){
    let image = &graphics::Image::new().path(Path::new("/home/kisis/0-0.bmp").to_path_buf()).resize(3).center(core.canvas.viewport());
    // display::show(&mut core,&graphics::Image::new().path(Path::new("/home/kisis/test.bmp").to_path_buf()).pos(50,50),1);
    // display::show(&mut core,&graphics::Image::new().path(Path::new("/home/kisis/0-1.bmp").to_path_buf()).pos(500,500),4);
    display::show(&mut core,image,1);
}

//Appellé toutes les frames par ::display
pub fn update(mut core:&mut core::Core){
}

fn main() {
    let mut cb = core::core_builder().width(WIDTH).height(HEIGHT);
    if FULLSCREEN {
        cb = cb.fullscreen();
    }
    run(cb);
}
