use std::fmt::Debug;
use std::str::FromStr;
use std::{collections::HashMap, time::Duration};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Response};
use serde::Serialize;

pub struct CustomClient {
    pub handler: Client,
}

impl CustomClient {
    pub fn new(timeout_in_secs: u64) -> Self {
        let handler = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_in_secs))
            .build()
            .unwrap();

        return CustomClient { handler };
    }

    fn construct_headers(&self, headers: &HashMap<String, String>) -> HeaderMap {
        let mut headermap = HeaderMap::new();

        for (key, value) in headers {
            headermap.insert(
                HeaderName::from_str(key.as_str()).unwrap(),
                HeaderValue::from_str(&value.as_str()).unwrap(),
            );
        }

        headermap
    }

    pub async fn get(
        &self,
        url: &str,
        headers: HashMap<String, String>,
    ) -> Result<Response, reqwest::Error> {
        let headermap = self.construct_headers(&headers);

        let resp = self.handler.get(url).headers(headermap).send().await?;

        Ok(resp)
    }

    pub async fn post<T: Serialize + Debug>(
        &self,
        url: &str,
        headers: HashMap<String, String>,
        body: Option<T>,
    ) -> Result<Response, reqwest::Error> {
        let headermap = self.construct_headers(&headers);

        let mut req = self.handler.post(url).headers(headermap);

        if body.is_some() {
            req = req.json(&body);
        }

        match req.send().await {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("Error occured while fetching {}, with headers {:?}, with body {:?}", url, headers, body);
                Err(e)
            },
        }
    }
}
