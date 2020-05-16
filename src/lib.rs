mod character;
pub mod renderer;
pub mod core;
pub mod graphics;
pub mod text;

extern crate sdl2;
extern crate rusttype;
extern crate rgb;

use std::option::*;
use sdl2::event::*;
use sdl2::keyboard::*;
use crate::graphics::*;
use crate::core::*;
use crate::text::*;
use std::time::{Duration, Instant};

const SCENE_LAYER : usize = 0;

/* Main Statements */

pub fn scene<T: DrawableBuilder>(core: &mut Core, img: &T){
    let mut img = img.build();
    img.width(core.canvas.viewport().width());
    img.height(core.canvas.viewport().height());
    core.layers.layers[SCENE_LAYER].push(img);
}

//Note : POUR METTRE UNE IMAGE EN PLEIN ECRAN, METTRE SON WIDTH & SON HEIGHT A LA TAILLE DU VIEWPORT
pub fn show<T: DrawableBuilder>(core: &mut Core, image: &T, index: usize){
    if index == SCENE_LAYER {
        return;
    }
    let image = image.build();
    core.layers.layers[index].push(image);
}

// pub fn say(text: &str){
//     let TextBuilder
// }

/* Launch Function */

pub fn start(mut core: Core){
    (core.init_func)(&mut core);
    mainloop(core);
}

////* Main Game Loop *////

fn mainloop(mut core: Core){
    //Affichage du canvas
    core.canvas.present();
    let mut timer;
    //Boucle englobante
    'mainloop: loop {
        //Timer
        timer = Instant::now();

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

        // -- Boucle principale --

        core.canvas.clear(); //On efface tout
        (core.update_func)(&mut core); //On appelle la fonction d'update logique du jeu

        //On dessine tous les calques
        for lay in core.layers.layers.iter_mut(){
            for img in lay.iter_mut(){
                img.as_mut().draw(&mut core.canvas, core.elapsed);
            }
        }

        //on affiche le canvas
        core.canvas.present();

        //timer
        core.elapsed = timer.elapsed().as_millis();
        timer = Instant::now();
    }
}
