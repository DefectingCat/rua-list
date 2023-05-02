use std::{net::SocketAddr, process::exit};

use log::error;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{tcp::ReadHalf, TcpListener, TcpStream},
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
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = BufReader::new(stream);
            let header = read_to_end(&mut buf).await;
            let body = read_to_end(&mut buf).await;
            let header: Vec<_> = header.split("\r\n").collect();
            let first_line = header.first().unwrap();
            let header = &header[1..header.len() - 2];
            let header: Vec<_> = header
                .iter()
                .filter(|head| head.contains(':'))
                .map(|head| head.to_string())
                .collect();
            let headers = format!("{first_line}\r\n{}\r\n\r\n", header.join(""));
            dbg!(&headers, &body);
        });
    }
}

async fn read_to_end(buf: &mut BufReader<TcpStream>) -> String {
    let mut target = String::new();
    loop {
        let count = buf.read_line(&mut target).await.unwrap();
        if count < 3 {
            return target;
        }
    }
}