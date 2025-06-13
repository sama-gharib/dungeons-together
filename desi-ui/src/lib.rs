//! This crate enables to create and manage very simple UIs with [macroquad](https://macroquad.rs/).

use macroquad::prelude::*;

use std::ops::Mul;

pub mod button;
pub mod frame;
pub mod label;
pub mod text_input;

// ==================== Data structures ==========================

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub center: Vec2,
    pub scale: Vec2,
}

#[derive(Debug, Clone)]
pub struct Activation {
    pub id: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Rest,
    Hovered,
    Pressed,
}

pub enum WidgetData {
    Frame {
        outline: f32,
    },
    Button {
        state: ButtonState,
    },
    Label {
        text: String,
        font_size: f32,
    },
    TextInput {
        placeholder: String,
        input: String,
        selected: bool,
    },
}

/// # Introduction
/// This is the main structure of this crate. It is defined like
/// a tree, which enables to define your widgets positions relative to their
/// parents.
/// Top-left is `vec2(-0.5, -0.5)`, bottom-right is `vec2(0.5, 0.5)` and center is `vec2(0.0, 0.0)`.
/// # Exemple
/// The following code creates a simple menu with a "Hello, World!" title and a "Click me" button.
/// ```
/// use macroquad::prelude::*;
/// use desi_ui::*;
/// #[macroquad::main("Exemple")]
/// async fn main() {
///    let mut ui = Widget::new(WidgetData::Frame { outline: 10.0 })
///        .with_primary(RED)
///        .with_secondary(BLUE)
///        .with_child(
///            Widget::new(WidgetData::Label { text: String::from("Hello, World!"), font_size: 32.0 })
///                .with_relative(Layout {
///                    center : vec2(0.0, -0.4),
///                    scale   : vec2(0.4, 0.2)
///                })
///        )
///        .with_child(
///            Widget::new(WidgetData::Button { state: ButtonState::Rest })
///                .with_id("the-click-me-button")
///                .with_relative(Layout {
///                    center: vec2(0.0, 0.0),
///                    scale: vec2(0.3, 0.3)
///                })
///                .with_child(Widget::new(WidgetData::Label { text: String::from("Click me"), font_size: 16.0 }))
///                    .with_primary(YELLOW)
///                    .with_secondary(GREEN)
///        );
///    
///    loop {
///        ui.update_absolutes( Layout { center: vec2(400.0, 300.0), scale: vec2(800.0, 600.0) } );
///        
///        for activation in ui.get_activations() {
///                 if &activation.id[..] == "the-click-me-button" {
///                     println!("Clicked !");
///                 } else {
///                     println!("{}", activation.id);
///                 }
///        }
///
///        ui.draw();
///
///        next_frame().await;
///    }
/// }
/// ```

pub struct Widget {
    id: String,

    data: WidgetData,

    absolute: Layout,
    relative: Layout,

    primary: Color,
    secondary: Color,

    children: Vec<Self>,
}

pub struct UiIterator<'a> {
    stack: Vec<&'a Widget>,
}

// ================ Implementations ===========================

impl Layout {
    pub fn new(center: Vec2, scale: Vec2) -> Self {
        Self { center, scale }
    }

    pub fn as_rect(&self) -> Rect {
        Rect {
            x: self.center.x - self.scale.x / 2.0,
            y: self.center.y - self.scale.y / 2.0,
            w: self.scale.x,
            h: self.scale.y,
        }
    }
}

impl Activation {
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
}

impl WidgetData {
    fn check_activation(&mut self, id: &str, layout: Layout) -> Option<Activation> {
        match self {
            WidgetData::Frame { .. } => Self::activate_frame(),
            WidgetData::Label { .. } => Self::activate_label(),
            WidgetData::Button { state, .. } => Self::activate_button(id, layout, state),
            WidgetData::TextInput {
                input, selected, placeholder
            } => Self::activate_text_input(id, layout, placeholder, input, selected),
        }
    }

    fn draw(&self, layout: Layout, primary: Color, secondary: Color) {
        match self {
            WidgetData::Frame { outline } => Self::draw_frame(layout, primary, secondary, *outline),
            WidgetData::Label { text, font_size } => {
                Self::draw_label(layout, primary, text, *font_size)
            }
            WidgetData::Button { state, .. } => {
                Self::draw_button(layout, primary, secondary, *state)
            }
            WidgetData::TextInput {
                placeholder,
                input,
                selected,
            } => Self::draw_text_input(layout, primary, secondary, placeholder, input, selected),
        }
    }
}

impl Widget {
    // === Constructor ===
    // == Classic ==
    pub fn new(data: WidgetData) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    // == Builders ==
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_owned();
        self
    }

    pub fn with_center(mut self, center: Vec2) -> Self {
        self.relative.center = center;
        self    
    }
    
    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.relative.scale = scale;
        self
    }
    
    pub fn with_relative(mut self, relative: Layout) -> Self {
        self.relative = relative;
        self
    }

    pub fn with_primary(mut self, color: Color) -> Self {
        self.primary = color;
        self
    }

    pub fn with_secondary(mut self, color: Color) -> Self {
        self.secondary = color;
        self
    }

    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    // === Recursives ===

    pub fn update_absolutes(&mut self, parent_absolute: Layout) {
        self.absolute = parent_absolute * self.relative;

        if let WidgetData::Label { font_size, text } = &mut self.data {
            let measures = measure_text(&text, None, *font_size as u16, 1.0);

            if measures.width > self.absolute.scale.x {
                *font_size *= self.absolute.scale.x / measures.width;
            }
            if measures.height > self.absolute.scale.y {
                *font_size *= self.absolute.scale.y / measures.height;
            }
        }

        for child in self.children.iter_mut() {
            child.update_absolutes(self.absolute)
        }
    }

    pub fn get_activations(&mut self) -> Vec<Activation> {
        let mut activations = Vec::new();

        for child in self.children.iter_mut() {
            activations.extend_from_slice(&child.get_activations());
        }

        if activations.is_empty() {
            let act = self.data.check_activation(&self.id, self.absolute);

            if let Some(act) = act {
                activations.push(act);
            }
        }

        activations
    }

    pub fn draw(&self) {
        self.data.draw(self.absolute, self.primary, self.secondary);

        for child in self.children.iter() {
            child.draw();
        }
    }

    // === Misc ===

    pub fn iter(&self) -> UiIterator {
        UiIterator { stack: vec![&self] }
    }
}

// ================== Trait implementations ====================

impl<'a> Iterator for UiIterator<'a> {
    type Item = &'a Widget;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.stack.pop();

        if let Some(current) = r {
            self.stack
                .extend_from_slice(&current.children.iter().collect::<Vec<_>>());
        }

        r
    }
}

// ============ Conversions ===========

impl From<&Widget> for Activation {
    fn from(w: &Widget) -> Self {
        Self {
            id: w.id.clone(),
            message: None,
        }
    }
}

// ============ Operators =============

impl Mul<Self> for Layout {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            scale: vec2(self.scale.x * other.scale.x, self.scale.y * other.scale.y),
            center: vec2(
                self.scale.x * other.center.x + self.center.x,
                self.scale.y * other.center.y + self.center.y,
            ),
        }
    }
}

// ============ Defaults =============

impl Default for Widget {
    fn default() -> Self {
        Self {
            id: String::from("Unnamed"),
            data: WidgetData::Frame { outline: 0f32 },
            relative: Layout::default(),
            absolute: Layout::default(),
            primary: BLACK,
            secondary: BLACK,
            children: Vec::default(),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            center: Vec2::ZERO,
            scale: Vec2::ONE,
        }
    }
}
