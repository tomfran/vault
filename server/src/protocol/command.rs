#[derive(PartialEq, Debug)]
pub enum Command {
    Ping,
    Set(String, String),
    Get(String),
}
