pub mod types;
pub mod wgp;

use std::collections::HashMap;
use pingora::http::{Method, StatusCode};
use crate::router::types::{ContextTrait, HandleMessage, Response};

pub struct Router<T> {
    handler: T,
    _groups: Vec<String>, // placeholder for later use
    posts: HashMap<String, Box<[HandleMessage<T>]>>,
    gets: HashMap<String, Box<[HandleMessage<T>]>>,
    puts: HashMap<String, Box<[HandleMessage<T>]>>,
    deletes: HashMap<String, Box<[HandleMessage<T>]>>,
}

impl<T> Router<T> {
    pub fn new(handler: T) -> Self {
        Router {
            handler,
            _groups: Vec::new(),
            posts: HashMap::new(),
            gets: HashMap::new(),
            puts: HashMap::new(),
            deletes: HashMap::new(),
        }
    }

    pub fn contains(&self, method: &Method, path: &str) -> bool {
        match *method {
            Method::POST => self.posts.contains_key(path),
            Method::GET => self.gets.contains_key(path),
            Method::PUT => self.puts.contains_key(path),
            Method::DELETE => self.deletes.contains_key(path),
            _ => false,
        }
    }

    fn get_handlers(&self, method: &Method, path: &str) -> Option<&Box<[HandleMessage<T>]>> {
        match *method {
            Method::POST => self.posts.get(path),
            Method::GET => self.gets.get(path),
            Method::PUT => self.puts.get(path),
            Method::DELETE => self.deletes.get(path),
            _ => return None,
        }
    }

    pub fn call_handler(&self, ctx: &dyn ContextTrait) -> Response {
        let method = ctx.method();
        let path = ctx.path();
        if let Some(handlers) = self.get_handlers(&method, path) {
            let mut response = Response::new(StatusCode::OK, None);
            for handler in handlers.iter() {
                response = handler(&self.handler, ctx);
                if response.status != StatusCode::OK {
                    return response;
                }
            }

            response
        } else {
            return Response::new(StatusCode::NOT_FOUND, None);
        }
    }

    pub fn post(&mut self, path: String, handlers: Box<[HandleMessage<T>]>) {
        self.posts.insert(path, handlers);
    }

    pub fn get(&mut self, path: String, handlers: Box<[HandleMessage<T>]>) {
        self.gets.insert(path, handlers);
    }

    pub fn put(&mut self, path: String, handlers: Box<[HandleMessage<T>]>) {
        self.puts.insert(path, handlers);
    }

    pub fn delete(&mut self, path: String, handlers: Box<[HandleMessage<T>]>) {
        self.deletes.insert(path, handlers);
    }
}


