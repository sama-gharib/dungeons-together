use macroquad::prelude::*;

use super::{ Activation, Layout, WidgetData };


impl WidgetData {
    pub fn draw_label(absolute: Layout, primary: Color, text: &str, font_size: f32) {
        let coords = absolute.as_rect();
        
        draw_text(text, coords.x, coords.y + font_size, font_size, primary);
    }
    
    pub fn activate_label() -> Option<Activation> {
        None
    }
}