use macroquad::prelude::*;

use super::{Activation, Layout, WidgetData};

impl WidgetData {
    pub fn draw_frame(absolute: Layout, primary: Color, secondary: Color, outline: f32) {
        let coords = absolute.as_rect();

        draw_rectangle(coords.x, coords.y, coords.w, coords.h, primary);

        draw_rectangle_lines(coords.x, coords.y, coords.w, coords.h, outline, secondary);
    }

    pub fn activate_frame() -> Option<Activation> {
        None
    }
}
