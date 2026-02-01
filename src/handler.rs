use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::{parser, command::Command};

pub async fn handle_client(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    let mut line = String::new();

    loop {
        line.clear();

        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            // client closed connection
            break;
        }

        let cmd = parser::parse_command(&line);

        match cmd {
            Command::Ping => {
                writer.write_all(b"PONG\n").await?;
            }
            Command::Echo(msg) => {
                writer.write_all(msg.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
            Command::Quit => {
                writer.write_all(b"BYE\n").await?;
                break;
            }
            Command::Unknown(name) => {
                writer
                    .write_all(format!("ERR unknown command: {}\n", name).as_bytes())
                    .await?;
            }
        }
    }

    Ok(())
}
