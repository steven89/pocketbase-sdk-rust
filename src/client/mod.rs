mod auth;
mod base;


use url::Url;
use crate::user::User;

#[derive(Debug)]
pub struct Client {
    pub base_url: Url,
    pub user: Option<User>
}

impl Client {
    pub fn new(raw_url: &str) -> Result<Client, url::ParseError> {
        match Url::parse(raw_url) {
            Ok(url_object) => Ok(Client { base_url: url_object, user: None }) ,
            Err(e) => Err(e)
        }
    }
}
