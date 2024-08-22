use crate::types::Args;

pub type RawStatus = String;

const DOT: &str = ".";

pub fn send_command(command: &str, args: Args, payload: &str) {
    if args.is_empty() {
        println!("{command}");
    } else {
        println!("{command} {args}");
    }
    println!("{payload}");
    println!("{DOT}");
}

pub fn read_runner() -> (RawStatus, Vec<String>) {
    let mut data = vec![];
    let mut status = String::new();

    std::io::stdin().read_line(&mut status).unwrap();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim() == DOT {
            break;
        }
        data.push(line.trim().to_string());
    }
    (status, data)
}
