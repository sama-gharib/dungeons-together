use macroquad::prelude::*;

use std::str::FromStr;
use std::net::TcpStream;
use std::io::{ Read, BufRead, BufReader };

pub mod client;
pub mod server;

#[derive(Copy, Clone, Debug)]
pub struct ShareableType<T> {
    value: T
}

impl <T: FromStr> ShareableType<T> {
    const SEPARATOR: u8 = 47; // Slash
    
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



pub enum FormatError {
    EmptyMessage,
    MissingField,
    WrongType,
    InvalidValue,
    ByteAfterEnd
}

pub enum Command {
    Spawn (usize),
    Reposition (usize, Vec2),
    EndGame,
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
                    4 => Command::EndGame,
                    _ => Command::Unknown
                }
            },
            None => Command::IllFormated (FormatError::EmptyMessage)
        }
    }
}

pub enum ProtocolError {
    Disconnection,
    WrongSequence
}

pub struct Protocol;
impl Protocol {
    pub fn reception(stream: &mut TcpStream) -> Result<Command, ProtocolError>{
        let mut buffer = Vec::<u8>::new();
        let mut reader = BufReader::new(stream);
        if let (Ok(_), Ok(_)) = (reader.skip_until(1), reader.read_until(0, &mut buffer)) {   
            if buffer.len() == 0 {
                return Err(ProtocolError::Disconnection)
            } else {            
                return Ok(Command::from(&buffer[..]));
            }
        } else {
            Err(ProtocolError::WrongSequence)
        }
    }
}