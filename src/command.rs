// Command module for mini_redis
#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set {key:String, value:String},
    Get {key:String},
    Del {key:String},
    Quit,
    Unknown(String)
}