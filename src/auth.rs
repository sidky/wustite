// extern crate untrusted;
// extern crate ring;

// use self::jwt::{encode, decode, Header, Validation, Algorithm};
// use self::jwt::errors::ErrorKind;

use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Error, ErrorKind};
use std::sync::Arc;

use hyper::{Client};
use hyper_rustls::TlsClient;
use yup_oauth2::{ServiceAccountAccess, ServiceAccountKey, service_account_key_from_file, GetToken, Token};
use tokio_core::reactor::Core;
use hyper::net::HttpsConnector;
use std::env;
use serde_json;

pub trait GoogleAuth {
    fn service_token(&self) -> Result<ServiceAccountKey, Error>;

    fn token(&self) -> Result<Token, Box<::std::error::Error>> {
        let mut core = Core::new().unwrap();
        let client_secret = self.service_token()?;
        let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
        let mut access = ServiceAccountAccess::new(client_secret, client);

        access.token(&vec![
            "https://www.googleapis.com/auth/userinfo.email",
            "https://www.googleapis.com/auth/firebase.database"
        ])
    }
}

pub struct FileAuth {
    file_name: String
}

pub struct ConfigVarAuth {
    var_name: String
}

impl GoogleAuth for FileAuth {
    fn service_token(&self) -> Result<ServiceAccountKey, Error>  {
        service_account_key_from_file(&self.file_name)
    }
}

impl FileAuth {
    pub fn new(file_name: String) -> FileAuth {
        FileAuth { file_name: file_name }
    }
}

impl GoogleAuth for ConfigVarAuth {
    fn service_token(&self) -> Result<ServiceAccountKey, Error> {
        let mut key = env::var(&self.var_name).unwrap();

        match serde_json::from_str(&key) {
            Err(e) => Err(Error::new(ErrorKind::InvalidData, format!("{}", e))),
            Ok(decoded) => Ok(decoded),
        }
    }
}

impl ConfigVarAuth {
    pub fn new(var_name: String) -> ConfigVarAuth {
        ConfigVarAuth { var_name: var_name }
    }
}
