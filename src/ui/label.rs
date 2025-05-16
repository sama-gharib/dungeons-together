use macroquad::prelude::*;

use super::{ Activation, Layout, WidgetData };


impl WidgetData {
    pub fn draw_label(absolute: Layout, primary: Color, text: &str, font_size: f32) {
        let coords = absolute.as_rect();
        let measures = measure_text(text, None, font_size as u16, 1.0);
        
        draw_text(
            text,
            coords.x + (coords.w - measures.width) / 2.0,
            coords.y + (coords.h + measures.height) / 2.0,
            font_size,
            primary
        );
    }
    
    pub fn activate_label() -> Option<Activation> {
        None
    }
}