use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::test]
async fn accept_illegal_headers() {
    illegal_headers().await;
}

async fn illegal_headers() {
    let mut stream = TcpStream::connect("localhost:3000").await.unwrap();
    let (read, mut write) = stream.split();
    let headers = r#"GET /sms.aspx HTTP/1.1
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Cache-Control: max-age=0

"#;
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