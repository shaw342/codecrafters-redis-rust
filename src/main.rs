use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

fn parse_command(buf: &[u8]) -> String {
    let data = String::from_utf8_lossy(buf);
    return data
        .split("\r\n")
        .filter(|s| !s.starts_with("*") && !s.starts_with("$") && !s.is_empty())
        .collect();
}

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
                        let cmd = parse_command(&mut buffer);
                        match cmd {
                            cmd => {
                                if cmd == "ECHO" {
                                    eprintln!("PONG");
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
        let str = parse_command(b"*1\r\n$4\r\nPING\r\n");

        assert_eq!(
            "PING", str,
            "we are testing addition with {} and {}",
            "PING", str
        );
    }
}
