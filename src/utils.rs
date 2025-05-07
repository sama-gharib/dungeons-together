use std::ffi::c_void;

use macroquad::math::Vec2;

unsafe extern "C" {
    fn srand(seed: usize);
    fn rand() -> usize;
    
    fn time(ptr: *mut c_void) -> usize;
}

pub struct Time;
impl Time {
   pub fn hour() -> (u8, u8, u8) {
        let unix_epoch = Self::unix_epoch();
    
        (
            ((unix_epoch / 3600 + 2) % 24) as u8,
            ((unix_epoch / 60)   % 60) as u8,
            ((unix_epoch)        % 60) as u8
        )
    }
    
    pub fn unix_epoch() -> usize {
        unsafe { time(0 as *mut c_void) }
    }
}

pub struct Random;
impl Random {
    pub fn seed() {
        unsafe { srand(Time::unix_epoch()) };
    }
    
    pub fn any() -> usize {
        unsafe {
            rand()
        }     
    }
    
    pub fn max(i: usize) -> usize {
        Self::any() % i
    }
    
    pub fn between(min: usize, max: usize) -> usize {
        Self::max(max - min) + min
    }
}

pub trait DefaultBehaviour {
    fn default_behaviour(&mut self);
}


pub fn base_format(n: u8, base: u8) -> String {
    format!("{}{}", (n/base)%base, n%base)
}