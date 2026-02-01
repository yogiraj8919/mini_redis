// Command module for mini_redis
#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Quit,
    Unknown(String)
}