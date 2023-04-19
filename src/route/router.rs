use std::borrow::BorrowMut;
use std::fmt::Display;

use serde::{Serialize, Deserialize};

use crate::{Method, handlers,handler, parser::{HttpRequest, HttpResponse}, Handler};

#[derive(Debug)]
pub struct Node {
    pub path:String,
    pub handlers: handlers,
    //children: Vec<Box<node>>
}

impl Node {
    pub fn new(path:String, handlers: handlers) -> Node {
        return Node { path: path, handlers: handlers }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        //let path = self.path.clone();
        Self { path: self.path.clone(), handlers: self.handlers.clone() }
    }
}


#[derive(Debug)]
pub struct MethodTree {
    method: Method,
    nodes: Vec<Node>,
}

impl Clone for MethodTree {
    fn clone(&self) -> Self {
        Self { method: self.method.clone(), nodes: self.nodes.clone() }
    }
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

    pub fn get<'a>(&'a self,path:String) ->Option<&'a Node> {
        for node in &self.nodes {
            if node.path == path {
                return Some(node);
            }
        }
        return None
    }

    
}


#[derive(Debug)]
pub struct MethodTrees{
    data: Vec<MethodTree>
}

impl Clone for MethodTrees {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
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

    pub fn get<'a>(&'a self,method:Method) ->Option<&'a MethodTree> {
        for tree in &self.data {
            if tree.method == method {
                //let cc = tree.clone();
                return Some(tree);
            }
        }
        return None
    }

}



pub  struct RouteInfo {
	pub method:      Method,
	pub path:        String,
	pub handlerFunc: handler,
}

impl RouteInfo {
    pub fn new<T>(method: Method, path: &str, handlerFunc: T)->RouteInfo
    where T:  Handler<HttpRequest, Output = HttpResponse>
    {
        let handlerFunc = Box::new(handlerFunc);
        let path = path.to_owned();
        return RouteInfo{method,path, handlerFunc};
    }
    
}

