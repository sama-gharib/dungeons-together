use std::ffi::c_void;
use std::mem;
use std::ops::{ Index, IndexMut };

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

#[derive(Clone, Debug)]
pub struct DiscriminantMap<Key, Value> {
    data: Vec<(Key, Value)>
}

impl <Key, Value> Default for DiscriminantMap<Key, Value> {
    fn default() -> Self {
        Self {
            data: Vec::new()
        }
    }
}

impl <Key, Value> Index<&Key> for DiscriminantMap<Key, Value> {
    type Output = Value;
    
    fn index(&self, what: &Key) -> &Self::Output {
        self.get(what).expect("Index not found.")
    }
}

impl <Key, Value> IndexMut<&Key> for DiscriminantMap<Key, Value> {
    
    fn index_mut(&mut self, what: &Key) -> &mut Self::Output {
        self.get_mut(what).expect("Index not found.")
    }
}

impl <Key, Value> DiscriminantMap<Key, Value> {
    pub fn push(&mut self, key: Key, value: Value) {
        self.data.push((key, value));
    }
    
    pub fn get(&self, what: &Key) -> Option<&Value> {
        for (key, value) in self.data.iter() {
            if mem::discriminant(what) == mem::discriminant(&key) {
                return Some(&value);
            }
        }
        None
    }
    
    pub fn get_mut(&mut self, what: &Key) -> Option<&mut Value> {
        for (key, value) in self.data.iter_mut() {
            if mem::discriminant(what) == mem::discriminant(&key) {
                return Some(value);
            }
        }
        None   
    }
    
    pub fn has_value_for(&self, what: Key) -> bool {
        self.data
            .iter()
            .map(|(key, _)| 
                mem::discriminant(key)
            )
            .collect::<Vec<_>>()
            .contains(&mem::discriminant(&what))
    }
}

pub trait Drawable {
    fn draw(&self);
}

pub trait Dynamic {
    fn update(&mut self);
}

pub trait Controlable {
    fn handle_events(&mut self) -> bool;
}
