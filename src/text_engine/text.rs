//En a-ton vraiment besoin ?
// Oui

use std::path::Path;
use sdl2::pixels::Color;
use rusttype::Font;
use rusttype::Point;
use rusttype::Scale;
use std::string::ToString;
use std::fs::*;

const DEFAULT_HEIGHT : f32 = 16.0;
const DEFAULT_WIDTH : f32 = 16.0;
const DEFAULT_COLOR : Color = FUSCIA;
const FUSCIA : Color = Color{ r: 238, g: 130, b: 238, a: 0};
const DEFAULT_POS : (f32,f32) = (0.0,0.0);

/* Struct(s) Data(s) */

#[derive(Debug)]
pub struct FontBuilder{
    font: Font<'static>,
    height: f32,
    width: f32,
    color: Color,
}

/* Struct(s) Implementation(s) */

impl FontBuilder{
    pub fn new(src: &Path)->FontBuilder{
        let font = Font::from_bytes(read(src).unwrap()).unwrap();
        FontBuilder{
            font,
            height: DEFAULT_HEIGHT,
            width: DEFAULT_WIDTH,
            color: DEFAULT_COLOR,
        }
    }
    pub fn create_sentence(&self, text: &str){
        let mut data = vec![];
        let text = text.to_string();
        let glyphs = self.font.glyphs_for(text.chars());
        for glyph in glyphs{
            let glyph = glyph.scaled(Scale{x:DEFAULT_WIDTH,y:DEFAULT_HEIGHT,}).positioned(Point{x:DEFAULT_POS.0,y:DEFAULT_POS.1,});
            let mut glyph_data = vec![];
            glyph.draw(|x,y,v |glyph_data.push((x,y)));
            data.push(glyph_data);
        }
    }
    pub fn get_color(){

    }
    pub fn color(){

    }
}
