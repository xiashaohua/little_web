use std::sync::{Arc, Mutex};

use crate::route::{MethodTrees, Controller};

pub struct App {
    URLTree :Arc<Mutex<MethodTrees>>

}

impl App {
    pub fn new()->App {
        let method_tree = MethodTrees::new();

        let URLTree = Arc::new(Mutex::new(method_tree));
        return App { URLTree: URLTree}
    }


    pub fn register<T>(&mut self, c:T )
    where T : Controller
    {
        let route_list = c.register();

        for route in route_list{
            let func_list = vec![route.handlerFunc];
            self.URLTree.lock().unwrap().addRoute(route.method, route.path, func_list);

        }
    }


    // pub fn start()-> Result<(ok)> {
        
    // }
    
}

