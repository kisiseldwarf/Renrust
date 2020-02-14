use std::path::{Path,PathBuf};
use rusttype::{Font,Scale,VMetrics,HMetrics,point};
use std::fs::*;
use sdl2::*;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use rgb::*;
use crate::graphics::{Drawable,DrawableBuilder,Sizeable};

/* Const(s) */

const DEFAULT_HEIGHT : f32 = 40.0;
const DEFAULT_WIDTH : f32 = 20.0;
const DEFAULT_SIZE : u32 = 40;
const DEFAULT_QUALITY : f32 = 70.0;
const DEFAULT_COLOR : RGBA<u8> = BLACK;
const DEFAULT_POS : (i32,i32) = (0,0);
const DEFAULT_FONT_POS : (f32,f32) = (20.0,20.0);
const DEFAULT_TEXT : &str = "";
const DEFAULT_SPACE : u32 = 0;
const DEFAULT_SPACE_SIZE : u32 = 20;
const RGBA_BPP : u32 = 4;
const BLACK : RGBA<u8> = RGBA::<u8>{r:0,g:0,b:0,a:255,};
const FUSCIA : RGBA<u8> = RGBA::<u8>{r:238,g:130,b:238,a:255,};
const VOID : RGBA<u8> = RGBA::<u8>{r:0,g:0,b:0,a:0,};

/* Struct(s) Data(s) */

#[derive(Debug)]
pub struct TextBuilder{
    font: Font<'static>,
    text: String,
    size: u32,
    color: RGBA<u8>,
    quality: Scale,
    vmetric: VMetrics,
    spacing: u32,
    pos: (i32,i32),
}

#[derive(Debug)]
struct Letter{
    width: u32,
    height: u32,
    data: Box<Vec<u8>>,
    hmetric: HMetrics,
    bounding_box: rusttype::Rect<i32>,
}

#[derive(Debug)]
pub struct Text{
    letters: Vec<Letter>,
    path: PathBuf,
    pos: (i32,i32),
    size: u32,
    vmetric: VMetrics,
    spacing: u32,
}

pub struct Style{
    font: Font<'static>,
    size: u32,
    color: RGBA<u8>,
    quality: Scale,
    spacing: u32,
    pos: (i32,i32),
}

/* Struct(s) Implementation(s) */

impl Style{
    pub fn new(font: &Path)->Style{
        let font = Font::from_bytes(read(font).unwrap()).unwrap();
        let quality = Scale::uniform(DEFAULT_QUALITY);
        Style{
            font,
            color: DEFAULT_COLOR,
            size: DEFAULT_SIZE,
            pos: DEFAULT_POS,
            spacing: DEFAULT_SPACE,
            quality,
        }
    }
    pub fn color(&mut self, col: RGBA<u8>){
        self.color = col;
    }
    pub fn quality(&mut self, quality: f32){
        let quality = Scale::uniform(quality);
        self.quality = quality;
    }
    pub fn spacing(&mut self, spacing: u32){
        self.spacing = spacing;
    }
    pub fn pos(&mut self, pos: (i32,i32)){
        self.pos = pos;
    }
    pub fn size(&mut self, size: u32){
        self.size = size;
    }
}

impl TextBuilder{
    pub fn new(font: &Path)->TextBuilder{
        let font = Font::from_bytes(read(font).unwrap()).unwrap();
        let quality = Scale::uniform(DEFAULT_QUALITY);
        let vmetric = font.v_metrics(quality);
        TextBuilder{
            font,
            text: DEFAULT_TEXT.to_string(),
            color: DEFAULT_COLOR,
            size: DEFAULT_SIZE,
            pos: DEFAULT_POS,
            spacing: DEFAULT_SPACE,
            quality,
            vmetric,
        }
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
    pub fn pos(&mut self, pos: (i32,i32)){
        self.pos = pos;
    }
    pub fn size(&mut self, size: u32){
        self.size = size;
    }
    pub fn quality(&mut self, quality: f32){
        let quality = Scale::uniform(quality);
        let vmetric = self.font.v_metrics(quality);
        self.quality = quality;
        self.vmetric = vmetric;
    }
    pub fn spacing(&mut self, spacing: u32){
        self.spacing = spacing;
    }
}

impl DrawableBuilder for TextBuilder{
    fn build(&self)->Box<dyn Drawable>{
        let mut letters = vec![];
        //point is placing the baseline
        let glyphs : Vec<_> = self.font.layout(self.text.as_str(), self.quality, point(DEFAULT_FONT_POS.0,DEFAULT_FONT_POS.1 + self.vmetric.ascent)).collect();
        let glyphs_height = (self.vmetric.ascent - self.vmetric.descent).ceil() as u32;
        for glyph in glyphs{
            let glyph_width = {
                match glyph.pixel_bounding_box(){
                    Some(i) => {
                        let min_x = glyph.pixel_bounding_box().unwrap().min.x;
                        let max_x = glyph.pixel_bounding_box().unwrap().max.x;
                        (max_x - min_x) as u32
                    }
                    None => { DEFAULT_SPACE_SIZE }
                }
            };
            let pixel_bounding_box;
            let glyph_height = {
                match glyph.pixel_bounding_box(){
                    Some(i) => {
                        let min_y = glyph.pixel_bounding_box().unwrap().min.y;
                        let max_y = glyph.pixel_bounding_box().unwrap().max.y;
                        pixel_bounding_box = glyph.pixel_bounding_box().unwrap();
                        (max_y - min_y) as u32
                    }
                    None => {
                        pixel_bounding_box = rusttype::Rect::<i32>{min: rusttype::Point{x:0,y:0,},max: rusttype::Point{x:1,y:1}};
                        DEFAULT_SPACE_SIZE
                    }
                }
            };
            let mut letter_data = vec![VOID; (glyphs_height * glyph_width) as usize];
            glyph.draw(|x,y,v| {
                letter_data[(x + y * glyph_width) as usize] = RGBA::<u8>{r:self.color.r,g:self.color.g,b:self.color.b,a:(v * self.color.a as f32) as u8,};
            });
            let letter_data = letter_data.as_bytes().to_vec();
            let letter = Letter{
                width: glyph_width,
                height : glyph_height,
                data: Box::new(letter_data),
                bounding_box: pixel_bounding_box,
                hmetric: glyph.unpositioned().h_metrics(),
            };
            letters.push(letter);
        }
        let path = Path::new("/").to_path_buf();
        Box::new(Text{
            letters,
            path,
            pos: self.pos,
            size: self.size,
            vmetric: self.vmetric,
            spacing: self.spacing,
        })
    }
}

impl Drawable for Text{
    fn draw(&mut self, canvas: &mut render::Canvas<video::Window>, delta: u128){
        let mut i = 0;
        let text_cr = canvas.texture_creator();
        let mut offset : u32 = 0;
        for letter in self.letters.iter_mut(){
            let width = letter.width;
            let height  = letter.height;
            let pos = sdl2::rect::Rect::new((self.pos.0 + offset as i32), self.pos.1 + letter.bounding_box.min.y - (self.vmetric.ascent + self.vmetric.descent) as i32, width * self.size, height * self.size);
            let surf = Surface::from_data(
                &mut letter.data,
                width,
                height,
                width * RGBA_BPP,
                PixelFormatEnum::ABGR8888
            ).unwrap();
            let text = text_cr.create_texture_from_surface(surf).unwrap();
            canvas.copy(&text,None,pos);
            i += 1;
            offset += width * self.size + self.spacing;
        }
    }
    fn get_path(&self)->&Path{
        &self.path
    }
}

impl Sizeable for Text{
    fn resize(&mut self,percentage: f32){}
    fn width(&mut self,w:u32){}
    fn height(&mut self,h:u32){}
    fn get_width(&self)->u32{4}
    fn get_height(&self)->u32{4}
}
