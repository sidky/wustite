use firebase_ref::Ref;

use std::io::{self, Write};
use std::cell::RefCell;
use std::convert::From;
use std::result::Result;
use yup_oauth2::Token;
use auth::{GoogleAuth, FileAuth, ConfigVarAuth};

use reqwest::{Client, Response, Error, Request};

use serde::Serialize;

pub struct FirebaseDatabase<'a> {
    pub database_url: &'a str,
    pub auth: Box<GoogleAuth>,
    token: RefCell<Token>,
    client: Client
}

impl<'a> FirebaseDatabase<'a> {
    fn db_ref(&'a self, path: &'a str) -> FirebaseRef<'a> {
        let r = FirebaseRef::new(self, path);
        r
    }

    fn new<'b>(url: &'b str, service_file: &'b str) -> Result<FirebaseDatabase<'b>, io::Error> {
        let auth = FileAuth::new(service_file.to_owned());
        let token = RefCell::new(auth.token().expect("Invalid token"));
        Ok(FirebaseDatabase {
            database_url: url,
            client: Client::new(),
            auth: Box::new(auth),
            token: token
        })
    }

    fn new_from_variable<'b>(url: &'b str, var_name: &'b str) ->
            Result<FirebaseDatabase<'b>, io::Error> {
        let auth = ConfigVarAuth::new(var_name.to_owned());
        let token = RefCell::new(auth.token().expect("Invalid token"));
        Ok(FirebaseDatabase {
            database_url: url,
            client: Client::new(),
            auth: Box::new(auth),
            token: token
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

    fn set<T: Serialize>(&self, name: &str, data: &T) -> Result<Response, Error> {
        self.current_ref.set(self.database, name, data)
    }

    fn new<'b>(database: &'b FirebaseDatabase, path: &str) -> FirebaseRef<'b> {
        FirebaseRef {
            database: database,
            current_ref: Ref::new(path)
        }
    }
}

trait FirebaseLoader {
    // type Response;

    fn once(&self, database: &FirebaseDatabase, name: &str) -> Result<Response, Error>; 

    fn set<T : Serialize> (&self, database: &FirebaseDatabase, name: &str, data: &T) -> Result<Response, Error>;
}

impl FirebaseLoader for Ref {
    fn once(&self, database: &FirebaseDatabase, name: &str) -> Result<Response, Error> {
        println!("Loading now");
        let mut path = database.database_url.to_owned();
        path.push_str(&self.path_string());
        path.push_str("/");
        path.push_str(name);
        path.push_str(".json");
        path.push_str(&format!("?access_token={}", database.token.borrow().access_token));
        database.client.get(&path).send()
    }

     fn set<T : Serialize> (&self, database: &FirebaseDatabase, name: &str, data: &T) -> Result<Response, Error> {
        println!("Saving now");
        let mut path = database.database_url.to_owned();
        path.push_str(&self.path_string());
        path.push_str("/");
        path.push_str(name);
        path.push_str(".json");
        path.push_str(&format!("?access_token={}", database.token.borrow().access_token));
        let serialized_data = ::serde_json::to_string(data).expect("serialize");
        database.client.put(&path).body(serialized_data).send()
    }
}