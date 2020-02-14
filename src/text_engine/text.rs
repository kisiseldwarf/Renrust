use std::path::{Path,PathBuf};
use rusttype::{Font,Scale,VMetrics,HMetrics,point};
use std::fs::*;
use sdl2::*;
use sdl2::video::Window;
use sdl2::rect::{Rect,Point};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use rgb::*;
use crate::graphics::{Drawable,DrawableBuilder,Sizeable};

/* Const(s) */

const DEFAULT_HEIGHT : f32 = 40.0;
const DEFAULT_WIDTH : f32 = 20.0;
const DEFAULT_SIZE : f32 = 1.0;
const DEFAULT_SCALE : f32 = 70.0;
const DEFAULT_COLOR : RGBA<u8> = BLACK;
const DEFAULT_POS : (i32,i32) = (0,0);
const DEFAULT_FONT_POS : (f32,f32) = (0.0,0.0);
const DEFAULT_TEXT : &str = "";
const DEFAULT_SPACING : u32 = 0;
const DEFAULT_SPACE_SIZE : u32 = 20;
const NO_BOUNDING_BOX : rusttype::Rect<i32> = rusttype::Rect::<i32>{min: rusttype::Point{x:0,y:0,},max: rusttype::Point{x:1,y:1}};
const DEFAULT_DISPLAY : (i32,i32,u32,u32) = (0,0,300,400);
const RGBA_BPP : u32 = 4;
const BLACK : RGBA<u8> = RGBA::<u8>{r:0,g:0,b:0,a:255,};
const VOID : RGBA<u8> = RGBA::<u8>{r:0,g:0,b:0,a:0,};

/* Struct(s) Data(s) */

#[derive(Debug)]
pub struct TextBuilder{
    font: Font<'static>,
    text: String,
    size: f32,
    color: RGBA<u8>,
    scale: Scale,
    vmetric: VMetrics,
    spacing: u32,
    display: Rect,
}

#[derive(Debug)]
struct Letter{
    width: u32,
    height: u32,
    data: Box<Vec<u8>>,
    hmetric: HMetrics,
    pos: (i32,i32),
    bounding_box: rusttype::Rect<i32>,
    size: f32,
}

#[derive(Debug)]
pub struct Text{
    letters: Vec<Letter>,
    size: f32,
    vmetric: VMetrics,
    spacing: u32,
    display: Rect,
}

/* Struct(s) Implementation(s) */

impl TextBuilder{
    pub fn new(font: &Path)->TextBuilder{
        let font = Font::from_bytes(read(font).unwrap()).unwrap();
        let scale = Scale::uniform(DEFAULT_SCALE);
        let vmetric = font.v_metrics(scale);
        TextBuilder{
            font,
            text: DEFAULT_TEXT.to_string(),
            color: DEFAULT_COLOR,
            size: DEFAULT_SIZE,
            spacing: DEFAULT_SPACING,
            scale,
            vmetric,
            display: {
                Rect::new(DEFAULT_DISPLAY.0,DEFAULT_DISPLAY.1,DEFAULT_DISPLAY.2,DEFAULT_DISPLAY.3)
            },
        }
    }
    pub fn display(&mut self, display: Rect){
        self.display = display;
    }
    pub fn get_color(&self)->RGBA<u8>{
        self.color
    }
    pub fn color(&mut self, col: RGBA<u8>){
        self.color = col;
    }
    pub fn text(&mut self, text: &str){
        self.text = text.to_string();
    }
    pub fn scale(&mut self, scale: f32){
        let scale = Scale::uniform(scale);
        let vmetric = self.font.v_metrics(scale);
        self.scale = scale;
        self.vmetric = vmetric;
    }
    pub fn spacing(&mut self, spacing: u32){
        self.spacing = spacing;
    }
}

impl DrawableBuilder for TextBuilder{
    fn build(&self)->Box<dyn Drawable>{
        let mut letters = vec![];
        /* point is placing the baseline */
        let glyphs : Vec<_> = self.font.layout(self.text.as_str(), self.scale, point(DEFAULT_FONT_POS.0,DEFAULT_FONT_POS.1 + self.vmetric.ascent)).collect();
        let glyphs_height = (self.vmetric.ascent - self.vmetric.descent).ceil() as u32;

        for glyph in glyphs{
            /* Metrics */
            let glyph_height;
            let glyph_width;
            let pixel_bounding_box;
            if glyph.pixel_bounding_box().is_some(){
                pixel_bounding_box = glyph.pixel_bounding_box().unwrap();
                glyph_width = (pixel_bounding_box.max.x - pixel_bounding_box.min.x) as u32;
                glyph_height = (pixel_bounding_box.max.y - pixel_bounding_box.min.y) as u32;
            } else {
                pixel_bounding_box = NO_BOUNDING_BOX;
                glyph_height = DEFAULT_SPACE_SIZE;
                glyph_width = DEFAULT_SPACE_SIZE;
            }

            /* Rasterizing */
            let mut letter_data = vec![VOID; (glyphs_height * glyph_width) as usize];
            glyph.draw(|x,y,v| {
                let pix = RGBA::<u8>{r:self.color.r,g:self.color.g,b:self.color.b,a:(v * self.color.a as f32) as u8,};
                letter_data[(x + y * glyph_width) as usize] = pix;
            });
            let letter_data = letter_data.as_bytes().to_vec();

            /* Creating Letter Struct */
            let letter = Letter{
                width: glyph_width,
                height : glyph_height,
                data: Box::new(letter_data),
                bounding_box: pixel_bounding_box,
                pos: (0,0),
                size: self.size,
                hmetric: glyph.unpositioned().h_metrics(),
            };
            letters.push(letter);
        }

        /* Create Text */
        Box::new(Text{
            letters,
            size: self.size,
            vmetric: self.vmetric,
            spacing: self.spacing,
            display: self.display,
        })
    }
}

impl Drawable for Text{
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>, delta: u128){
        let mut offset : i32 = 0;
        let mut offset_y : f32 = 0.0;
        let display = self.display;

        /* on affiche chaque lettre */
        for letter in self.letters.iter_mut(){
            letter.pos = (self.display.left() + offset as i32, self.display.top() + (letter.bounding_box.min.y as f32 * letter.size) as i32 + offset_y as i32);
            letter.draw(canvas,delta);
            offset += (letter.hmetric.advance_width * letter.size + self.spacing as f32) as i32;
            if self.display.left() + offset > display.right(){
                offset_y += (self.vmetric.ascent - self.vmetric.descent).ceil() + self.vmetric.line_gap;
                offset = 0;
            }
        }
    }

    fn get_path(&self)->&Path{
        Path::new("/")
    }
}

impl Drawable for Letter{
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>, delta: u128){
        let text_cr = canvas.texture_creator();
        /* metrics */
        let width = self.width;
        let height  = self.height;
        let x = self.pos.0 as i32;
        let y = self.pos.1 as i32;
        let size = self.size;
        let pos = sdl2::rect::Rect::new(x, y, (width as f32 * size) as u32, (height as f32 * size) as u32);

        /* creation texture */
        let surf = Surface::from_data(
            &mut self.data,
            width,
            height,
            width * RGBA_BPP,
            PixelFormatEnum::ABGR8888
        ).unwrap();

        /* texture copy */
        let text = text_cr.create_texture_from_surface(surf).unwrap();
        canvas.copy(&text,None,pos);
    }

    fn get_path(&self)->&Path{
        Path::new("/")
    }
}

impl Sizeable for TextBuilder{
    fn resize(&mut self,percentage: f32){
        self.size = percentage;
    }
    fn width(&mut self,w:u32){}
    fn height(&mut self,h:u32){}
    fn get_width(&self)->u32{4}
    fn get_height(&self)->u32{4}
}

impl Sizeable for Letter{
    fn resize(&mut self,percentage: f32){
        self.size = percentage;
    }
    fn width(&mut self,w:u32){}
    fn height(&mut self,h:u32){}
    fn get_width(&self)->u32{4}
    fn get_height(&self)->u32{4}
}

impl Sizeable for Text{
    fn resize(&mut self,percentage: f32){
        for letter in self.letters.iter_mut(){
            letter.size = percentage;
        }
        self.size = percentage;
    }
    fn width(&mut self,w:u32){}
    fn height(&mut self,h:u32){}
    fn get_width(&self)->u32{4}
    fn get_height(&self)->u32{4}
}
