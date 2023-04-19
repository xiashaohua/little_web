use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::split_once_line;

#[derive(Serialize, Deserialize, Debug)]
pub struct Path {
    pub url:String,
    params:HashMap<String,String>,
    params_string:String,
}

impl  Path{
    pub fn new(uri: String) -> Path {
        let mut words = uri.split_once("?");
        let mut params: HashMap<String, String> = HashMap::new();
        let mut params_string = serde_json::to_string(&params).unwrap();
        match words {
            Some((url, paramiters)) => {
                if paramiters.len() > 0 {

                    let prs = paramiters.split("&");
                    let vec:Vec<_> = prs.collect();
                    for s in vec {
                        let (key, val) = split_once_line(s, "=");
                        if key.len() > 0 {
                            params.insert(key, val);
                        }
                    }
                }
                
                match serde_json::to_string(&params) {
                    Ok(s) => {
                        params_string = s
                    }
                    Err(_) => {}
                }
                return Path{url:url.to_owned(), params, params_string}
            }
            None => {
                return Path{url:uri, params, params_string};
            }
        }
    }
    
}