use std::fmt;

pub struct Args<'a> {
    data: Vec<&'a str>,
}

impl Args<'_> {
    pub fn new<'a>() -> Args<'a> {
        Args { data: vec![] }
    }
    #[allow(dead_code)]
    pub fn from_vec<'a>(data: Vec<&'a str>) -> Args<'a> {
        Args { data }
    }
    pub fn from_str<'a>(data: &'a str) -> Args<'a> {
        Args { data: vec![data] }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_args() {
        let args = Args::from_vec(vec!["a", "b", "c"]);
        assert_eq!(args.to_string(), "a b c");
    }
}
