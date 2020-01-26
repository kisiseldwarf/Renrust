mod character;
pub mod renderer;
pub mod core;
pub mod graphics;
extern crate sdl2;
use std::option::*;
use sdl2::event::*;
use sdl2::keyboard::*;
use crate::graphics::*;
use crate::core::*;

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
    (core.init_func)(&mut core);
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
        (core.update_func)(&mut core); //On appelle la fonction d'update logique du jeu

        //On dessine tous les calques
        for lay in core.layers.layers.iter_mut(){
            for img in lay.iter_mut(){
                img.as_mut().draw(&mut core.canvas);
            }
        }
        core.canvas.present();
    }
}
