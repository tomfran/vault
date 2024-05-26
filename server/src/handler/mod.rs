use crate::{executor::ExecutorFactory, protocol::parser::parse_command};
use log::{debug, error};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const BUFFER_SIZE: usize = 4096;

pub struct Handler {
    executor_factory: ExecutorFactory,
    stream: TcpStream,
}

impl Handler {
    pub fn new(executor_factory: ExecutorFactory, stream: TcpStream) -> Handler {
        Handler {
            executor_factory,
            stream,
        }
    }

    pub async fn handle_loop(&mut self) {
        let mut buffer = [0; BUFFER_SIZE];

        loop {
            match self.stream.read(&mut buffer).await {
                Ok(n) => {
                    if n == 0 {
                        debug!("Read 0 bytes from client stream, closing handler");
                        break;
                    }

                    if let Some(response) = self.handle_command(&buffer[..n]) {
                        match self.stream.write_all(&response).await {
                            Ok(_) => {
                                debug!("Sent response");
                            }
                            Err(e) => {
                                error!("Error while responsing to client {}", e);
                            }
                        };
                    }
                }
                Err(e) => {
                    error!("Error while reading client message {}", e);
                }
            }
        }
    }

    fn handle_command(&self, command: &[u8]) -> Option<Box<[u8]>> {
        parse_command(command)
            .map(|c| self.executor_factory.build_executor(c))
            .map(|e| e.execute())
    }
}
