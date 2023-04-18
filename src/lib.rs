use serde::{Serialize, Deserialize};

pub mod core;
pub mod parser;
pub mod route;


#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub enum Method {
    POST,
    GET,
    OPTION,
    HEAD,
    PUT,
    DELETE
}

impl Copy for Method {
    
}

impl Clone for Method {

    fn clone(&self) -> Self {
        match self {
            Self::POST => Self::POST,
            Self::GET => Self::GET,
            Self::OPTION => Self::OPTION,
            Self::HEAD => Self::HEAD,
            Self::PUT => Self::PUT,
            Self::DELETE => Self::DELETE,
        }
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "POST" => Method::POST,
            "GET" => Method::GET,
            _ => Method::PUT
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Protocal {
    HTTP,
    HTTPS,
}

impl From<&str> for Protocal {
    fn from(s: &str) -> Self {
        match s {
            "HTTP" => Protocal::HTTP,
            "HTTPS" => Protocal::HTTPS,
            _ => Protocal::HTTP,
        }
    }
}

impl From<String> for Protocal {
    fn from(s: String) -> Self {
        if s == String::from("HTTP"){
            return Protocal::HTTP;
        }else if  s == String::from("HTTPS"){
            return Protocal::HTTPS;
        }else{
            return Protocal::HTTPS;
        }
        
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Version {
    V1_1,
    UNKONE,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "1.1" => Version::V1_1,
            _ => Version::UNKONE
        }
    }
}

impl From<String> for Version {
    fn from(s: String) -> Self {
        if s == String::from("1.1") {
            return Version::V1_1;
        }else{
            Version::UNKONE
        }
    }
}

pub type handler= Box<dyn FnOnce() + Send + 'static>;


pub type handlers = Vec<handler>;
