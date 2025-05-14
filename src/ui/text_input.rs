use macroquad::prelude::*;

use super::{ Activation, Layout, WidgetData };


impl WidgetData {
    pub fn draw_text_input(absolute: Layout, primary: Color, secondary: Color, placeholder: &str, text: &str, selected: &bool) {
        let coords = absolute.as_rect();
        let (text, color) = if text.is_empty() {
            (placeholder, secondary.with_alpha(0.8))
        } else {
            (text, secondary)
        };
        
        draw_rectangle(coords.x, coords.y, coords.w, coords.h, primary);
        draw_text(text, coords.x, coords.y + coords.h, coords.h, color);
        if *selected {
            draw_rectangle_lines(coords.x, coords.y, coords.w, coords.h, 1.0, RED);
        }
    }
    
    pub fn activate_text_input(id: &str, text: &mut String, selected: &mut bool) -> Option<Activation> {
        if *selected {
            if is_key_pressed(KeyCode::Enter) {
                Some(Activation {id: id.to_string(), message: Some(text.clone())})
            } else {
                None
            }
        } else {
            None
        }
    }
}