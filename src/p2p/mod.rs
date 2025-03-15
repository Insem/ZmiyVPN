use reqwest::Url;
use std::io::{self, ErrorKind};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use tokio::io::Interest;
use tokio::net::{TcpSocket, TcpStream};

pub struct P2PTalker {
    is_node: bool,
    stream: TcpStream,
    queue: Vec<Url>,
}
impl P2PTalker {
    pub async fn new(dst_addr: String, dst_port: usize, bind_port: usize, is_node: bool) -> Self {
        println!("--Arhs {:?} {:?} {:?}", dst_addr, dst_port, bind_port);
        let socket = TcpSocket::new_v4().unwrap();
        let bind_str: SocketAddr = format!("0.0.0.0:{}", bind_port)
            .parse::<SocketAddr>()
            .unwrap()
            .into();
        socket.set_reuseaddr(true).unwrap();
        socket.bind(bind_str).unwrap();
        let stream = socket
            .connect(
                format!("{dst_addr}:{dst_port}")
                    .parse::<SocketAddr>()
                    .unwrap()
                    .into(),
            )
            .await
            .unwrap();

        P2PTalker {
            stream,
            queue: Vec::new(),
            is_node,
        }
    }

    pub fn add_req(&mut self, url: String) {
        self.queue.push(Url::parse(&url).unwrap());
    }

    pub async fn talk(&mut self) -> io::Result<()> {
        println!("Start Talk");
        loop {
            thread::sleep(Duration::from_millis(2000));
            println!("--Work");
            let ready = self
                .stream
                .ready(Interest::READABLE | Interest::WRITABLE)
                .await
                .unwrap();

            if ready.is_readable() {
                if !self.is_node {
                    let mut req = [0; 4096];
                    // Try to read data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match self.stream.try_read(&mut req) {
                        Ok(n) => {
                            println!("Recieved data {:?} {:?}", req, n);
                            //req.truncate(n);

                            continue;
                        }
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            }

            if ready.is_writable() {
                if self.is_node {
                    let url = self.queue.last().unwrap().clone();
                    let data = reqwest::get(url).await.unwrap().bytes().await.unwrap();

                    match self
                        .stream
                        .try_write("Elisey ne viebal motuznuyu\n".as_bytes())
                    {
                        Ok(_) => {
                            println!("--Write");
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            }
        }
    }
}
