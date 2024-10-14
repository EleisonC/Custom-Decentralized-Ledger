use std::{error::Error, net::TcpListener, sync::Arc, thread, sync::mpsc::channel};


use crate::domain::{Block, Blockchain};



pub struct Server {
   blockchain: Arc<Blockchain>, 
}

impl Server {
    pub fn new (blockchain: Blockchain) -> Server {
        Server {
            blockchain: Arc::new(blockchain)
        }
    }

    pub fn run(&self, addr: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).map_err(|e| format!("Failed to bind to addr {}: {}", addr, e))?;

        if addr.eq(CENTRAL_NODE) == false {
            let best_height = self.blockchain.get_best_height();
            send_version(CENTRAL_NODE, best_height);
        }

        let (tx, rx) = channel();

        for stream in listener.incoming() {
            let blockchain = self.blockchain.clone();
            let tx = tx.clone();
            thread::spawn(|| match  stream {
                Ok(stream) => {
                    // will be sending it to the serve function
                }
                Err(e) => {
                    tx.send(format!("Error handling incoming stream: {}", e));
                }
            });
        }

        for recev in rx {
            return  Err(recev.into())
        }
        Ok(())
    }
}