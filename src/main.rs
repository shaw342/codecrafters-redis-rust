use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener; // Import nécessaire pour .read() et .write_all()

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        // 1. On accepte la connexion de manière asynchrone
        let (mut socket, _) = listener.accept().await?;

        // 2. On spawn une tâche pour ce client
        tokio::spawn(async move {
            let mut buffer = [0; 512];

            loop {
                // 3. On lit de manière asynchrone
                match socket.read(&mut buffer).await {
                    Ok(0) => return, // Connexion fermée par le client
                    Ok(_) => {
                        // 4. On écrit de manière asynchrone
                        if let Err(e) = socket.write_all(b"+PONG\r\n").await {
                            eprintln!("Erreur lors de l'écriture : {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Erreur lors de la lecture : {}", e);
                        return;
                    }
                }
            }
        });
    }
}
