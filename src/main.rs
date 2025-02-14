use p2p::P2PTalker;
use std::path::PathBuf;
use structopt::StructOpt;

mod client;
mod p2p;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    dst_addr: String,
    dst_port: usize,
    bind_port: usize,
}

#[tokio::main]
async fn main() {
    let Opt {
        dst_port,
        dst_addr,
        bind_port,
    } = Opt::from_args();

    let mut p2p = P2PTalker::new(dst_addr, dst_port, bind_port).await;
    p2p.talk().await.unwrap();
}
