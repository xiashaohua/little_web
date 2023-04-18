mod router;
pub use self::router::{Node, MethodTree,MethodTrees,RouteInfo};


pub trait Controller {
    fn register(&self) -> Vec<RouteInfo>;
}

