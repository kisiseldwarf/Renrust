use sdl2::pixels::*;
use crate::*;
use crate::graphics::*;
use crate::core::*;
use sdl2::video::*;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = false;

pub fn build_window(video_subsystem:&sdl2::VideoSubsystem,title:&str,width:u32,height:u32,fullscreen:bool) -> sdl2::video::Window {
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

pub fn renderer_init(builder: CoreBuilder) -> Core {
    //Initialisation de la SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    //Création de la fenêtre
    let mut window : Option<Window> = None;
    if builder.width.is_some() && builder.height.is_some() && builder.fullscreen.is_some(){
        window = Some(build_window(&video_subsystem,"renrust",builder.width.unwrap(),builder.height.unwrap(),builder.fullscreen.unwrap()));
    } else {
        window = Some(build_window(&video_subsystem,"renrust",WIDTH,HEIGHT,FULLSCREEN));
    }
    let window = window.expect("Failed to create window");

    //Création du canvas a partir de la fenêtre
    let canvas = build_canvas(window, Color::RGB(255,255,255));
    let mut frames = Vec::<Vec<Box<dyn Drawable>>>::new();
    for i in 0..5{
        let layer = Vec::<Box<dyn Drawable>>::new();
        frames.push(layer);
    }
    let layers = Layers { layers: frames, };

    //Initialisation Core
    builder.canvas(canvas).layers(layers).event_pump(event_pump).build()
}
