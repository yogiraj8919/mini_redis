use tokio::io::join;

use crate::command::Command;

pub fn parse_command(input:&str) -> Command{
    let input = input.trim();
    if input.is_empty(){
        return Command::Unknown("empty".to_string());
    }

    let mut parts  = input.split_whitespace();

    let cmd = parts.next().unwrap().to_uppercase();

    match cmd.as_str() {
        "PING" => Command::Ping,
        "ECHO" => Command::Echo(parts.collect::<Vec<_>>().join(" ")),
        "QUIT" => Command::Quit,
        other=> Command::Unknown(other.to_string()),
    }
}