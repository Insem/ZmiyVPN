use clap::Parser;
use p2p::P2PTalker;

mod client;
mod p2p;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    dst_addr: String,
    dst_port: usize,
    bind_port: usize,
    #[arg(short, requires("is_node"))]
    url: Option<String>,
    #[arg(short, long)]
    is_node: bool,
}

#[tokio::main]
async fn main() {
    let Args {
        dst_addr,
        dst_port,
        bind_port,
        url,
        is_node,
    } = Args::parse();

    let mut p2p = P2PTalker::new(dst_addr, dst_port, bind_port, is_node).await;
    if url.is_some() {
        p2p.add_req(url.unwrap());
    };
    p2p.talk().await.unwrap();
}
