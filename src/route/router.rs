use std::borrow::BorrowMut;

use serde::{Serialize, Deserialize};

use crate::{Method, handlers,handler};



pub struct Node {
    path:String,
    handlers: handlers,
    //children: Vec<Box<node>>
}

impl Node {
    pub fn new(path:String, handlers: handlers) -> Node {
        return Node { path: path, handlers: handlers }
    }
}


pub struct MethodTree {
    method: Method,
    nodes: Vec<Node>,
}

impl MethodTree {
    pub fn new(method: Method) -> MethodTree {
        let nodes = vec![];
        return MethodTree { method: method, nodes: nodes }
    }


    pub fn addRoute(&mut self, path: String, handlers: handlers) {
        let mut start = 0;
        let end = path.len();
        if end == 0 {
            panic!("can`t add a empty path")
        }
        for s in path.chars() {
            if start == 0 && s != '/' {
                panic!("path {} is not start with /", path)
            }
            if s != '/' {
                if s == ':' || s == '*' {
                    panic!("path {} is invalid", path)
                }
            }
            start += 1;
        }
        
        let path = path.trim_end_matches("/");

        for node in &self.nodes {
            if node.path == path {
                panic!("cant use same path with same method")
            }
        }
        self.nodes.insert(0, Node::new(path.to_owned(), handlers));

        //self.nodes.append())
    }

    
}


pub struct MethodTrees{
    data: Vec<MethodTree>
}

impl MethodTrees {
    
    pub fn new() -> MethodTrees {
        return MethodTrees{data:vec![]};
    }


    pub fn addRoute(&mut self,method:Method, path: String, handlers: handlers) {
        let end = self.data.len();
        let mut index = end + 1;
        for i in 0..end {
            let a:&MethodTree = self.data.get(i).unwrap();
            if a.method == method {
                index = i;
                break;
            }
        }

        if index > end {
            let mut tree = MethodTree::new(method);
            tree.addRoute(path, handlers);
            self.data.push(tree);

        }else{
            let tree:&mut MethodTree = self.data.get_mut(index).unwrap();
            tree.addRoute(path, handlers);
        }

    }

}



pub  struct RouteInfo {
	pub method:      Method,
	pub path:        String,
	pub handlerFunc: handler,
}

