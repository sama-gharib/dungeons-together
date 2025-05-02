use std::net::{ SocketAddr, TcpListener, TcpStream };
use std::io::{ Error, Read, Write, BufRead, BufReader };
use std::ffi::c_void;

use crate::utils::DefaultBehaviour;

// Utils FFI
unsafe extern "C" {
    fn time(ptr: *mut c_void) -> usize;
}

type Client = (TcpStream, SocketAddr);

pub struct GameServer {
    listener: TcpListener,
    clients: Vec<Client>,
    
    to_broadcast: Vec<(String, SocketAddr)>,

    connection_string: String
}

impl DefaultBehaviour for GameServer {
    fn default_behaviour(&mut self) {
        self.accept_connection();
        self.receive_message();
    }
}

impl GameServer {
    pub fn new(connection_string: &str) -> Result<Self, Error> {
        let listener = TcpListener::bind(connection_string)?;
        listener.set_nonblocking(true)?;
        
        Ok(Self {
            listener,
            clients: Default::default(),
            to_broadcast: Default::default(),
            connection_string: connection_string.to_string()
        })
    }
    
    pub fn accept_connection(&mut self) {
        if let Ok(new_client) =  self.listener.accept() {
            if let Ok(()) = new_client.0.set_nonblocking(true) {
                self.log(&format!("New client connected : {}", new_client.1));
                self.clients.push(new_client);
            } else {
                self.log("Failed to set client non-blocking.");
            }
        }
    }
    
    pub fn receive_message(&mut self) {
        
        let mut received = 0;
        let mut disconnections = Vec::new();
        
        for (index, client) in self.clients.iter_mut().enumerate() {
            
            let mut buffer = Vec::<u8>::new();
            let mut reader = BufReader::new(&client.0);
            reader.skip_until(1).unwrap();
            reader.read_until(0, &mut buffer).unwrap();
                
            if buffer.len() == 0 {
                disconnections.push(index);
            } else {            
                let message = buffer
                    .into_iter()
                    .map(|x| x as char)
                    .collect();
                
                println!(">>> '{message}'");
                self.to_broadcast.push((message, client.1));
                received += 1;
            }
           
        }
        
        if received > 0 {
            self.log(&format!("Added {received} message(s) into the broadcast queue."));
        }
        
        for i in disconnections {
            self.log(&format!("Client {} disconnected.", self.clients[i].1));
            self.clients.remove(i);
        }
    }
    
    pub fn broadcast(&mut self) {
        
        let mut success = 0;
        let mut error = 0;
        for (message, source) in self.to_broadcast.iter() {
            for (stream, target) in self.clients.iter_mut() {
                if *source != *target {
                    if let Ok(_) = stream.write_all(message.as_bytes()) {
                        success += 1;
                    } else {
                        error += 1;
                    }
                }
            }
        }
        
        if success + error > 0 {
            self.log(
                    &format!
                    (
                        "Broadcasted {} messages to {} clients.\n\tSuccess: {success}\nFailures: {error}\t",
                        success + error,
                        self.clients.len()
                    )
            );
        }
        
        self.to_broadcast.clear();
    }
    
    fn log(&self, message: &str) {
        let (hour, minute, second) = Self::hour();
    
        println!(
            "[{}:{}:{}] {} > {message}",
            Self::base_format(hour, 10),
            Self::base_format(minute, 10),
            Self::base_format(second, 10),
            self.connection_string
        )
    }
    
    fn hour() -> (u8, u8, u8) {
        let unix_epoch = unsafe { time(0 as *mut c_void) };
        
        (
            ((unix_epoch / 3600 + 2) % 24) as u8,
            ((unix_epoch / 60)   % 60) as u8,
            ((unix_epoch)        % 60) as u8
        )
    }
    
    fn base_format(n: u8, base: u8) -> String {
        format!("{}{}", (n/base)%base, n%base)
    }
}