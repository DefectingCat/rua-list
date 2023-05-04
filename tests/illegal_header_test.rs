use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::test]
async fn accept_illegal_headers() {
    illegal_headers().await;
}

async fn illegal_headers() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    let (read, mut write) = stream.split();
    let headers =
        "GET /sms.aspx HTTP/1.1\r\nHost: localhost:3000\r\nAccept: */*\r\nUser-Agent: curl/7.87.0\r\nthis is illgeal headers\r\n\r\n";
    println!("{headers}");
    write.write_all(headers.as_bytes()).await.unwrap();

    let mut buf = BufReader::new(read);
    let mut res = String::new();
    loop {
        let count = buf.read_line(&mut res).await.unwrap();
        if count < 3 {
            break;
        }
    }
    println!("{}", res);
}