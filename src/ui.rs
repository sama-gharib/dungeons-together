use macroquad::prelude::*;

use crate::utils::Random;
use crate::game::Drawable;

pub mod button;
pub mod label;

pub type Id = usize;

#[derive(Clone, Debug)]
pub enum Action {
    Toggle,
    Input (String),
    Press,
    Release,
    Activate,
    Hover,
    None,
    Dummy
}

#[derive(Debug)]
pub struct Activation {
    source: Id,
    action: Action
}
impl Activation {
    pub fn get_source(&self) -> usize {
        self.source
    }
    
    pub fn get_action(&self) -> Action {
        self.action.clone()
    }
}

#[derive(Clone)]
pub struct Layout {
    center: Vec2,
    size: Vec2
}

#[derive(Clone)]
pub struct Widget {
    id: Id,
    name: String,
    
    primary: Color,
    secondary: Color,
    
    relative: Layout,
    absolute: Rect,
    
    background: fn(&Self),
    foreground: fn(&Self),
    
    on_mouse_enter: fn(&mut Self),
    on_mouse_exit: fn(&mut Self),
    on_mouse_press: fn(&mut Self) -> Activation,
    on_mouse_release: fn(&mut Self) -> Activation,
    on_click: fn(&mut Self) -> Activation,
    
    mouse_in: bool,
    held: bool,
    
    children: Vec<Self>
}

impl Drawable for Widget {
    fn draw(&self) {
        (self.background)(self);
        (self.foreground)(self);
        
        for child in self.children.iter() {
            child.draw();
        }
    }
}

impl Default for Widget {
    fn default() -> Self {
        Self {
            id: Random::any(),
            name: String::new(),
            primary: WHITE,
            secondary: BLACK,
            relative: Layout { center: Vec2::ZERO, size: Vec2::ONE },
            absolute: Rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 },
            background: default_background,
            foreground: default_foreground,
            on_mouse_enter: nothing,
            on_mouse_exit: nothing,
            on_mouse_press: nothing_masking,
            on_mouse_release: nothing_masking,
            on_click: default_click_callback,
            mouse_in: false,
            held: false,
            children: Vec::new()
        }
    }
}

impl Widget {
    
    pub fn with_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }
    
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
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
    
    pub fn with_center(mut self, center: Vec2) -> Self {
        self.relative.center = center;
        self
    }
    
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.relative.size = size;
        self
    }
    
    pub fn with_mouse_press_callback(mut self, callback: fn(&mut Self) -> Activation) -> Self {
        self.on_mouse_press = callback;
        self
    }
    
    pub fn with_mouse_release_callback(mut self, callback: fn(&mut Self) -> Activation) -> Self {
        self.on_mouse_release = callback;
        self
    }
    
    pub fn with_mouse_enter_callback(mut self, callback: fn(&mut Self)) -> Self {
        self.on_mouse_enter = callback;
        self
    }
    
    pub fn with_click_callback(mut self, callback: fn(&mut Self) -> Activation) -> Self {
        self.on_click = callback;
        self
    }
    
    pub fn with_mouse_exit_callback(mut self, callback: fn(&mut Self)) -> Self {
        self.on_mouse_exit = callback;
        self
    }
    
    pub fn with_background(mut self, callback: fn(&Self)) -> Self {
        self.background = callback;
        self
    }
    
    pub fn with_foreground(mut self, callback: fn(&Self)) -> Self {
        self.foreground = callback;
        self
    }
    
    pub fn with_children(mut self, children: &mut [Self]) -> Self {
        self.children = children
            .iter_mut()
            .map(|x| std::mem::take(x))
            .collect();
        self
    }
    
    pub fn recalculate_absolutes(&mut self, ref_center: Vec2, ref_size: Vec2) {
        let size_prop = vec2(
            ref_size.x * self.relative.size.x,
            ref_size.y * self.relative.size.y  
        );
        let pos_prop = vec2(
            ref_center.x + self.relative.center.x * ref_size.x - size_prop.x / 2.0,
            ref_center.y + self.relative.center.y * ref_size.y - size_prop.y / 2.0
        );
        
        self.absolute = Rect {
            x: pos_prop.x,
            y: pos_prop.y,
            w: size_prop.x,
            h: size_prop.y
        };
        
        println!("{:?}", self.absolute);
        
        for child in self.children.iter_mut() {
            child.recalculate_absolutes(pos_prop + size_prop / 2.0, size_prop);
        }
    }
    
    pub fn get_activations(&mut self) -> Vec<Activation> {
        let mut activations: Vec<_> = self.children
            .iter_mut()
            .map(|child| child.get_activations())
            .flatten()
            .filter(|x| if let Action::None = x.action { false } else { true })
            .collect();
        
        
        let mut mouse_released = false;
        if is_mouse_button_released(MouseButton::Left) && self.held {
            mouse_released = true;
            self.held = false;
            self.log("released");
        }
        
        let mouse = mouse_position();
        if activations.is_empty()
        && mouse.0 > self.absolute.x 
        && mouse.0 < self.absolute.x + self.absolute.w
        && mouse.1 > self.absolute.y
        && mouse.1 < self.absolute.y + self.absolute.h {
                
                if !self.mouse_in {
                    (self.on_mouse_enter)(self);
                }
                
                self.mouse_in = true;
                if is_mouse_button_pressed(MouseButton::Left) {
                    activations.push((self.on_mouse_press)(self));
                    self.held = true;
                    self.log("held");
                } else if is_mouse_button_released(MouseButton::Left) {
                    self.log("clicked");
                    activations.push((self.on_click)(self));
                    self.held = false;
                } else if mouse_released{
                    activations.push(Activation { source: self.id, action: Action::Hover });
                }
        } else if self.mouse_in {
            (self.on_mouse_exit)(self);
            self.mouse_in = false;
        }
        
        if mouse_released {
            activations.push((self.on_mouse_release)(self));
        }
        
        return activations;
    }
    
    fn log(&self, msg: &str) {
        println!("{} >>> {msg}", self.name);
    }
}

pub fn default_background(s: &Widget) {
    draw_rectangle(s.absolute.x, s.absolute.y, s.absolute.w, s.absolute.h, s.secondary);
}

pub fn default_foreground(s: &Widget) {
    draw_text(&s.name, s.absolute.x, s.absolute.y + 100.0, 16.0, s.primary);
}

pub fn default_click_callback(s: &mut Widget) -> Activation {
    Activation {
        source: s.id,
        action: Action::Press
    }
}

pub fn default_hover_callback(s: &mut Widget) {
    std::mem::swap(&mut s.primary, &mut s.secondary);
}

pub fn nothing_masking(s: &mut Widget) -> Activation{
    Activation {
        source: s.id,
        action: Action::Dummy
    }
}

pub fn nothing(s: &mut Widget) {
    // Nothing...
}