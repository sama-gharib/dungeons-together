use macroquad::prelude::*;

use super::{ Activation, Action, Widget };

const MARGIN: f32 = 0.04;

impl Widget {
    pub fn default_button() -> Self {
        Widget::default()
            .with_name("Button")
            .with_center(vec2(0.0, 0.0))
            .with_size(vec2(0.2, 0.1))
            .with_mouse_press_callback(mouse_press_callback)
            .with_mouse_release_callback(mouse_release_callback)
            .with_click_callback(mouse_click_callback)
            .with_background(background)
            .with_foreground(foreground)
            .with_mouse_enter_callback(mouse_enter_callback)
            .with_mouse_exit_callback(mouse_exit_callback)
            .with_primary(RED)
            .with_secondary(BLUE)
    }
}

pub fn background(s: &Widget) {
    let margin = s.absolute.w * MARGIN;
    
    draw_rectangle(
        s.absolute.x,
        s.absolute.y,
        s.absolute.w - margin,
        s.absolute.h,
        s.primary
    );
    draw_triangle(
        vec2(s.absolute.x + s.absolute.w - margin, s.absolute.y + s.absolute.h - margin),
        vec2(s.absolute.x + s.absolute.w, s.absolute.y + s.absolute.h),
        vec2(s.absolute.x + s.absolute.w - margin, s.absolute.y + s.absolute.h),
        s.primary
    );
    
    draw_rectangle(
        s.absolute.x,
        s.absolute.y,
        s.absolute.w,
        s.absolute.h - margin,
        s.secondary
    );
    draw_triangle(
        vec2(s.absolute.x + s.absolute.w - margin, s.absolute.y + s.absolute.h - margin),
        vec2(s.absolute.x + s.absolute.w, s.absolute.y + s.absolute.h),
        vec2(s.absolute.x + s.absolute.w, s.absolute.y + s.absolute.h - margin),
        s.secondary
    );
    
}

pub fn foreground(s: &Widget) {
    let margin = s.absolute.w * MARGIN;
    
    draw_rectangle(
        s.absolute.x,
        s.absolute.y,
        s.absolute.w - margin,
        s.absolute.h - margin,
        if s.mouse_in { Color::from_rgba(210, 210, 210, 255) } else { WHITE }
    );
}

pub fn mouse_press_callback(s: &mut Widget) -> Activation {
    std::mem::swap(&mut s.primary, &mut s.secondary);
    Activation {
        source: s.id,
        action: Action::Press
    }
}

pub fn mouse_release_callback(s: &mut Widget) -> Activation {
    std::mem::swap(&mut s.primary, &mut s.secondary);
    
    Activation {
        source: s.id,
        action: Action::Release
    }
}

pub fn mouse_click_callback(s: &mut Widget) -> Activation {
    Activation {
        source: s.id,
        action: Action::Activate
    }
}

pub fn mouse_enter_callback(s: &mut Widget) {
    s.primary.r *= 0.95;
    s.primary.g *= 0.95;
    s.primary.b *= 0.95;
    s.secondary.r *= 0.95;
    s.secondary.g *= 0.95;
    s.secondary.b *= 0.95;
}

pub fn mouse_exit_callback(s: &mut Widget) {
    s.primary.r *= 1.05;
    s.primary.g *= 1.05;
    s.primary.b *= 1.05;
    s.secondary.r *= 1.05;
    s.secondary.g *= 1.05;
    s.secondary.b *= 1.05;
}
