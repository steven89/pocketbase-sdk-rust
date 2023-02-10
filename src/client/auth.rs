use super::Client;
use crate::api::{ApiError, RequestError};
use crate::user::{User, UserTypes};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessAuthResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AuthResponse {
    SuccessAuthResponse(SuccessAuthResponse),
    FailureAuthResponse(ApiError),
}

#[derive(Debug, Serialize, Deserialize)]
struct PasswordCredentials {
    identity: String,
    password: String,
}

impl Client {
    pub async fn auth_via_email(
        &mut self,
        email: String,
        password: String,
        usertype: UserTypes,
    ) -> Result<(), RequestError> {
        let credentials: PasswordCredentials = PasswordCredentials {
            identity: email,
            password,
        };
        Ok(self.authenticate(&usertype, &credentials).await?)
    }

    fn get_auth_url(&self, user_type: &UserTypes) -> &str {
        match user_type {
            UserTypes::User => "collections/users/auth-with-password",
            UserTypes::Admin => "admins/auth-with-password",
        }
    }

    async fn authenticate(
        &mut self,
        user_type: &UserTypes,
        credentials: &PasswordCredentials,
    ) -> Result<(), RequestError> {
        let auth_response = self
            .post(
                String::from(self.get_auth_url(user_type)),
                Some(&credentials),
            )
            .await?;
        let parsed_resp = auth_response.json::<AuthResponse>().await?;

        match parsed_resp {
            AuthResponse::SuccessAuthResponse(response) => {
                self.user = Some(User {
                    usertype: user_type.clone(),
                    token: response.token,
                });

                Ok(())
            }
            AuthResponse::FailureAuthResponse(e) => Err(RequestError::Api(e)),
        }
    }

    pub async fn refresh_auth(&mut self) -> Result<(), RequestError> {
        let res = self
            .post::<()>("admins/auth-refresh".to_string(), None)
            .await?;
        let parsed_resp = res.json::<AuthResponse>().await?;
        match parsed_resp {
            AuthResponse::SuccessAuthResponse(r) => {
                let mut user = self.user.clone().unwrap_or(User {
                    usertype: UserTypes::User,
                    token: "".to_string(),
                });
                user.token = r.token;
                self.user = Some(user);
                Ok(())
            }
            AuthResponse::FailureAuthResponse(e) => Err(RequestError::Api(e)),
        }
    }
}
