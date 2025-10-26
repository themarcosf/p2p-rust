use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

async fn process(socket: TcpStream) {
    let mut conn = Connection::new(socket);

    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        conn.write_frame(&response).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let handle = tokio::spawn(async move {
            // process each socket concurrently
            process(socket).await;
        });

        let out = handle.await;
        if out.is_err() {
            eprintln!("task failed: {:?}", out.err());
        }
    }
}
