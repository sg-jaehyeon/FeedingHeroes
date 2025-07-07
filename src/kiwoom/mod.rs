pub struct Context {
    // Request
    appkey: String,
    secret_key: String,

    // From response
    expires_dt: String,
    token_type: String,
    token: String,

    // others
    is_valid: bool,
    is_expired: bool
}

impl Context {
    pub fn new(appkey: String, secret_key: String) -> Self {
        Self {
            appkey,
            secret_key,
            expires_dt: String::new(),
            token_type: String::new(),
            token: String::new(),
            is_valid: false,
            is_expired: false,
        }
    }

    pub fn set_expires_dt(&mut self, expires_dt: String) {
        self.expires_dt = expires_dt;
    }

    pub fn set_token_type(&mut self, token_type: String) {
        self.token_type = token_type;
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }
}
