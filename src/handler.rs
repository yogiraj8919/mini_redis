use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::{command::Command, parser, store::Store};

pub async fn handle_client(socket: TcpStream,store:Store) -> Result<(), Box<dyn std::error::Error>> {
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
            Command::Set { key, value } =>{
                store.set(key, value).await;
                writer.write_all(b"OK\n").await?;
            }
            Command::Get { key } =>{
                if let Some(val) = store.get(&key).await{
                    writer.write_all(val.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }else{
                    writer.write_all(b"{nil}\n").await?;
                }
            }
            Command::Del { key } => {
                let deleted = store.del(&key).await;
                writer.write_all(format!("{}\n",deleted as u8).as_bytes()).await?;
            }
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
