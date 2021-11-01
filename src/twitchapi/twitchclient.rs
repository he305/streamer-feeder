use crate::structs::TwitchStreamData;
use crate::{http::customclient::CustomClient, structs};
use reqwest::{Url};
use std::collections::HashMap;
use structs::{TwitchData};
use structs::{TwitchErrorMessage};
use structs::TwitchAuth;
use structs::{TwitchUserData, TwitchValidate};

use crate::errors::twitcherror::TwitchError;

//TODO
use serde::{Serialize};
#[derive(Serialize)]
struct Empty;

pub struct TwitchClient<'a> {
    client: CustomClient,
    client_secret: &'a str,
    client_id: &'a str,
    token: String,
}

impl<'a> TwitchClient<'a> {
    pub fn new(client: CustomClient, client_id: &'a str, client_secret: &'a str) -> Self {
        TwitchClient {
            client,
            client_secret,
            client_id,
            token: String::from(""),
        }
    }

    fn is_error_message(&self, text: &'a String) -> bool {
        match serde_json::from_str::<'a, TwitchErrorMessage>(&text) {
            Ok(res) => {
                println!("{:?}", &res);
                true
            }
            Err(_) => false,
        }
    }

    fn get_auth_headers(&self) -> HashMap<String, String> {
        let mut auth_headers: HashMap<String, String> = HashMap::new();
        auth_headers.insert(
            String::from("Authorization"),
            format!("Bearer {}", self.token),
        );
        auth_headers.insert("Client-Id".to_string(), self.client_id.to_string());
        auth_headers
    }

    async fn validate(&self) -> Result<bool, TwitchError> {
        if self.token == "" {
            return Err(TwitchError::TokenInvalid);
        }
        let auth_headers = self.get_auth_headers();

        let resp = match self
            .client
            .get(&"https://id.twitch.tv/oauth2/validate", auth_headers)
            .await
        {
            Ok(res) => res,
            Err(_) => return Err(TwitchError::ServerUnavailable),
        };

        let text = resp.text().await.unwrap();

        if self.is_error_message(&text) {
            return Err(TwitchError::AuthQueryInvalid);
        }

        let validate_message: TwitchValidate = serde_json::from_str(&text).unwrap();

        if validate_message.client_id == self.client_id {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn get_token(&mut self) -> Result<(), TwitchError> {
        let url = Url::parse_with_params(
            "https://id.twitch.tv/oauth2/token",
            &[
                ("client_id", self.client_id),
                ("client_secret", self.client_secret),
                ("grant_type", &"client_credentials"),
            ],
        )
        .unwrap();

        let resp = match self
            .client
            .post::<Empty>(url.as_str(), HashMap::new(), None)
            .await
        {
            Ok(res) => res,
            Err(_) => return Err(TwitchError::ServerUnavailable),
        };

        let text = resp.text().await.unwrap();

        if self.is_error_message(&text) {
            return Err(TwitchError::AuthQueryInvalid);
        }

        let twitch_auth: TwitchAuth = serde_json::from_str(&text).unwrap();

        self.token = twitch_auth.access_token;

        //self.token = json["access_token"].to_string().replace('\"', "");

        match self.validate().await {
            Ok(t) => {
                println!("token is {}", t);
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }

    async fn validate_before_request(&mut self) -> Result<(), TwitchError> {
        match self.validate().await {
            Ok(_) => Ok(()),
            Err(e) => {
                if e == TwitchError::ServerUnavailable {
                    return Err(e);
                } else {
                    match self.get_token().await {
                        Err(e) => return Err(e),
                        Ok(_) => Ok(()),
                    }
                }
            }
        }
    }

    pub async fn get_broadcaster_id(
        &mut self,
        broadcaster_name: &str,
    ) -> Result<String, TwitchError> {
        self.validate_before_request().await?;

        let url = Url::parse_with_params(
            "https://api.twitch.tv/helix/users",
            &[("login", broadcaster_name)],
        )
        .unwrap();

        let auth_headers = self.get_auth_headers();

        let resp = match self.client.get(url.as_str(), auth_headers).await {
            Ok(res) => res,
            Err(_) => return Err(TwitchError::ServerUnavailable),
        };

        let text = resp.text().await.unwrap();

        if self.is_error_message(&text) {
            return Err(TwitchError::InternalError);
        }

        // let message_value: Value = serde_json::from_str(&text).unwrap();
        // println!("{}", message_value);
        let message: TwitchData<TwitchUserData> = serde_json::from_str(&text).unwrap();

        //println!("{:?}", message);

        Ok(message.data[0].id.clone())
    }

    pub async fn get_stream_data(
        &mut self,
        broadcaster_name: &str,
    ) -> Result<TwitchData<TwitchStreamData>, TwitchError> {

        let broadcaster_id = self.get_broadcaster_id(&broadcaster_name).await?;

        let auth_headers = self.get_auth_headers();

        let url = Url::parse_with_params(
            "https://api.twitch.tv/helix/streams",
            &[("user_id", broadcaster_id)],
        )
        .unwrap();

        let resp = match self.client.get(url.as_str(), auth_headers).await {
            Ok(res) => res,
            Err(_) => return Err(TwitchError::ServerUnavailable),
        };

        let text = resp.text().await.unwrap();
        if self.is_error_message(&text) {
            return Err(TwitchError::InternalError);
        }

        let data: TwitchData<TwitchStreamData> = serde_json::from_str(&text).unwrap();

        println!("{:?}", data);

        Ok(data)
    }
}
