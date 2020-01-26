mod character;
pub mod renderer;
pub mod core;
mod graphics;

extern crate sdl2;
use std::option::*;
use sdl2::pixels::*;
use sdl2::event::*;
use sdl2::keyboard::*;
use graphics::sprite::*;
use renderer::*;
use std::path::*;
use crate::graphics::*;
use crate::core::*;
use graphics::Sizeable;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = false;

////* Runtime Functions *////
pub fn init(core: &mut Core){
    let frame = SpriteBuilder::new(Path::new("/home/kisis/animations/objection-0043.bmp")).pos((100,200));
    let frame2 = SpriteBuilder::new(Path::new("/home/kisis/renrust/bg.bmp"));
    show(core,&frame,2);
    let frame = frame.pos((54,202));
    show(core,&frame,2);
    let frame = frame.pos((54,10));
    show(core,&frame,2);
    scene(core,&frame2);
}

pub fn update(core: &mut Core){
}

////* Main Statements *////
pub fn scene<T: DrawableBuilder + Clone + Sizeable>(core: &mut Core, img: &T){
    let img = img.clone();
    let img = img.width(core.canvas.viewport().width());
    let img = img.height(core.canvas.viewport().height());
    let img = img.build();
    core.layers.layers[0].push(img);
}

//Note : POUR METTRE UNE IMAGE EN PLEIN ECRAN, METTRE SON WIDTH & SON HEIGHT A LA TAILLE DU VIEWPORT
pub fn show<T: DrawableBuilder>(core: &mut Core, image: &T, index: usize){
    if index == 0 {
        return;
    }
    let image = image.build();
    core.layers.layers[index].push(image);
}

// fn say(text:String){
// TO DO
// }

////* Launch Function *////
pub fn start(mut core: Core){
    init(&mut core);
    mainloop(core);
}

////* Main Game Loop *////
fn mainloop(mut core: Core){
    //Affichage du canvas
    core.canvas.present();

    //Boucle englobante
    'mainloop: loop {
        //Boucle évenementielle
        let event_pump = &mut core.event_pump;
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} |
                Event::Quit {..} => break 'mainloop,
                _ => {
                }
            }
        }

        //Boucle principale
        core.canvas.clear(); //On efface tout
        update(&mut core); //On appelle la fonction d'update logique du jeu

        //On dessine tous les calques
        for lay in core.layers.layers.iter_mut(){
            for img in lay.iter_mut(){
                img.as_mut().draw(&mut core.canvas);
            }
        }
        core.canvas.present();
    }
}
