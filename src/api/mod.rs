mod macros;
pub mod acnt;
pub mod oauth2;
pub mod ordr;

use std::fmt::Debug;
use std::collections::HashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use axum::http::{HeaderMap, HeaderName, HeaderValue};

pub trait ApiSpec {
    type ReqHeader: Serialize;
    type Req: Serialize;
    type ResHeader: Serialize + DeserializeOwned + Debug + Clone;
    type Res: Serialize + DeserializeOwned + Debug + Clone;

    const HOST: &'static str;
    const ENDPOINT: &'static str;
    const METHOD: reqwest::Method;
}

pub struct GenericApi<T: ApiSpec> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: ApiSpec> GenericApi<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn request(
        &self,
        header: &T::ReqHeader,
        body: &T::Req
    ) -> Result<(T::ResHeader, T::Res), reqwest::Error> {
        let url = format!("{}/{}", T::HOST.trim_end_matches('/'), T::ENDPOINT.trim_start_matches('/'));

        let header_value = serde_json::to_value(header)
            .expect("Failed to serialize header");

        let mut map = HeaderMap::new();

        let pairs = match header_value {
            Value::Object(map) => {
                map.into_iter()
                    .filter_map(|(k, v)| match v {
                        Value::String(s) => Some((k, s)),
                        Value::Number(n) => Some((k, n.to_string())),
                        Value::Bool(b) => Some((k, b.to_string())),
                        _ => None,
                    }).collect()
            },
            _ => vec![],
        };

        for (k, v) in pairs {
            let key = HeaderName::from_bytes(k.as_bytes()).unwrap();
            let value = HeaderValue::from_bytes(v.as_bytes()).unwrap();
            map.insert(key, value);
        }

        let request_builder = reqwest::Client::new()
            .request(T::METHOD.clone(), &url)
            .headers(map);

        let request_builder = if let Some(body_string) = serde_json::to_value(&body).ok() {
            request_builder.json(&body_string)
        } else {
            request_builder
        };

        let response = request_builder.send().await?;
        let response = response.error_for_status()?;

        let res_header_map: HashMap<String, String> = response.headers()
            .iter()
            .filter_map(|(k, v)| {
                v.to_str().ok().map(|s| (k.to_string(), s.to_string()))
            })
            .collect();

        let res_header_json = serde_json::to_value(&res_header_map).unwrap();
        let res_header: T::ResHeader = serde_json::from_value(res_header_json).unwrap();

        let res_body = response.json::<T::Res>().await?;

        Ok((res_header, res_body))
    }
}

