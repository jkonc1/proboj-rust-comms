use super::runner;
type PlayerName = String;

pub fn read_config() -> (Vec<PlayerName>, Vec<String>) {
    let (status, data) = runner::read_runner();
    if status != "CONFIG" {
        panic!("Unexpected status: {status}");
    }

    let players: Vec<PlayerName> = data[0].split_whitespace().map(|s| s.to_string()).collect();
    let mut cfg: Vec<String> = vec![];

    for i in data[1..].iter() {
        cfg.push(i.to_string());
    }

    (players, cfg)
}
