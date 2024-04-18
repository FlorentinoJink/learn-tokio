use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream){
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}