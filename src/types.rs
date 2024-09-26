use std::str::FromStr;

pub enum Status {
    Ok,
    Error,
    Died,
}

impl FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Status, Self::Err> {
        match s {
            "OK" => Ok(Status::Ok),
            "DIED" => Ok(Status::Died),
            "ERROR" => Ok(Status::Error),
            _ => Err(()),
        }
    }
}
