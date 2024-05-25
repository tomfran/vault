#[derive(PartialEq, Debug)]
pub enum Command {
    Ping,
    Set(Box<[u8]>, Box<[u8]>),
    Get(Box<[u8]>),
}
