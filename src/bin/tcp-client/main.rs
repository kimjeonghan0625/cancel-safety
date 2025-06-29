use std::time::Duration;

use cancel_safety::Foo;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let mut file = File::create("foo.txt").await.unwrap();
    let foo = Foo::new("hello cancelation safety!");
    file.write_all(&serde_json::to_vec(&foo).unwrap())
        .await
        .unwrap();

    let mut file = File::open("foo.txt").await.unwrap();

    loop {
        tokio::select! {
            data = read_foo_from_file(&mut file) => {
                send_foo_to_stream(&mut stream, &data).await;
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(2)) => {
                println!("timeout occured")
            }
        }
    }
}

async fn read_foo_from_file(f: &mut File) -> String {
    let mut string = String::new();
    loop {
        let mut buf = [0u8; 8];
        let n = f.read(&mut buf).await.unwrap();
        string.push_str(str::from_utf8(&buf[..n]).unwrap());
        if n == 0 {
            return string;
        }
        println!("{string}");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn send_foo_to_stream(stream: &mut TcpStream, data: &str) {
    stream.write_all(data.as_bytes()).await.unwrap();
}
