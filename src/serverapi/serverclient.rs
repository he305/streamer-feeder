use std::collections::HashMap;

use reqwest::{StatusCode, Url};
use serde_json::Value;

use crate::errors::servererror::{AuthError, ServerError};
use crate::http::customclient::CustomClient;
use crate::serverapi::server_messages::{ChannelData, RegisterMessage, ServerMessage, StreamInfoData, TokenInfo};

use super::serverconfig::ServerConfig;
use super::structs::StreamerData;

pub struct ServerClient {
    config: ServerConfig,
    handler: CustomClient,
    token: String,
}

impl ServerClient {
    pub fn new(config: ServerConfig, handler: CustomClient) -> Self {
        ServerClient {
            config,
            handler,
            token: "".to_string(),
        }
    }

    fn get_app_body(&self) -> HashMap<String, String> {
        let mut body = HashMap::new();

        body.insert("appName".to_string(), self.config.app_name.clone());
        body.insert("password".to_string(), self.config.password.clone());

        body
    }

    fn get_auth_headers(&self) -> HashMap<String, String> {
        let mut auth_headers: HashMap<String, String> = HashMap::new();
        auth_headers.insert(
            String::from("Authorization"),
            format!("Bearer {}", self.token),
        );
        auth_headers
    }

    async fn register(&mut self) -> Result<(), ServerError> {
        let url = Url::parse(format!("{}/{}", self.config.base_url, "auth/register").as_str()).unwrap();
        let app_body = self.get_app_body();

        let resp = match self
            .handler
            .post(url.as_str(), HashMap::new(), Some(app_body))
            .await
        {
            Ok(res) => res,
            Err(_) => return Err(ServerError::ServerUnavailable),
        };

        if resp.status() == StatusCode::UNAUTHORIZED {
            return Err(ServerError::ServerAuthError(
                AuthError::UserNameAlreadyExists,
            ));
        }

        let text = resp.text().await.unwrap();
        println!("{}", text);
        let message: ServerMessage<RegisterMessage> = serde_json::from_str(&text).unwrap();

        self.token = message.data[0].token_info.token.clone();
        Ok(())
    }

    async fn login(&mut self) -> Result<(), ServerError> {
        let url = Url::parse(format!("{}/{}", self.config.base_url, "auth/login").as_str()).unwrap();
        let app_body = self.get_app_body();

        let resp = match self
            .handler
            .post(url.as_str(), HashMap::new(), Some(app_body))
            .await
        {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                return Err(ServerError::ServerUnavailable);
            }
        };

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(ServerError::ServerAuthError(AuthError::NotRegistered));
        }

        let text = resp.text().await.unwrap();

        let message: ServerMessage<TokenInfo> =
            match serde_json::from_str::<ServerMessage<TokenInfo>>(&text) {
                Ok(mes) => mes,
                Err(e) => {
                    println!("{}", e);
                    return Err(ServerError::ServerUnavailable);
                }
            };

        self.token = message.data[0].token.clone();

        Ok(())
    }

    async fn validate_token(&mut self) -> Result<(), ServerError> {
        match self.login().await {
            Ok(_) => Ok(()),
            Err(e) => {
                if e == ServerError::ServerAuthError(AuthError::NotRegistered) {
                    match self.register().await {
                        Ok(_) => return Ok(()),
                        Err(e) => return Err(e),
                    };
                } else {
                    return Err(e);
                }
            }
        }
    }

    pub async fn get_all_users(&mut self) -> Result<(), ServerError> {
        self.validate_token().await?;

        let url = Url::parse(format!("{}/{}", self.config.base_url, "users/get_all").as_str()).unwrap();
        let auth_headers = self.get_auth_headers();

        let resp = match self.handler.get(url.as_str(), auth_headers).await {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                return Err(ServerError::ServerUnavailable);
            }
        };

        //if resp.status() == StatusCode::UNAUTHORIZED

        let text = resp.text().await.unwrap();

        let json: Value = serde_json::from_str(&text).unwrap();

        println!("{}", json);

        Ok(())
    }

    pub async fn get_all_channels(&mut self) -> Result<Vec<StreamerData>, ServerError> {
        self.validate_token().await?;

        let url = Url::parse(format!("{}/{}", self.config.base_url, "channels/get_all").as_str()).unwrap();
        let auth_headers = self.get_auth_headers();
        
        let resp = match self.handler.get(url.as_str(), auth_headers).await {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                return Err(ServerError::ServerUnavailable);
            }
        };

        //if resp.status() == StatusCode::UNAUTHORIZED

        let text = resp.text().await.unwrap();

        let message: ServerMessage<ChannelData> = match serde_json::from_str(&text) {
            Ok(data) => data,
            Err(_) => return Err(ServerError::ServerUnavailable),
        };

        let mut streamers = Vec::new();
        for data in message.data {
            streamers.push(StreamerData {
                channel: data,
                game_name: "".to_string(),
                title: "".to_string(),
                viewer_count: 0,
                is_live: false,
                was_live: false,
            })
        }


        Ok(streamers)
    }

    pub async fn add_streamer_data(&mut self, streamer: &StreamerData) -> Result<(), ServerError> {
        self.validate_token().await?;

        let url = Url::parse(format!("{}/{}", self.config.base_url, "streams/add_info").as_str()).unwrap();
        let auth_headers = self.get_auth_headers();
        
        let body = StreamInfoData {
            channel: streamer.channel.clone(),
            game_name: streamer.game_name.clone(),
            title: streamer.title.clone(),
            viewer_count: streamer.viewer_count,
            is_live: streamer.is_live,
        };

        let resp = match self
            .handler
            .post(url.as_str(), auth_headers, Some(body))
            .await
        {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                return Err(ServerError::ServerUnavailable);
            }
        };

        //if resp.status() == StatusCode::UNAUTHORIZED

        let text = resp.text().await.unwrap();

        println!("{}", text);
        Ok(())
    }
}

