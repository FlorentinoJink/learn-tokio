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
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) =>{
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.to_owned().into())
                } else {
                    Frame::Null
                }
            }
            cmd=> panic!("unimplelemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}