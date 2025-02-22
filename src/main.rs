use clap::Parser;
use p2p::P2PTalker;
use std::{
    io,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

mod client;
mod p2p;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    dst_addr: String,
    dst_port: usize,
    bind_port: usize,
}

#[tokio::main]
async fn main() {
    let Args {
        dst_port,
        dst_addr,
        bind_port,
    } = Args::parse();

    let msg = Arc::new(RwLock::new(String::new()));
    let c_lock = Arc::clone(&msg);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));

        let mut msg = match c_lock.try_write() {
            Ok(v) => v,
            Err(_) => continue,
        };
        msg.clear();
        io::stdin()
            .read_line(&mut msg)
            .expect("error: unable to read user input");
    });

    let mut p2p = P2PTalker::new(dst_addr, dst_port, bind_port).await;
    p2p.talk(msg).await.unwrap();
}
