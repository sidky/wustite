
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate http;
extern crate reqwest;

use firebase_ref::Ref;

use std::io::{self, Write};
use std::convert::From;
use std::result::Result;

use self::reqwest::{Client, Response, Error};

pub struct FirebaseDatabase<'a> {
    pub database_url: &'a str
}

impl<'a> FirebaseDatabase<'a> {
    fn db_ref(&'a self, path: &'a str) -> FirebaseRef<'a> {
        let r = FirebaseRef::new(self, path);
        r
    }

    fn new<'b>(url: &'b str) -> Result<FirebaseDatabase<'b>, io::Error> {
        Ok(FirebaseDatabase {
            database_url: url
        })
    }
}

pub struct FirebaseRef<'a> {
    database: &'a FirebaseDatabase<'a>,
    current_ref: Ref
}

impl<'a> FirebaseRef<'a> {
    fn db_ref(&self, path: &str) -> Self {
        FirebaseRef { database: self.database, current_ref: self.current_ref.db_ref(path) }
    }

    fn once(&self, name: &str) -> Result<Response, Error> {
        self.current_ref.once(self.database, name)
    }

    fn new<'b>(database: &'b FirebaseDatabase, path: &str) -> FirebaseRef<'b> {
        FirebaseRef {
            database: database,
            current_ref: Ref::new(path)
        }
    }
}

type GenError = Box<::std::error::Error>;

trait FirebaseLoader {
    // type Response;

    fn once(&self, database: &FirebaseDatabase, name: &str) -> Result<Response, Error>; 
}

impl FirebaseLoader for Ref {
    fn once(&self, database: &FirebaseDatabase, name: &str) -> Result<Response, self::reqwest::Error> {
        let mut path = database.database_url.to_owned();
        path.push_str(&self.path_string());
        path.push_str("/");
        path.push_str(name);
        path.push_str(".json");

        let client = Client::new();

        println!("Loading: {}", &path);
        client.get(&path).send()



        // let uri = path.parse::<Uri>()?;

        // client.get(uri).send()
    }
}



#[cfg(test)]
mod database_test {
extern crate serde;
extern crate serde_json;
    use super::*;

use self::serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ComicProperties {
        url: String,
    }

    // #[test]
    // fn test_path() {
    //     let reference = Ref::new("users").db_ref("jack").db_ref("name");
    //     assert_eq!("users/jack/name.json", reference.path_json());
    // }

    #[test]
    fn test_single_query() {
        let db = FirebaseDatabase::new("https://comical-d3eca.firebaseio.com/").unwrap();
        let v: ComicProperties = db.db_ref("webcomic/xkcd").once("properties").unwrap().json().unwrap();
        println!("loaded: {:?}", v);
    }
}