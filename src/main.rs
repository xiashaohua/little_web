use std::{io::{self, Read, BufReader, BufRead, Write}, cmp::Ordering, net::{TcpListener, TcpStream}, fs, thread::Thread};
use little_web::{core::{processer, App}, route::{Controller, RouteInfo}, parser::{HttpRequest, HttpResponse}};
use rand::{self, Rng};
use std::{cmp, str};
use little_web::Method;

struct webController {

}

impl webController {
    fn new()->webController{
        return webController {  }
    }
    
}


impl Controller for webController {
    fn register(&self) -> Vec<RouteInfo> {
        return vec![
            RouteInfo::new(Method::GET, "/cc", index)
        ];
    }
}

pub fn index(r:HttpRequest)->HttpResponse {
    println!("123");
    return HttpResponse::new("i am the test".to_owned());
    
}


fn main() {
    let mut app = App::new();
    let c = webController::new();
    app.register(c);
    app.start();
}

























