use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// type SharedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

// fn new_shared_db(num_shared: usize) -> SharedDb {
//     let mut db = Vec::with_capacity(num_shared);
//     for _ in 0..num_shared {
//         db.push(Mutex::new(HashMap::new()));
//     }
//     Arc::new(db)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await?;

        let db = db.to_owned();

        println!("Accepted");

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db){
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().to_owned());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) =>{
                let db = db.lock().unwrap();
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