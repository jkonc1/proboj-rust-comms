use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    name: String,
}

impl PlayerInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
