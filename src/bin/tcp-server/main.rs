use std::sync::{Arc, Mutex};

use cancel_safety::Foo;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // create tcp listener
    let listen = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let foo_store = Arc::new(Mutex::new(Vec::<Foo>::new()));

    loop {
        let (stream, _) = listen.accept().await.unwrap();
        let f = foo_store.clone();
        tokio::spawn(async move {
            let buf = BufReader::new(stream);
            let mut lines = buf.lines();
            while let Ok(Some(string)) = lines.next_line().await {
                let foo = serde_json::from_str::<Foo>(&string);
                match foo {
                    Err(err) => {
                        eprintln!("{string}를 파싱하는 데 실패했습니다: {err:?}\n");
                    }
                    Ok(d) => {
                        let mut g = f.lock().unwrap();
                        g.push(d);
                        println!("{:?}", *g);
                    }
                }
            }
        });
    }
}
