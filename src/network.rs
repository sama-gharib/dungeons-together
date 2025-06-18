use macroquad::prelude::*;

use std::str::FromStr;
use std::net::TcpStream;
use std::io::{ BufRead, BufReader, ErrorKind, Read, Write };

use crate::utils::{ Dynamic, Drawable, Controlable };

pub mod client;
pub mod server;

pub trait GameAgent : Dynamic + Drawable + Controlable {}

#[derive(Copy, Clone, Debug)]
pub struct ShareableType<T> {
    value: T
}

impl <T: FromStr> ShareableType<T> {
    pub const SEPARATOR: u8 = 47; // Slash
    
    pub fn parse<'a>(i: &mut impl Iterator<Item=&'a u8>) -> Result<T, FormatError> {
        if let Ok(v) = Self::extract_next_field(i).parse::<T>() {
            Ok(v)
        } else {
            Err(FormatError::InvalidValue)
        }
    }
    
    fn extract_next_field<'a>(i: &mut impl Iterator<Item=&'a u8>) -> String {
        let mut buffer = String::new();
        
        loop {
            let s = *i.next().unwrap_or(&0);
            
            if s == Self::SEPARATOR || s == 0 {
                break;
            } else {
                buffer.push(s as char);
            }
        }
                
        buffer
    }
}


#[derive(Copy, Clone, Debug)]
pub enum FormatError {
    EmptyMessage,
    MissingField,
    WrongType,
    InvalidValue,
    ByteAfterEnd
}

#[derive(Copy, Clone, Debug)]
pub enum Command {
    Spawn (usize),
    Reposition (usize, Vec2),
    Despawn(usize),
    Unknown,
    IllFormated (FormatError)
}

impl From<&[u8]> for Command {
    fn from(s: &[u8]) -> Self {
        match s.get(0) {
            Some(command_id) => {
                let mut iterator = s.into_iter();
                iterator.next(); // Ignore command id
                match command_id {
                    2 => match ShareableType::<usize>::parse(&mut iterator) {
                        Ok(id) => Command::Spawn(id),
                        Err(e) => Command::IllFormated(e)
                    },
                    3 => match (ShareableType::<usize>::parse(&mut iterator), ShareableType::<f32>::parse(&mut iterator), ShareableType::<f32>::parse(&mut iterator)) {
                        (Ok(id), Ok(x), Ok(y)) => Command::Reposition(id, vec2(x, y)),
                        (Err(e0), _, _) => Command::IllFormated(e0),
                        (_, Err(e1), _) => Command::IllFormated(e1),
                        (_, _, Err(e2)) => Command::IllFormated(e2)
                    },
                    4 => match ShareableType::<usize>::parse(&mut iterator) {
                        Ok(id) => Command::Despawn(id),
                        Err(e) => Command::IllFormated(e)
                    },
                    _ => Command::Unknown
                }
            },
            None => Command::IllFormated (FormatError::EmptyMessage)
        }
    }
}

impl Command {
    pub fn as_bytes(&self) -> Vec<u8>{        
        let separator: u8 = ShareableType::<u8>::SEPARATOR;
        
        let (header, body) = match self {
            Command::Spawn(id) => (2u8, Vec::from(id.to_string().as_bytes())),
            Command::Reposition(id, pos) => (
                3,
                id
                    .to_string()
                    .as_bytes()
                    .into_iter()
                    .chain(&[separator])
                    .chain(
                        pos.x
                            .to_string()
                            .as_bytes()
                    )
                    .chain(&[separator])
                    .chain(pos.y.to_string().as_bytes())
                    .map(|x| *x)
                    .collect()
            ),
            Command::Despawn(id) => (
                4,
                id
                    .to_string()
                    .as_bytes()
                    .into_iter()
                    .map(|x| *x)
                    .collect()
            ),
            Command::IllFormated(_) => (5, Vec::new()),
            Command::Unknown => (6, Vec::new())
        };
        
        [header].into_iter().chain(body.into_iter()).collect::<Vec<u8>>() 
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    Disconnection,
    WrongSequence,
    OutdatedPackage,
    IllFormatedSequenceNumber
}

#[derive(Debug)]
pub struct Protocol {
    last_reception: [u8; 4],
    last_send: [u8; 4]
}

impl Protocol {
    pub fn new() -> Self {
        Self {
            last_reception: [0; 4],
            last_send: [0; 4]
        }
    }
    
    pub fn reception(&mut self, stream: &mut TcpStream) -> Result<Command, ProtocolError> {
        let mut timestamp_buffer = [0u8;4];
        let mut body_buffer = Vec::<u8>::new();
        let mut reader = BufReader::new(stream);
        match (reader.skip_until(1), reader.read_exact(&mut timestamp_buffer), reader.read_until(0, &mut body_buffer)) {   
            (Ok(_), Ok(_), Ok(_)) => 
                if Self::lower_4_bytes(&self.last_reception, &timestamp_buffer) {
                    self.last_reception = timestamp_buffer;
                    if body_buffer.len() == 0 {
                        return Err(ProtocolError::Disconnection)
                    } else {   
                        return Ok(Command::from(&body_buffer[..body_buffer.len()-1]));
                    }
                } else {
                    Err(ProtocolError::OutdatedPackage)
                },
            (_, Err(e), _) => {
                if let ErrorKind::UnexpectedEof = e.kind() {
                    Err(ProtocolError::Disconnection)
                } else {
                    Err(ProtocolError::IllFormatedSequenceNumber)
                }
            },
            (_, _, _) => Err(ProtocolError::WrongSequence)
        } 
    }
    
    pub fn send(&mut self, stream: &mut TcpStream, command: Command) -> Result<(), std::io::Error> {
        let message = [1]
                .into_iter()
                .chain(self.last_send)
                .chain(command.as_bytes())
                .chain([0])
                .collect::<Vec<u8>>();
        
        Self::increment_4_bytes(&mut self.last_send);
        
        stream.write_all(&message)
    }
    
    fn lower_4_bytes(a: &[u8; 4], b: &[u8; 4]) -> bool {
        for i in 0..4 {
            if a[i] < b[i] {
                return true;
            } else if a[i] > b[i] {
                return false;
            }
        }
        return true;
    }
    
    fn increment_4_bytes(target: &mut [u8;4]) {
        for i in (0..4).rev() {
            if !Self::increment_byte(&mut target[i]) {
                break;
            }
        }
    }
    
    fn increment_byte(target: &mut u8) -> bool {
        *target = (*target + 1) % u8::MAX;
        
        *target == 0
    }
}