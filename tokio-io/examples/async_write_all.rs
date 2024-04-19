use tokio::io::{self,AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("bar.txt").await?;

    file.write_all(b"some bytes all").await?;
    Ok(())  
}