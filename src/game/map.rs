use macroquad::prelude::*;

use std::ops::{BitAnd, BitOr};
use std::collections::VecDeque;

use super::component::GameComponent;
use super::object::GameObject;
use super::body::Body;
use super::controller::Controller;
use super::keys::KeyBinding;

use crate::network::server::GameServer;
use crate::utils::Random;
use crate::utils::srand;

pub struct Direction {
    flag: AccessFlag,
    vector: (i32, i32)
}
impl Direction {
    pub const VALUES: [Self; 5] = [
        Self::NONE,
        Self::UP,
        Self::LEFT,
        Self::DOWN,
        Self::RIGHT
    ];
    
    pub const NONE: Self  = Self { flag: AccessFlag (0b00000000), vector: (0, 0) };
    pub const UP: Self    = Self { flag: AccessFlag (0b00000001), vector: (-1, 0) };
    pub const LEFT: Self  = Self { flag: AccessFlag (0b00000010), vector: (0, -1) };
    pub const DOWN: Self  = Self { flag: AccessFlag (0b00000100), vector: (1, 0) };
    pub const RIGHT: Self = Self { flag: AccessFlag (0b00001000), vector: (0, 1) };
    
    pub fn flag_to_index(f: u8) -> usize {
        match f {
            0b00000001 => 1,
            0b00000010 => 2,
            0b00000100 => 3,
            0b00001000 => 4,
            _ => 0
        }
    }
}

macro_rules! combine_flags {
    ($first_flag: expr $(,$flag: expr)*) => {
        AccessFlag ( $first_flag.flag.0 $( | $flag.flag.0)*)
    };
}

#[derive(Debug, Default, Clone)]
pub struct AccessFlag (u8);
impl BitOr for AccessFlag {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        self.with(other)
    }
}

impl BitAnd for AccessFlag {
    type Output = Self;
    fn bitand(self, other: Self) -> Self::Output {
        Self ( self.0 & other.0 )
    }
}

impl AccessFlag {
    pub fn opposite(self) -> Self {
        match self.0 {
            0b00000001 => Self (0b00000100),
            0b00000010 => Self (0b00001000),
            0b00000100 => Self (0b00000001),
            0b00001000 => Self (0b00000010),
            _ => self,
        }
    }
    
    pub fn with(self, other: Self) -> Self {
        Self ( self.0 | other.0 )
    }
    
    pub fn without(self, other: Self) -> Self {
        let mut result = AccessFlag(0);
        for i in 0..8 {
            let a = ((self.0 >> i) % 2) == 1;
            let b = ((other.0 >> i ) % 2) == 1;
            let r = a && !b;
            result.0 += (r as u8) << i;
        }
        result
    }
    
    pub fn directions(&self) -> impl Iterator<Item=((i32, i32), AccessFlag)> {
        (0..8)
            .filter_map(|x| {
                let bit = (self.0 >> x) % 2 == 1;
                if bit {
                    let i = 1 << x;
                    Some( (Direction::VALUES[Direction::flag_to_index(i)].vector, AccessFlag(i)) )
                } else {
                    None
                }
            })
    }
}

#[derive(Debug, Default)]
pub struct Map {
    pub rooms: Vec<Vec<Chunk>>
}

#[derive(Clone, Debug)]
pub enum Chunk {
    Uninitialized,
    InQueue,
    Generated (Room)
}

impl Map {
    
    pub fn get_rooms_iterator(&self) -> impl Iterator<Item = &Room> {
        self.rooms
            .iter()
            .flatten()
            .filter_map(|x|
                if let Chunk::Generated(room) = x {
                    Some(room)
                } else {
                    None
                }
            )
    }
    
    pub fn generate(max_width: usize, max_height: usize, seed: usize) -> Self {
        
        unsafe { srand(seed); }
        
        let mut room_matrix: Vec<Vec<Chunk>> = vec![vec![Chunk::Uninitialized; max_width]; max_height];
        // Using a VecDeque rather than a Vec enables to easily switch from DFS to BFS 
        let mut generation_stack = VecDeque::<((usize, usize), AccessFlag)>::new();
        
        generation_stack.push_back(((max_height/2, max_width/2), Direction::UP.flag));
        room_matrix[max_height/2][max_width/2] = Chunk::Generated(Room::CROSSROADS);
                  
        loop {
            match generation_stack.pop_front() {
                Some((cursor, constraint)) => {                    
                    let constraint = constraint.opposite();
                    
                    let new_room = Random::choice_if(
                        &Room::VARIANTS,
                        |x: &Room| (x.access_flag.clone() & constraint.clone()).0 != 0
                    );
                    
                    if let None = new_room {
                        continue;
                    }
                    
                    let new_room = new_room.unwrap();
                    
                    for (direction, flag) in new_room.access_flag.directions() {
                        let mut next_coord = (cursor.0 as i32 + direction.0, cursor.1 as i32 + direction.1);
                        next_coord.0 = next_coord.0.min(max_height as i32 - 1).max(0);
                        next_coord.1 = next_coord.1.min(max_width as i32 - 1).max(0);
                        let next_coord = (next_coord.0 as usize, next_coord.1 as usize);
                        
                        if let Chunk::Uninitialized = room_matrix[next_coord.0][next_coord.1] {
                            generation_stack.push_back((next_coord, flag));
                            room_matrix[next_coord.0][next_coord.1] = Chunk::InQueue;
                        }
                    }    
                    
                    room_matrix[cursor.0][cursor.1] = Chunk::Generated(
                        new_room.with_indices(
                            cursor.0 as i32 - GameServer::MAP_HEIGHT as i32 / 2, 
                            cursor.1 as i32 - GameServer::MAP_WIDTH as i32 / 2
                        )
                    );             
                },
                None => break
            }
        }
                
        Map {
            rooms: room_matrix
        }
    }
}

macro_rules! wall {
    ($x: literal, $y: literal, $w: literal, $h: literal) => {
        Some(
            GameComponent {
                body: Body {
                    position: Vec2 { x: $x * Room::WIDTH, y: $y * Room::HEIGHT },
                    size: Vec2 { x: $w * Room::WIDTH, y: $h * Room::HEIGHT },
                    friction_factor: 1.0,
                    velocity: Vec2::ZERO
                },
                object: GameObject::Wall,
                controller: Controller::BrainDead
          }
        )
    };
}

#[derive(Default, Debug, Clone)]
pub struct Room {
    access_flag: AccessFlag,
    pub components: [Option<GameComponent>; 4]
}

impl Room {
    pub const WIDTH:  f32 = 800.0;
    pub const HEIGHT: f32 = 600.0;
    
    pub const VARIANTS: [Self; 8] = [
        Self::H_CORRIDOR,
        Self::V_CORRIDOR,
        Self::CROSSROADS,
        Self::T_UP,
        Self::T_LEFT,
        Self::T_DOWN,
        Self::T_RIGHT,
        Self::BLOCKED
    ];
    
    pub const EMPTY: Self = Self {
        access_flag: combine_flags!(Direction::UP, Direction::LEFT, Direction::DOWN, Direction::RIGHT),
        components: [
            None,
            None,
            None,
            None
        ]
    };
    
    pub const H_CORRIDOR: Self = Self {
        access_flag: combine_flags!(Direction::LEFT, Direction::RIGHT),
        components: [
            wall!(0.0, 0.0, 1.0, 0.25),
            wall!(0.0, 0.75, 1.0, 0.25),
            None,
            None
        ]
    };
    
    pub const V_CORRIDOR: Self = Self {
        access_flag: combine_flags!(Direction::UP, Direction::DOWN),
        components: [
            wall!(0.0, 0.0, 0.25, 1.0),
            wall!(0.75, 0.0, 0.25, 1.0),
            None,
            None
        ]
    };
    
    pub const T_UP: Self = Self {
        access_flag: combine_flags!(Direction::UP, Direction::LEFT, Direction::RIGHT),
        components: [
            wall!(0.0, 0.0, 0.25, 0.25),
            wall!(0.75, 0.0, 0.25, 0.25),
            wall!(0.0, 0.75, 1.0, 0.25),
            None
        ]
    };
    
    pub const T_LEFT: Self = Self {
        access_flag: combine_flags!(Direction::UP, Direction::LEFT, Direction::DOWN),
        components: [
            wall!(0.0, 0.0, 0.25, 0.25),
            wall!(0.0, 0.75, 0.25, 0.25),
            wall!(0.75, 0.0, 0.25, 1.0),
            None
        ]
    };
    
    pub const T_DOWN: Self = Self {
        access_flag: combine_flags!(Direction::LEFT, Direction::DOWN, Direction::RIGHT),
        components: [
            wall!(0.0, 0.0, 1.0, 0.25),
            wall!(0.0, 0.75, 0.25, 0.25),
            wall!(0.75, 0.75, 0.25, 0.25),
            None
        ]
    };
    
    pub const T_RIGHT: Self = Self {
        access_flag: combine_flags!(Direction::UP, Direction::DOWN, Direction::RIGHT),
        components: [
            wall!(0.0, 0.0, 0.25, 1.0),
            wall!(0.75, 0.0, 0.25, 0.25),
            wall!(0.75, 0.75, 0.25, 0.25),
            None
        ]
    };
    
    pub const CROSSROADS: Self = Self {
        access_flag: combine_flags!(Direction::UP, Direction::LEFT, Direction::DOWN, Direction::RIGHT),
        components: [
            wall!(0.0, 0.0, 0.05, 0.05),
            wall!(0.0, 0.95, 0.05, 0.05),
            wall!(0.95, 0.0, 0.05, 0.05),
            wall!(0.95, 0.95, 0.05, 0.05)
        ]
    };
    
    pub const BLOCKED: Self = Self {
        access_flag: combine_flags!(Direction::NONE),
        components: [
            wall!(0.0, 0.0, 1.0, 1.0),
            None,
            None,
            None
        ]
    };
    
    pub fn with_indices(mut self, line: i32, column: i32) -> Self {
        for e in self.components.iter_mut() {
            if let Some(wall) = e {
                wall.body.position.x += column as f32 * Room::WIDTH;
                wall.body.position.y += line as f32 * Room::HEIGHT;
            }
        }
        self
    }
    
}