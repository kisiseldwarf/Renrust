use sdl2::*;
use sdl2::pixels::Color;
use sdl2::pixels::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use crate::*;
use crate::graphics::Drawable;
use image::*;

// fn scene(img: String){
//
// }

//Ici, on ajoute juste dans les layers
//Show crée une nouvelle Image à chaque appel, même sur le même chemin
//POUR METTRE UNE IMAGE EN PLEIN ECRAN, METTRE SON WIDTH & SON HEIGHT A LA TAILLE DU VIEWPORT
pub fn show(core:&mut crate::core::Core,image:&graphics::Image,layer:usize){
    let this_image = image.clone();
    core.layers.layers[layer].push(this_image);
}

//Dessine une image en fonction d'un Path
fn draw_img(canvas: &mut render::Canvas<video::Window>, img: &graphics::Image){
    // let sp = image::open(img.get_path()).unwrap();
    // let mut image = sp.raw_pixels();
    // let surface = surface::Surface::from_data(&mut image,sp.width(),sp.height(),600,pixels::PixelFormatEnum::ARGB8888).unwrap();
    let surface = surface::Surface::load_bmp(img.get_path()).unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut width = surface.width();
    let mut height = surface.height();
    if img.width.is_some()
        { width = img.width.unwrap(); }
    if img.height.is_some()
        { height = img.height.unwrap(); }
    if img.pos.is_some(){
         x = img.pos.unwrap().0 as i32;
         y = img.pos.unwrap().1 as i32;
     }
    let rect = rect::Rect::new(
        x,
        y,
        width,
        height,
    );
    let old = canvas.viewport();
    canvas.set_viewport(rect);
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(surface).unwrap();
    canvas.copy(&texture,None,None).unwrap();
    canvas.set_viewport(old);
}

// fn say(text:String){
//
// }

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

pub fn start(builder: crate::core::CoreBuilder){

    //Initialisation de la SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Création de la fenêtre
    let window = build_window(&video_subsystem,"renrust",builder.width.unwrap(),builder.height.unwrap(),builder.fullscreen.unwrap());

    //Création du canvas a partir de la fenêtre
    //window n'est plus utilisable après ça
    let canvas = build_canvas(window, Color::RGB(255,255,255));

    //Initialisation Core
    let mut core = builder.canvas(canvas).layers(graphics::Layers{
        layers: [Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new()],
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
        for lay in &mut core.layers.layers.iter(){ //On dessine tous les calques
            for img in &mut lay.iter(){
                // img.draw(&mut core.canvas);
                draw_img(&mut core.canvas,img);
            }
        }
        core.canvas.present();
    }
}
