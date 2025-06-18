use macroquad::prelude::*;

use super::{Activation, ButtonState, Layout, WidgetData};

impl WidgetData {
    pub fn activate_button(
        id: &str,
        absolute: Layout,
        state: &mut ButtonState,
    ) -> Option<Activation> {
        let mouse = mouse_position();
        let coords = absolute.as_rect();

        let mouse_in = coords.contains(Vec2::from(mouse));

        match state {
            ButtonState::Rest => {
                *state = if mouse_in {
                    ButtonState::Hovered
                } else {
                    ButtonState::Rest
                };
                None
            }
            ButtonState::Hovered => {
                *state = if !mouse_in {
                    ButtonState::Rest
                } else if is_mouse_button_pressed(MouseButton::Left) {
                    ButtonState::Pressed
                } else {
                    ButtonState::Hovered
                };
                None
            }
            ButtonState::Pressed => {
                let released = is_mouse_button_released(MouseButton::Left);
                if mouse_in && released {
                    *state = ButtonState::Hovered;
                    Some(Activation {
                        id: id.to_string(),
                        message: None,
                    })
                } else if released {
                    *state = ButtonState::Rest;
                    None
                } else {
                    *state = ButtonState::Pressed;
                    None
                }
            }
        }
    }

    pub fn draw_button(absolute: Layout, primary: Color, secondary: Color, state: ButtonState) {
        let coords = absolute.as_rect();
        let color = match state {
            ButtonState::Rest => primary,
            ButtonState::Hovered => primary.with_alpha(0.8),
            ButtonState::Pressed => secondary,
        };

        draw_rectangle(coords.x, coords.y, coords.w, coords.h, color);
    }
}
