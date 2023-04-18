use std::{io::{self, Read, BufReader, BufRead, Write}, cmp::Ordering, net::{TcpListener, TcpStream}, fs, thread::Thread};
use rand::{self, Rng};
use std::{cmp, str};
use test1::core::ThreadPool;
use test1::parser::HttpParser;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();

        println!("Connection established!, client addr is {}", stream.peer_addr().unwrap().to_string());
        pool.execute(|| {
            hand_connection(stream);
        });
    }
    println!("Shutting down.");
}


fn hand_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 2048];
    let n = stream.read(&mut buffer).unwrap();
   
    match str::from_utf8(&buffer[0..n]){
        Err(e) => {
            println!("read data from tcp stream error, error detail is {:#?}", e);
            hand_system_error(stream);
        }
        Ok(request) => {
            //println!("Request :{:#?}, count is {}",request,n);
            let http_parser = HttpParser::new(request.to_owned());
            match http_parser.parse(){
                Some(http_request) => {
                    println!("http_request is  :{:#?}",http_request);
                },
                None => {
                    hand_system_error(stream)
                }
            }
        }
    }

    
    return;
    
}


fn hand_system_error(mut stream: TcpStream) {
    let (file_name, status_line) = ("src/static/500.html", "HTTP/1.1 500 System Error");
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();

    

}






















