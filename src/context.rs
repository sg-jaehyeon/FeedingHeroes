use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserKey {
    pub appkey: String,
    pub secretkey: String,
}

type OAuth2Token = crate::api::oauth2::Au10001Res;

#[derive(Debug)]
pub struct UserContext {
    pub user_key: UserKey,
    pub oauth2_token: Option<OAuth2Token>,
}

pub static USER_CONTEXT: Lazy<Mutex<UserContext>> = Lazy::new(|| {
    // Load the app key from a file or environment variable
    let file = File::open("res/practice/key.json")
        .expect("Failed to open app key file");
    let reader = BufReader::new(file);

    let contents: UserKey = serde_json::from_reader(reader)
        .expect("Failed to parse app key from res/practice/key.json");

    Mutex::new(UserContext {
        user_key: contents,
        oauth2_token: None,
    })
});

impl UserContext {
    pub fn get_appkey(&self) -> String {
        self.user_key.appkey.clone()
    }

    pub fn get_secretkey(&self) -> String {
        self.user_key.secretkey.clone()
    }

    pub fn set_oauth2_token(&mut self, token: OAuth2Token) {
        self.oauth2_token = Some(token);
    }

    pub fn get_oauth2_token(&self) -> Option<&OAuth2Token> {
        self.oauth2_token.as_ref()
    }
}