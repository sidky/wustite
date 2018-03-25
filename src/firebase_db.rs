extern crate futures;
extern crate hyper;
extern crate tokio_core;

use firebase_ref::Ref;

use std::io::{self, Write};

use self::futures::{Future, Stream};
use self::hyper::Client;
use self::hyper::client::{Response, FutureResponse};
use self::tokio_core::reactor::Core;

pub struct FirebaseDatabase {
    database_url: String
}


impl FirebaseDatabase {
    pub fn once(name: &str) -> FutureResponse {

    }
}

trait FirebaseLoader {
    // type Response;

    fn once(self, name: &str) -> FutureResponse; 
}

impl FirebaseLoader for Ref {
    fn once(self, name: &str) -> FutureResponse {
        let mut core = Core::new();
        let client = Client::new(&core.handle());

        let path = [self.path_string(), "/", name].join();
    }
}

#[cfg(test)]
mod database_test {
    use super::*;

    // #[test]
    // fn test_path() {
    //     let reference = Ref::new("users").db_ref("jack").db_ref("name");
    //     assert_eq!("users/jack/name.json", reference.path_json());
    // }
}