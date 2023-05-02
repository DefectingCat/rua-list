use log::error;
use std::{net::SocketAddr, process::exit};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

pub async fn headers_parser(port: usize) {
    let addr: SocketAddr = match format!("0.0.0.0:{:?}", port).parse() {
        Ok(addr) => addr,
        Err(err) => {
            error!("Failed to parse address {}", err);
            exit(1);
        }
    };
    let listener = TcpListener::bind(addr).await.expect("Can not start server");

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = BufReader::new(&mut stream);
            let header = read_to_end(&mut buf).await;
            let header: Vec<_> = header.split("\r\n").collect();
            let first_line = header.first().unwrap();
            let header = &header[1..header.len() - 2];

            let mut content_len: usize = 0;
            let header: Vec<_> = header
                .iter()
                .filter(|head| head.contains(':'))
                .map(|head| head.to_string())
                .map(|h| {
                    if h.to_lowercase().starts_with("content-length") {
                        let content: Vec<_> = h.split(':').collect();
                        content_len = content[1].trim().parse().unwrap();
                    }
                    h
                })
                .collect();
            let headers = format!("{first_line}\r\n{}\r\n\r\n", header.join(""));

            let request = if content_len > 0 {
                dbg!(&content_len);
                let mut body = vec![0; content_len];
                if let Err(err) = buf.read_exact(&mut body).await {
                    error!("Can not read body {}", err)
                }
                format!("{headers}{}", String::from_utf8_lossy(&body))
            } else {
                headers
            };
            dbg!(&request);
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await.unwrap();
        });
    }
}

async fn read_to_end(buf: &mut BufReader<&mut TcpStream>) -> String {
    let mut target = String::new();
    loop {
        let count = buf.read_line(&mut target).await.unwrap();
        if count < 3 {
            break;
        }
    }
    target
}