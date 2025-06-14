use macroquad::prelude::*;

use super::{Activation, Layout, WidgetData};

impl WidgetData {
    pub fn draw_text_input(
        absolute: Layout,
        primary: Color,
        secondary: Color,
        placeholder: &str,
        text: &str,
        selected: &bool,
    ) {
        let coords = absolute.as_rect();
        let (text, color) = if text.is_empty() {
            (placeholder, secondary.with_alpha(0.8))
        } else {
            (text, secondary)
        };

        draw_rectangle(coords.x, coords.y, coords.w, coords.h, primary);
        draw_text(text, coords.x, coords.y + coords.h, coords.h, color);
        draw_rectangle_lines(
            coords.x,
            coords.y,
            coords.w,
            coords.h,
            2.0,
            if *selected { RED } else { DARKGRAY },
        );
    }

    pub fn activate_text_input(
        id: &str,
        absolute: Layout,
        placeholder: &str,
        text: &mut String,
        selected: &mut bool,
    ) -> Option<Activation> {
        let mouse = mouse_position();
        let coords = absolute.as_rect();

        let mouse_in = coords.contains(Vec2::from(mouse));


        if is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left) && !mouse_in {
            *selected = false;
            Some(Activation {
                id: id.to_string(),
                message: Some(if text.is_empty() { placeholder.to_string() } else { text.clone() }),
            })
        } else if *selected {
            if let Some(k) = get_last_key_pressed() {
                match k {
                    KeyCode::Backspace => {
                        let _ = text.pop();
                    },
                    KeyCode::Space => text.push(' '),
                    KeyCode::LeftShift | KeyCode::RightShift => {
                        // Ignored keys
                    },
                    _ => {
                        text.push(get_char_pressed().unwrap_or('?'));
                        if measure_text(&text, None, coords.h as u16, 1.0).width > coords.w {
                            text.pop();
                            println!("FIXME: Text to long for text input")
                        }
                    }
                }
            }

            None
        } else if mouse_in && is_mouse_button_pressed(MouseButton::Left) {
            *selected = true;
            None
        } else {
            None
        }
    }
}
