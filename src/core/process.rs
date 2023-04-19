pub mod processer{
    use std::{io::{self, Read, BufReader, BufRead, Write}, cmp::Ordering, net::{TcpListener, TcpStream}, fs, thread::Thread, sync::{Mutex, Arc}};
    use std::{cmp, str};

    use crate::{parser::{HttpParser, HttpResponse}, route::MethodTrees, Method};

    pub fn hand_connection(mut stream: TcpStream, url_tree:Arc<MethodTrees>, remote_ip:String) {
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
                        println!("get_request is , url is {:#?}, remote ip is {}",http_request.header.url.url.clone(), remote_ip);
                        //let d = url_tree.clone();
                        let methodTree = url_tree.get(http_request.header.method.clone());
                        match methodTree {
                            Some(tree) => {
                                let node = tree.get(http_request.header.url.url.clone());
                                 match node {
                                    Some(r) =>{
                                        if r.handlers.len() > 0 {
                                            let data = r.handlers[0].call(http_request);
                                            hand_normal_response(stream, data);
                                            println!("finish")
                                        }
                                    },
                                    None => hand_system_error(stream),
                                }

                            },
                            None => hand_system_error(stream),
                        }
                    },
                    None => {
                        hand_system_error(stream)
                    }
                }
            }
        }
        return;
        
    }
    
    
    pub fn hand_system_error(mut stream: TcpStream) {
        let (file_name, status_line) = ("src/static/500.html", "HTTP/1.1 500 System Error");
        let contents = fs::read_to_string(file_name).unwrap();
        let length = contents.len();
    
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        
    
        stream.write_all(response.as_bytes()).unwrap();
    
        
    
    }


    pub fn hand_normal_response(mut stream:TcpStream, response: HttpResponse) {
        let status_line = "HTTP/1.1 200";

        let contents = response.ss;
        let length = contents.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        //println!("{}", response);
        stream.write_all(response.as_bytes()).unwrap();
        
    }
}