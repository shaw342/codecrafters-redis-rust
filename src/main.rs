use bytes::{BytesMut, buf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0; 512];

            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => return,
                    Ok(_n) => {
                        let data = String::from_utf8_lossy(&buffer);

                        if data.contains("ECHO") {
                            let result: Vec<&str> = data.split("\r\n").collect();

                            if let Some(arg) = result.get(4) {
                                let response = format!("${}\r\n{}\r\n", arg.len(), arg);
                                if let Err(e) = socket.write_all(response.as_bytes()).await {
                                    eprintln!("Erreur lors de l'écriture : {}", e);
                                    return;
                                }
                            }
                        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn feature_parse_command() {
        let str = parse_command(b"*2\r\n$4\r\nECHO\r\n$5\r\nmango\r\n");

        assert_eq!(Input::Command("ECHO"), str);
    }
}
