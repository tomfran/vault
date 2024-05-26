mod executor;
mod handler;
mod protocol;
mod storage;

use std::sync::{Arc, Mutex};

use crate::{executor::ExecutorFactory, handler::Handler, storage::Storage};
use log::{info, LevelFilter};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Server started!");

    let storage = Arc::new(Mutex::new(Storage::default()));
    let executor_factory = ExecutorFactory::new(storage.clone());

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let mut handler = Handler::new(executor_factory.clone(), stream);
        tokio::spawn(async move {
            handler.handle_loop().await;
        });
    }
}
