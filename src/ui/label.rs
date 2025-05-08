use macroquad::prelude::*;

use super::{ Activation, Action, Widget };

impl Widget {
    pub fn default_label() -> Self {
        Widget::default()
            .with_name("Title")
            .with_center(vec2(0.0, 0.0))
            .with_size(vec2(0.8, 0.6))
            .with_background(background)
            .with_foreground(foreground)
            .with_mouse_press_callback(mouse_callback)
            .with_mouse_release_callback(mouse_callback)
            .with_click_callback(mouse_callback)
    }
}

pub fn background(s: &Widget) {
    // super::default_background(s);
    draw_rectangle(
        s.absolute.x,
        s.absolute.y + s.absolute.h * 0.9,
        s.absolute.w,
        s.absolute.h * 0.1,
        s.secondary
    );
}

pub fn foreground(s: &Widget) {
    let font_size = s.absolute.h;
    let measures = measure_text(&s.name, None, font_size as u16, 1.0);
    
    draw_text(
        &s.name,
        s.absolute.x + (s.absolute.w - measures.width) / 2.0,
        s.absolute.y + (s.absolute.h + measures.height) / 2.0,
        font_size,
        s.primary
    );
}

pub fn mouse_callback(s: &mut Widget) -> Activation {
    Activation {
        source: s.id,
        action: Action::None
    }
}
