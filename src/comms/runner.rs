use crate::{internal_types::Args, Status};
use std::str::FromStr;

pub type RawStatus = String;

const COMMUNICATION_END_LINE: &str = ".";

pub fn send_command(command: &str, args: Args, payload: &str) -> (Status, Vec<String>) {
    send_command_stream(std::io::stdout().lock(), command, args, payload);
    let (status, payload) = read_runner();
    (Status::from_str(&status).unwrap_or(Status::Error), payload)
}

fn send_command_stream<W>(mut output: W, command: &str, args: Args, payload: &str)
where
    W: std::io::Write,
{
    if args.is_empty() {
        writeln!(output, "{command}").unwrap();
    } else {
        writeln!(output, "{command} {args}").unwrap();
    }
    writeln!(output, "{payload}").unwrap();
    writeln!(output, "{COMMUNICATION_END_LINE}").unwrap();
    output.flush().unwrap();
}

pub fn read_runner() -> (RawStatus, Vec<String>) {
    read_runner_stream(std::io::stdin().lock())
}

pub fn read_runner_stream<R>(mut input: R) -> (RawStatus, Vec<String>)
where
    R: std::io::BufRead,
{
    let mut data = vec![];
    let mut status = String::new();

    input.read_line(&mut status).unwrap();
    let status = status.trim().to_string();

    loop {
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        if line.trim() == COMMUNICATION_END_LINE {
            break;
        }
        data.push(line.trim().to_string());
    }
    (status, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_runner() {
        let input = "OK\na\nb\nc\n.\n";
        let mut reader = std::io::BufReader::new(input.as_bytes());
        let (status, data) = read_runner_stream(&mut reader);
        assert_eq!(status, "OK");
        assert_eq!(data, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_multiple_read_runner() {
        let input = "OK\na\nb\nc\n.\nOK\nd\ne\n.\n";
        let mut reader = std::io::BufReader::new(input.as_bytes());
        let (status, data) = read_runner_stream(&mut reader);
        assert_eq!(status, "OK");
        assert_eq!(data, vec!["a", "b", "c"]);
        let (status, data) = read_runner_stream(&mut reader);
        assert_eq!(status, "OK");
        assert_eq!(data, vec!["d", "e"]);
    }

    #[test]
    fn test_send_command() {
        let mut output = vec![];
        send_command_stream(
            &mut output,
            "COMMAND",
            Args::from_vec(vec!["a", "b"]),
            "payload",
        );
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "COMMAND a b\npayload\n.\n");
    }

    #[test]
    fn test_send_command_empty_args() {
        let mut output = vec![];
        send_command_stream(&mut output, "COMMAND", Args::new(), "payload");
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "COMMAND\npayload\n.\n");
    }
}
