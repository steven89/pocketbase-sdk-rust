use super::Client;
use crate::{api::RequestError, user::UserTypes};
use reqwest::{
    header::{self, AUTHORIZATION},
    Response, RequestBuilder,
};
use serde::Serialize;
use std::error;

impl Client {

    fn add_auth(&self, builder: RequestBuilder) -> RequestBuilder {
        if let Some(user) = &self.user {
            builder.header(AUTHORIZATION, &user.token)
        } else {
            builder
        }
    }

    pub async fn get(
        &self,
        path: impl AsRef<str>,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Response, RequestError> {
        let request_url = self.base_url.join(path.as_ref())?;
        let req_client = reqwest::Client::new();
        let mut request = req_client.get(request_url);
        if let Some(args) = params {
            request = request.query(args);
        }

        request = self.add_auth(request);

        Ok(request.send().await?)
    }

    pub async fn post<T: Serialize + Sized>(
        &self,
        path: String,
        body: &T,
    ) -> Result<Response, RequestError> {
        let request_url = self.base_url.join(path.as_str())?;
        let req_client = reqwest::Client::new();
        let b = serde_json::to_string(body).unwrap();
        let mut req = req_client
            .post(request_url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(b);

        req = self.add_auth(req);
        Ok(req.send().await?)
    }

    pub async fn patch<T: Serialize + Sized>(
        &self,
        path: String,
        body: &T,
    ) -> Result<Response, Box<dyn error::Error>> {
        match self.base_url.join(path.as_str()) {
            Ok(request_url) => {
                let req_client = reqwest::Client::new();
                let req = req_client
                    .patch(request_url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(serde_json::to_string(body).unwrap());

                let authed_req = match &self.user {
                    Some(user) => match &user.usertype {
                        UserTypes::User => {
                            req.header(AUTHORIZATION, format!("User {}", user.token))
                        }
                        UserTypes::Admin => {
                            req.header(AUTHORIZATION, format!("Admin {}", user.token))
                        }
                    },
                    None => req,
                };

                match authed_req.send().await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(Box::new(e) as Box<dyn error::Error>),
                }
            }
            Err(e) => Err(Box::new(e) as Box<dyn error::Error>),
        }
    }

    pub async fn delete(
        &self,
        path: String,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Response, RequestError> {
        let request_url = self.base_url.join(path.as_str())?;
        let req_client = reqwest::Client::new();
        let mut req = req_client
            .delete(request_url)
            .header(header::CONTENT_TYPE, "application/json");

        if let Some(args) = params {
            req = req.query(args);
        }

        req = self.add_auth(req);

        Ok(req.send().await?)
    }
}
