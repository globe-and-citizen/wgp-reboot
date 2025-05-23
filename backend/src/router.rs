use std::collections::HashMap;
use pingora::http::Method;

// Box<dyn std::error::Error + Send + Sync> is used to represent any error type that implements the std::error::Error trait and can be sent across thread boundaries.
type HandleMessage<T> = fn(&T, &Vec<u8>) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>>;

pub struct Router<T> {
    handler: T,
    pub groups: Vec<String>, // placeholder for later use
    pub posts: HashMap<String, HandleMessage<T>>,
    pub gets: HashMap<String, HandleMessage<T>>,
    pub puts: HashMap<String, HandleMessage<T>>,
    pub deletes: HashMap<String, HandleMessage<T>>,
}

impl<T> Router<T> {
    pub fn new(handler: T) -> Self {
        Router {
            handler,
            groups: Vec::new(),
            posts: HashMap::new(),
            gets: HashMap::new(),
            puts: HashMap::new(),
            deletes: HashMap::new(),
        }
    }

    pub fn contains(&self, method: &Method, path: &str) -> bool {
        println!("Checking if {} exists for {}, {:?}, {}", path, method, self.posts.keys(), self.posts.contains_key(path));
        match *method {
            Method::POST => self.posts.contains_key(path),
            Method::GET => self.gets.contains_key(path),
            Method::PUT => self.puts.contains_key(path),
            Method::DELETE => self.deletes.contains_key(path),
            _ => false,
        }
    }

    fn get_handler(&self, method: &Method, path: &str) -> Option<&HandleMessage<T>> {
        match *method {
            Method::POST => self.posts.get(path),
            Method::GET => self.gets.get(path),
            Method::PUT => self.puts.get(path),
            Method::DELETE => self.deletes.get(path),
            _ => return None,
        }
    }

    pub fn call_handler(&self, method: &Method, path: &str, data: &Vec<u8>) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(handler) = self.get_handler(method, path) {
            println!("Handler found for {}: {:?}", method, handler);
            handler(&self.handler, data)
        } else {
            println!("No handler found for {}: {}", method, path);
            Ok(None)
        }
    }

    pub fn group(&mut self, group: String) {
        self.groups.push(group);
    }

    pub fn post(&mut self, path: String, handler: HandleMessage<T>) {
        self.posts.insert(path, handler);
    }

    pub fn get(&mut self, path: String, handler: HandleMessage<T>) {
        self.gets.insert(path, handler);
    }

    pub fn put(&mut self, path: String, handler: HandleMessage<T>) {
        self.puts.insert(path, handler);
    }

    pub fn delete(&mut self, path: String, handler: HandleMessage<T>) {
        self.deletes.insert(path, handler);
    }
}