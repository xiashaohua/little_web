use std::{process::Output, future::Future, fmt::{Display, Debug}};

use parser::{HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

pub mod core;
pub mod parser;
pub mod route;
pub mod error;


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


// pub trait Handler<Input>: dyn_clone::DynClone + Send + Sync + 'static {
//     /// The returned type after the call operator is used.
//     type Output;

//     /// Performs the call operation.
//     #[must_use]
//     fn call(&self, input: Input) -> Self::Output;
// }

// impl<I, T> HandlerExt<I> for T where T: Handler<I> + ?Sized {}


// impl<F, I, Fut, O> Handler<I> for F
// where
//     I: Send + 'static,
//     F: Fn(I) -> Fut + ?Sized + Clone + Send + Sync + 'static,
//     Fut: Future<Output = O> + Send,
// {
//     type Output = Fut::Output;

//     fn call(&self, i: I) -> Self::Output {
//         (self)(i)
//     }
// }

// auto trait Handler {

// }

// trait NewTrait: FnOnce(HttpRequest) -> HttpResponse + Clone {}

pub trait Handler<Input>: dyn_clone::DynClone + Send + Sync + 'static {
    /// The returned type after the call operator is used.
    type Output;

    /// Performs the call operation.
    #[must_use]
    fn call(&self, input: Input) -> Self::Output;
}

impl<F, I, O> Handler<I> for F
where
    I: Send + 'static,
    F: Fn(I) -> O + ?Sized + Clone + Send + Sync + 'static,
    O: Send + 'static
    //Fut: Future<Output = O> + Send,
{
    type Output = O;

    fn call(&self, i: I) -> Self::Output {
        (self)(i)
    }
}


pub type handler<I=HttpRequest, O = HttpResponse>= Box<dyn Handler<I, Output=O>>;
//pub type handler = Box<dyn FnOnce(HttpRequest) -> HttpResponse + Send  + 'static>;

impl Clone for handler {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }

}

impl Display for handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"let me see")
    }
}

impl Debug for handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"let me see")
    }
}


pub type handlers = Vec<handler>;

