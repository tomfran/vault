use super::command::Command;
use log::{debug, warn};

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
            read_variable_bytes(&payload[2..]).map(|(_, s)| Command::Get(s))
        }
        3 => {
            debug!("Parsing Set command");
            let (l1, s1) = read_variable_bytes(&payload[2..])?;
            let (_, s2) = read_variable_bytes(&payload[(2 + l1)..])?;

            Some(Command::Set(s1, s2))
        }
        _ => {
            warn!("Attempted to parse unknown instruction {}", instruction);
            None
        }
    }
}

fn read_variable_bytes(payload: &[u8]) -> Option<(usize, Box<[u8]>)> {
    if payload.len() < 4 {
        return None;
    }

    let len = u32::from_be_bytes(payload[0..4].try_into().unwrap()) as usize;
    Some((4 + len, payload[4..(4 + len)].into()))
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
        let expected = "hello".as_bytes();

        let header = [0, 2];
        let len_bytes = (expected.len() as u32).to_be_bytes();
        let get_bytes = [&header[..], &len_bytes, expected].concat();

        assert_eq!(
            parse_command(&get_bytes).unwrap(),
            Command::Get(expected.into())
        );
    }

    #[test]
    fn parse_set() {
        let first = "hello".as_bytes();
        let second = "world".as_bytes();

        let header = [0, 3];
        let len_bytes = (first.len() as u32).to_be_bytes();
        let set_bytes = [&header[..], &len_bytes, first].concat();

        let len_bytes = (second.len() as u32).to_be_bytes();
        let set_bytes = [&set_bytes[..], &len_bytes, second].concat();

        assert_eq!(
            parse_command(&set_bytes).unwrap(),
            Command::Set(first.into(), second.into())
        );
    }
}
