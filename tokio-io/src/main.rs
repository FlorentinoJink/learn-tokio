// // runtime 写法
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;
// use tokio::runtime::Builder;
// fn main() {
//     let rt = Builder::new_current_thread().build().unwrap();
//     rt.block_on(async {
//         let mut f = File::open("foo.txt").await.unwrap();
//         let mut buffer = [0; 10];

//         let n = f.read(&mut buffer[..]).await.unwrap();

//         println!("The bytes: {:?}", &buffer[..n]);
//     });
// }


// #[tokio::main] 写法
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await.unwrap();
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await.unwrap();

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
