use sdl2::pixels::*;
use sdl2::event::*;
use sdl2::keyboard::*;
use crate::*;
use crate::graphics::*;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = false;

//Ajout d'une scène (image prenant toute la dimension de l'écran)
pub fn scene<T: Drawable + std::clone::Clone + Sizeable + 'static>(core:&mut crate::core::Core, img: &T){
    let this_image = img.clone();
    let this_image = this_image.width(core.canvas.viewport().width());
    let this_image = this_image.height(core.canvas.viewport().height());
    let my_box = std::boxed::Box::new(this_image);
    core.layers.layers[0].push(my_box);
}

//On ajoute juste dans les layers
//Note : POUR METTRE UNE IMAGE EN PLEIN ECRAN, METTRE SON WIDTH & SON HEIGHT A LA TAILLE DU VIEWPORT
pub fn show<T: DrawableBuilder>(core: &mut crate::core::Core,image:T,index:usize){
    if index == 0 {
        return;
    }
    let image = image.build();
    core.layers.layers[index].push(image);
}

// fn say(text:String){
//
// }

/* MAIN FUNCTION */

pub fn start(builder: crate::core::CoreBuilder){

    //Initialisation de la SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Création de la fenêtre
    let window = build_window(&video_subsystem,"renrust",WIDTH,HEIGHT,FULLSCREEN);
    if builder.width.is_some() && builder.height.is_some() && builder.fullscreen.is_some(){
        let window = build_window(&video_subsystem,"renrust",builder.width.unwrap(),builder.height.unwrap(),builder.fullscreen.unwrap());
    }

    //Création du canvas a partir de la fenêtre
    //window n'est plus utilisable après ça
    let canvas = build_canvas(window, Color::RGB(255,255,255));

    //Initialisation Core
    let mut core = builder.canvas(canvas).layers(graphics::Layers{
        layers: vec![Vec::<std::boxed::Box<dyn Drawable>>::new(),Vec::<std::boxed::Box<dyn Drawable>>::new(),Vec::<std::boxed::Box<dyn Drawable>>::new(),Vec::<std::boxed::Box<dyn Drawable>>::new(),Vec::<std::boxed::Box<dyn Drawable>>::new()],
    }).build();

    //Affichage du canvas
    core.canvas.present();

    //Appel de init()
    crate::init(&mut core);

    //Boucle englobante
    'mainloop: loop {
        //Boucle évenementielle
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
        crate::update(&mut core); //On appelle la fonction d'update logique du jeu

        //On dessine tous les calques
        for lay in core.layers.layers.iter_mut(){
            for img in lay.iter_mut(){
                img.as_mut().draw(&mut core.canvas);
            }
        }
        core.canvas.present();
    }
}

/* SDL2 Administration */
fn build_window(video_subsystem:&sdl2::VideoSubsystem,title:&str,width:u32,height:u32,fullscreen:bool) -> sdl2::video::Window {
    let mut window = video_subsystem.window(title, width, height);
    if fullscreen
    { window.fullscreen(); }
    let window = window.build()
    .unwrap();
    window
}

fn build_canvas(window:sdl2::video::Window,background:Color) -> sdl2::render::Canvas<sdl2::video::Window>{
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(background);
    canvas.clear();
    canvas
}
