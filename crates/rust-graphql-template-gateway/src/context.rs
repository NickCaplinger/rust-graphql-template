use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct MyContext {
    pub up_since: DateTime<Utc>,
}

pub struct AuthContext {
    pub bearer_token: Option<String>,
}
