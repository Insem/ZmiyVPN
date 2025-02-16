use std::io::{self, ErrorKind};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::io::Interest;
use tokio::net::{TcpSocket, TcpStream};

pub struct P2PTalker {
    stream: TcpStream,
}
impl P2PTalker {
    pub async fn new(dst_addr: String, dst_port: usize, bind_port: usize) -> Self {
        let socket = TcpSocket::new_v4().unwrap();
        let bind_str: SocketAddr = format!("0.0.0.0:{}", bind_port)
            .parse::<SocketAddr>()
            .unwrap()
            .into();
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

        P2PTalker { stream }
    }

    pub async fn talk(&mut self, msg: Arc<RwLock<String>>) -> io::Result<()> {
        loop {
            let ready = self
                .stream
                .ready(Interest::READABLE | Interest::WRITABLE)
                .await
                .unwrap();

            if ready.is_readable() {
                let mut data = vec![0; 1024];
                // Try to read data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                match self.stream.try_read(&mut data) {
                    Ok(_n) => {
                        println!("Собеседник: {}", String::from_utf8_lossy(data.as_slice()));
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }

            if ready.is_writable() {
                // Try to write data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                // let mut msg = "xuy".to_string();
                let msg = match msg.try_read() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                match self.stream.try_write(msg.as_bytes()) {
                    Ok(n) => {
                        println!("write {} bytes", n);
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
