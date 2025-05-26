use std::collections::HashMap;
use pingora::http::Method;

// Box<dyn std::error::Error + Send + Sync> is used to represent any error type that implements the std::error::Error trait and can be sent across thread boundaries.
type HandleMessage<T> = fn(&T, &Vec<u8>) -> (Option<Vec<u8>>, pingora::http::StatusCode);

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

    pub fn call_handler(&self, method: &Method, path: &str, data: &Vec<u8>) -> (Option<Vec<u8>>, pingora::http::StatusCode) {
        if let Some(handlers) = self.get_handlers(method, path) {
            let mut response = None;
            let mut status = pingora::http::StatusCode::OK;
            for handler in handlers.iter() {
                (response, status) = handler(&self.handler, data);
                if status != pingora::http::StatusCode::OK {
                    return (response, status);
                }
            }

            (response, status)
        } else {
            return (None, pingora::http::StatusCode::NOT_FOUND);
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