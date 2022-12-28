use crate::prelude::*;
use std::net::{SocketAddr, TcpListener};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(config: &Config) -> Result<Self> {
        let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

        let server = Server {
            listener: TcpListener::bind(addr)?,
        };
        println!("SicQL is listening on port {}", config.port);
        Ok(server)
    }

    pub fn run(&self) -> Result<()> {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            println!("{:?}", stream)
        }

        Ok(())
    }
}
