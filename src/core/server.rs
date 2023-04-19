use std::{sync::{Arc, Mutex}, net::TcpListener};

use crate::{route::{MethodTrees, Controller}, core::{processer, ThreadPool}, error::AppError};

pub struct App {
    URLTree :MethodTrees

}

impl App {
     /// Creates a new application 
    ///
    /// # Examples
    ///
    /// ```
    /// use little_web::core::App;
    ///
    /// let app = App::new();
    /// ```
    pub fn new()->App {
        let method_tree = MethodTrees::new();

        let URLTree = method_tree;
        return App { URLTree: URLTree}
    }


    pub fn register<T>(&mut self, c:T )
    where T : Controller
    {
        let route_list = c.register();

        for route in route_list{
            let func_list = vec![route.handlerFunc];
            self.URLTree.addRoute(route.method, route.path, func_list);

        }
    }


    pub fn start(&mut self)-> Result<String, AppError> {
        let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
        let pool = ThreadPool::new(4);
        for stream in listener.incoming(){
            let stream = stream.unwrap();


            
            let remote_ip =  stream.peer_addr().unwrap().to_string();
            println!("Connection established!, client addr is {}", remote_ip.clone());
            let url_tree = Arc::new(self.URLTree.clone());
            pool.execute(|| {
                processer::hand_connection(stream, url_tree, remote_ip);
            });
        }
        println!("Shutting down.");
        return Ok("aaa".to_owned())
        
    }

    // pub fn hand_connection(&mut self,mut stream: TcpStream) {
    //     let mut buffer = [0u8; 2048];
    //     let n = stream.read(&mut buffer).unwrap();
       
    //     match str::from_utf8(&buffer[0..n]){
    //         Err(e) => {
    //             println!("read data from tcp stream error, error detail is {:#?}", e);
    //             processer::hand_system_error(stream);
    //         }
    //         Ok(request) => {
    //             //println!("Request :{:#?}, count is {}",request,n);
    //             let http_parser = HttpParser::new(request.to_owned());
    //             match http_parser.parse(){
    //                 Some(http_request) => {
    //                     println!("http_request is  :{:#?}",http_request);
    //                 },
    //                 None => {
    //                     processer::hand_system_error(stream)
    //                 }
    //             }
    //         }
    //     }
    //     return;
        
    // }
    
}

