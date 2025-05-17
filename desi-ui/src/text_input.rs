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
        text: &mut String,
        selected: &mut bool,
    ) -> Option<Activation> {
        let mouse = mouse_position();
        let coords = absolute.as_rect();

        let mouse_in = coords.contains(Vec2::from(mouse));

        if *selected {
            if is_mouse_button_pressed(MouseButton::Left) && !mouse_in {
                *selected = false;
            }

            if is_key_pressed(KeyCode::Enter) {
                *selected = false;
                Some(Activation {
                    id: id.to_string(),
                    message: Some(text.clone()),
                })
            } else {
                if let Some(k) = get_last_key_pressed() {
                    if k as usize >= KeyCode::A as usize && k as usize <= KeyCode::Z as usize {
                        text.push(format!("{k:?}").chars().next().unwrap());
                    } else {
                        match k {
                            KeyCode::Backspace => {
                                let _ = text.pop();
                            }
                            KeyCode::Space => text.push(' '),
                            _ => {
                                eprintln!("Unhandled key code : {k:?}");
                            }
                        }
                    }
                }

                None
            }
        } else {
            if mouse_in && is_mouse_button_pressed(MouseButton::Left) {
                *selected = true;
            }

            None
        }
    }
}
