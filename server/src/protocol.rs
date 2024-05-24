// Protocol definition:
//
// Common Header = | 1b version | 1b instruction |
//
// Ping ->  Header
// Get  ->  Header | 4 bytes string len | String 1 |
// Set  ->  Header | 4 bytes string len | String 1 | 4 bytes string len | String 2 |

use log::{debug, warn};

#[derive(PartialEq, Debug)]
pub enum Command {
    Ping,
    Set(String, String),
    Get(String),
}

pub fn parse_command(payload: &[u8]) -> Option<Command> {
    if payload.len() < 2 {
        return None;
    }

    let version = payload[0];
    let instruction = payload[1];

    if version != 0 {
        warn!("Attempted to parse unknown version {}", version);
        return None;
    };

    match instruction {
        1 => {
            debug!("Parsing Ping command");
            Some(Command::Ping)
        }
        2 => {
            debug!("Parsing Get command");
            read_variable_bytes_string(&payload[2..]).map(|(_, s)| Command::Get(s))
        }
        3 => {
            debug!("Parsing Set command");
            let (l1, s1) = read_variable_bytes_string(&payload[2..])?;
            let (_, s2) = read_variable_bytes_string(&payload[(2 + l1)..])?;

            Some(Command::Set(s1, s2))
        }
        _ => {
            warn!("Attempted to parse unknown instruction {}", instruction);
            None
        }
    }
}

pub fn string_to_variable_bytes(payload: String) -> Vec<u8> {
    let string_bytes = payload.as_bytes();
    let len_bytes = (string_bytes.len() as u32).to_be_bytes();

    [string_bytes, &len_bytes].concat()
}

fn read_variable_bytes_string(payload: &[u8]) -> Option<(usize, String)> {
    if payload.len() < 4 {
        return None;
    }

    let len = u32::from_be_bytes(payload[0..4].try_into().unwrap()) as usize;
    let res = String::from_utf8_lossy(&payload[4..(4 + len)]).to_string();

    Some((4 + len, res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ping() {
        let ping_bytes = [0, 1];

        assert_eq!(parse_command(&ping_bytes).unwrap(), Command::Ping);
    }

    #[test]
    fn parse_get() {
        let expected = "hello".to_string();

        let header = [0, 2];
        let string_bytes = expected.as_bytes();
        let len_bytes = (string_bytes.len() as u32).to_be_bytes();

        let get_bytes = [&header[..], &len_bytes, string_bytes].concat();

        assert_eq!(parse_command(&get_bytes).unwrap(), Command::Get(expected));
    }

    #[test]
    fn parse_set() {
        let first = "hello".to_string();
        let second = "world".to_string();

        let header = [0, 3];
        let string_bytes = first.as_bytes();
        let len_bytes = (string_bytes.len() as u32).to_be_bytes();

        let set_bytes = [&header[..], &len_bytes, string_bytes].concat();

        let string_bytes = second.as_bytes();
        let len_bytes = (string_bytes.len() as u32).to_be_bytes();

        let set_bytes = [&set_bytes[..], &len_bytes, string_bytes].concat();

        assert_eq!(
            parse_command(&set_bytes).unwrap(),
            Command::Set(first, second)
        );
    }
}
