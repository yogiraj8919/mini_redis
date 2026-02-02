



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
        "SET" => {
            let key = match parts.next() {
                Some(k) => k.to_string(),
                None => return Command::Unknown("SET missing key".to_string()),
            };
            let value = parts.collect::<Vec<_>>().join(" ");
            Command::Set { key, value }
        },
        "GET" => {
            let key:String = match parts.next() {
                Some(k) => k.to_string(),
                None => return Command::Unknown("GET missing key".to_string())
            };
            Command::Get { key }
        }
        "DEL" => {
            let key:String = match parts.next() {
                Some(k) => k.to_string(),
                None => return Command::Unknown("DEL missing key".to_string())
            };
            Command::Del { key } 
        }
        "ECHO" => Command::Echo(parts.collect::<Vec<_>>().join(" ")),
        "QUIT" => Command::Quit,
        other=> Command::Unknown(other.to_string()),
    }
}