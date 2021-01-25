use std::sync::Arc;

use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::{Devices, Locations};

enum Token {
    PersonalAccessToken(String),
}

impl Token {
    fn get_authorization_header_value(&self) -> String {
        match self {
            Token::PersonalAccessToken(token) => format!("Bearer {}", token),
        }
    }
}

struct ClientInner {
    http_client: reqwest::Client,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

impl ClientInner {
    fn new(token: Token) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&token.get_authorization_header_value())
                .expect("Failed to create authorization header"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("failed to build http client");

        Self { http_client }
    }
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

impl Client {
    pub fn with_personal_access_token(token: impl Into<String>) -> Self {
        let inner = Arc::new(ClientInner::new(Token::PersonalAccessToken(token.into())));
        Self { inner }
    }

    pub fn locations(&self) -> Locations {
        Locations {
            client: self.clone(),
        }
    }

    pub fn devices(&self) -> Devices {
        Devices {
            client: self.clone(),
        }
    }

    pub(crate) fn http(&self) -> &reqwest::Client {
        &self.inner.http_client
    }
}
