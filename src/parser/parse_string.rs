use crate::{Method, Protocal, Version};



pub fn split_once_line(s: &str, delimiter: &str ) -> (String, String) {
    let  words = s.split_once(delimiter);
    match words {
        Some((key, val)) => {
            return (key.trim().to_string(), val.trim().to_string());
        }
        None => {
            return ("".to_string(), "".to_string())
        }
    }
}

pub fn parse_meta_data(s:&str) -> (Method, String, Protocal,Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let path =  words.next().unwrap();
    let version = words.next().unwrap();
    let (protocal, version) = split_once_line(version, "/");

    return (method.into(), path.to_string(),protocal.into(), version.into());
}