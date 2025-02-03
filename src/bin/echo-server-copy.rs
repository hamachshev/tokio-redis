use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6739").await?;

    while let (mut socket, _) = listener.accept().await? {
        tokio::spawn(async move {
            let mut buf = vec![0; 128];
            //if io::copy(&mut rd, &mut wr).await.is_err() {
            //    eprintln!("Failed to copy");
            //};
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&mut buf).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
    Ok(())
}
