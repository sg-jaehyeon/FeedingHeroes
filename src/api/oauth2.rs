use crate::define_api;

use std::fmt::Debug;

define_api!(
    Au10001 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/oauth2/token",
        method: reqwest::Method::POST,

        ReqHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String
        },
        Req {
            pub grant_type: String,
            pub appkey: String,
            pub secretkey: String
        },
        ResHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String,
        },
        Res {
            pub expires_dt: String,
            pub token_type: String,
            pub token: String
        }
    }
);


define_api!(
    Au10002 {
        host: "https://mockapi.kiwoom.com",
        endpoint: "/oauth2/revoke",
        method: reqwest::Method::POST,

        ReqHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String,
        },
        Req {
            pub appkey: String,
            pub secretkey: String,
            pub token: String,
        },
        ResHeader {
            #[serde(rename = "Content-Type")]
            pub content_type: String,
        },
        Res {
            // no contents
        }
    }
);

