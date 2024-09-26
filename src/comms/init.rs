use super::runner;
type PlayerName = String;

/**
Read config from runner.

Returns list of players and vector of additional config values split by whitespace characters.

# Panics

The `read_config` function will panic if it receives a wrong header from runner or if number of players send by runner is zero.
*/
pub fn read_config() -> (Vec<PlayerName>, Vec<String>) {
    _read_config(std::io::stdin().lock())
}

fn _read_config<R>(input: R) -> (Vec<PlayerName>, Vec<String>)
where
    R: std::io::BufRead,
{
    let (status, data) = runner::_read_runner(input);
    let mut data = data.iter();
    if status != "CONFIG" {
        panic!("Unexpected header when reading config: {status}");
    }

    let players: Vec<PlayerName> = match data.next() {
        Some(s) => s.split_whitespace().map(|s| s.to_string()).collect(),
        None => panic!("No players specified in config"),
    };

    if players.len() == 0 {
        panic!("No players specified in config");
    }

    let cfg: Vec<_> = data.map(|s| s.to_string()).collect();

    (players, cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config() {
        let input = "CONFIG\na b c\n1 0 1\n2\n3\n.\n";
        let mut reader = std::io::BufReader::new(input.as_bytes());
        let (players, cfg) = _read_config(&mut reader);
        assert_eq!(players, vec!["a", "b", "c"]);
        assert_eq!(cfg, vec!["1 0 1", "2", "3"]);
    }

    #[test]
    #[should_panic(expected = "No players specified in config")]
    fn test_read_config_no_players() {
        let input = "CONFIG\n\na\nb\nc\n.\n";
        let mut reader = std::io::BufReader::new(input.as_bytes());
        _read_config(&mut reader);
    }

    #[test]
    #[should_panic(expected = "No players specified in config")]
    fn test_read_config_no_players2() {
        let input = "CONFIG\n\n.\n";
        let mut reader = std::io::BufReader::new(input.as_bytes());
        _read_config(&mut reader);
    }

    #[test]
    #[should_panic(expected = "Unexpected header when reading config:")]
    fn test_read_config_wrong_header() {
        let input = "WRONG\na b c\n1\n2\n3\n.\n";
        let mut reader = std::io::BufReader::new(input.as_bytes());
        _read_config(&mut reader);
    }
}
