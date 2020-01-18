use sdl2::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::PathBuf;
use std::path::Path;
use crate::*;


// fn scene(img: String){
//
// }

//Ici, on ajoute juste ajouter dans le Store
pub fn show(core:&mut crate::core::Core,path:&Path,layer:usize){
    let image = graphics::Image::new().path(path.to_path_buf());
    core.layers.layers[layer].push(image);
}

pub fn draw_img(canvas: &mut render::Canvas<video::Window>, path: &Path){
    let surface = surface::Surface::load_bmp(path).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(surface).unwrap();
    canvas.copy(&texture,None,None).unwrap();
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
    let window = build_window(&video_subsystem,"renrust",builder.width.unwrap(),builder.height.unwrap(),false);

    //Création du canvas a partir de la fenêtre
    //window n'est plus utilisable après ça
    let canvas = build_canvas(window, Color::RGB(255,255,255));

    //Initialisation Core
    let mut core = builder.canvas(canvas).layers(graphics::Layers{
        layers: [Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new(),Vec::<graphics::Image>::new()],
    }).build();

    //Affichage du canvas
    core.canvas.present();

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
        core.canvas.clear();
        crate::update(&mut core);
        for i in core.layers.layers.iter(){
            for j in i.iter(){
                draw_img(&mut core.canvas,&j.path.as_ref().expect("All Images should have a Path."));
            }
        }
        // draw_layers(&mut core);
        core.canvas.present();
    }
}
