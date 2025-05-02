use std::net::{ SocketAddr, TcpListener, TcpStream };
use std::io::{ Error, Read, Write };

// Utils FFI
unsafe extern "C" {
    fn time() -> usize;
}

type Client = (TcpStream, SocketAddr);

pub struct GameServer {
    listener: TcpListener,
    clients: Vec<Client>,
    
    to_broadcast: Vec<(String, SocketAddr)>,

    connection_string: String
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
    
    pub fn default_behaviour(&mut self) {
        self.accept_connection();
        self.receive_message();
    }
    
    pub fn accept_connection(&mut self) {
        if let Ok(new_client) =  self.listener.accept() {
            if let Ok(()) = new_client.0.set_nonblocking(true) {
                self.log(&format!("New client connected : {}", new_client.1));
                self.clients.push(new_client)
            } else {
                self.log("Failed to set client non-blocking.");
            }
        }
    }
    
    pub fn receive_message(&mut self) {
        
        let mut received = 0;
        for client in self.clients.iter_mut() {
            let mut buffer = String::new();
            if let Ok(_) = client.0.read_to_string(&mut buffer) {
                self.to_broadcast.push((buffer, client.1));
                received += 1;
            }
        }
        
        if received > 0 {
            self.log(&format!("Added {received} message(s) into the broadcast queue."));
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
            Self::base_format(second,10),
            self.connection_string
        )
    }
    
    fn hour() -> (u8, u8, u8) {
        let unix_epoch = unsafe { time() };
        
        (
            ((unix_epoch / 3600) % 24) as u8,
            ((unix_epoch / 60)   % 60) as u8,
            ((unix_epoch)        % 60) as u8
        )
    }
    
    fn base_format(n: u8, base: u8) -> String {
        format!("{}{}", (n/base)%base, n%base)
    }
}