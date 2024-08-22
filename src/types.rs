use std::fmt;

pub type PlayerInfo<'a> = &'a str;

pub struct Args<'a> {
    data: Vec<&'a str>,
}

impl Args<'_> {
    pub fn new<'a>(data: Vec<&'a str>) -> Args<'a> {
        Args { data }
    }
    pub fn new_name<'a>(name: PlayerInfo<'a>) -> Args<'a> {
        Args { data: vec![name] }
    }
    pub fn new_empty<'a>() -> Args<'a> {
        Args { data: vec![] }
    }
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}

impl<'a> fmt::Display for Args<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for (i, &s) in self.data.iter().enumerate() {
            if i != 0 {
                res += " ";
            }
            res += s;
        }
        write!(f, "{res}")
    }
}

pub enum Status {
    Ok,
    Err,
    Died,
}

impl Status {
    pub fn from_string(s: &String) -> Status {
        match s.as_str() {
            "OK" => Status::Ok,
            "DIED" => Status::Died,
            _ => Status::Err,
        }
    }
}
