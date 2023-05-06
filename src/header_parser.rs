use anyhow::Result;
use log::{debug, error, info};
use std::{net::SocketAddr, process::exit};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot},
};

type Responder = oneshot::Sender<String>;
#[derive(Debug)]
struct Frame {
    request: String,
    responder: Responder,
}

/// Parse request headers.
///
/// Delete all illegal headers, the reqeust to localhost:port + 1
/// Also forward response from server to client
pub async fn headers_parser(port: usize) {
    let addr: SocketAddr = match format!("0.0.0.0:{:?}", port).parse() {
        Ok(addr) => addr,
        Err(err) => {
            error!("Failed to parse address {}", err);
            exit(1);
        }
    };
    let listener = TcpListener::bind(addr).await.expect("Can not start server");
    info!("Server listening on {}", &addr);

    let (tx, mut rx) = mpsc::channel::<Frame>(128);

    tokio::spawn(async move {
        while let Some(frame) = rx.recv().await {
            let mut connector = match TcpStream::connect("127.0.0.1:3001").await {
                Ok(stream) => stream,
                Err(err) => {
                    error!("Can not request to server {}", err);
                    break;
                }
            };
            // let connector = connector.clone();
            // let mut connector = connector.lock().await;
            let (reader, mut writer) = connector.split();
            // Forward all request without illegal headers
            if let Err(err) = writer.write_all(frame.request.as_bytes()).await {
                error!("Can not write to server {}", err);
                break;
            }
            let mut reader = BufReader::new(reader);
            let mut res_header = String::new();
            loop {
                let count = reader.read_line(&mut res_header).await.unwrap();
                if count < 3 {
                    break;
                }
            }
            let mut res_len: usize = 0;
            let res_headers: Vec<_> = res_header.split("\r\n").collect();
            let res_headers: Vec<_> = res_headers
                .iter()
                .map(|h| {
                    if h.to_lowercase().starts_with("content-length") {
                        let content: Vec<_> = h.split(':').collect();
                        res_len = content[1].trim().parse().unwrap();
                    }
                    h
                })
                .collect();
            debug!("{res_headers:?}");
            let response = if res_len > 0 {
                let mut body = vec![0; res_len];
                if let Err(err) = reader.read_exact(&mut body).await {
                    error!("Can not read response body {}", err)
                }
                format!("{res_header}{}", String::from_utf8_lossy(&body))
            } else {
                res_header
            };
            frame.responder.send(response).unwrap();
        }
    });

    loop {
        let tx = tx.clone();
        let (mut stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = BufReader::new(&mut stream);
            let header = match read_to_end(&mut buf).await {
                Ok(c) => c,
                Err(err) => {
                    error!("Failed to read headers {}", err);
                    return;
                }
            };
            let header: Vec<_> = header.split("\r\n").collect();
            let first_line = header.first().unwrap();
            let header = &header[1..header.len() - 2];

            let mut content_len: usize = 0;
            // Remove all illegal headers
            let header: Vec<_> = header
                .iter()
                .filter(|head| head.contains(':') && !head.contains(';'))
                .map(|head| head.to_string())
                .map(|h| {
                    if h.to_lowercase().starts_with("content-length") {
                        let content: Vec<_> = h.split(':').collect();
                        content_len = content[1].trim().parse().unwrap();
                    }
                    h
                })
                .collect();
            debug!("{:?}", &header);
            let headers = format!("{first_line}\r\n{}\r\n\r\n", header.join("\r\n"));
            // If has content-length, read request body
            let request = if content_len > 0 {
                let mut body = vec![0; content_len];
                if let Err(err) = buf.read_exact(&mut body).await {
                    error!("Can not read request body {}", err)
                }
                format!("{headers}{}", String::from_utf8_lossy(&body))
            } else {
                headers
            };

            let (res_tx, rx) = oneshot::channel();
            if let Err(err) = tx
                .send(Frame {
                    request,
                    responder: res_tx,
                })
                .await
            {
                error!("Can not send frame with mpsc {}", err);
                return;
            }

            let response = match rx.await {
                Ok(res) => res,
                Err(err) => {
                    error!("Failed to receive response {}", err);
                    return;
                }
            };
            if let Err(err) = stream.write_all(response.as_bytes()).await {
                error!("Failed to write reponse to client {}", err);
            }
        });
    }
}

async fn read_to_end(buf: &mut BufReader<&mut TcpStream>) -> Result<String> {
    let mut target = String::new();
    loop {
        let count = buf.read_line(&mut target).await?;
        if count < 3 {
            break;
        }
    }
    Ok(target)
}