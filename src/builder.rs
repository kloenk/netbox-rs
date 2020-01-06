use super::{NetBox, Result};

use std::error::Error;

use reqwest::ClientBuilder;

#[derive(Debug)]
pub struct NetBoxBuilder {
    // the netbox token
    token: String,

    // the base url of nebox (including /api/)
    base_url: String,

    // the reqwest request builder
    client_builder: Option<ClientBuilder>,
}

impl NetBoxBuilder {
    pub fn new(token: &str, base_url: &str) -> Result<Self> {
        let client_builder = ClientBuilder::new();
        let client_builder = client_builder.use_rustls_tls();
        let client_builder = client_builder.timeout(std::time::Duration::from_secs(5));

        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Token {}", token))?,
        );

        let client_builder = client_builder.default_headers(headers);

        if !base_url.ends_with("/api/") {
            return Err(Box::<dyn Error>::from("invalid url".to_string()));
        }

        if token.len() < 40 {
            return Err(Box::<dyn Error>::from("token is to short".to_string()));
        }

        Ok(Self {
            base_url: base_url.to_string(),
            token: base_url.to_string(),
            client_builder: Some(client_builder),
        })
    }

    pub fn build(mut self) -> Result<NetBox> {
        let builder: ClientBuilder = self.client_builder.take().unwrap();
        let builder = builder.build()?;

        Ok(NetBox {
            base_url: self.base_url,
            version_cache: None,
            client: builder,
        })
    }

    // reqwest functions
    pub fn accept_invalid(&mut self, accept_invalid_certs: bool) {
        let builder = self.client_builder.take();
        let builder = builder.map(|b| b.danger_accept_invalid_certs(accept_invalid_certs));
        self.client_builder = builder;
    }

    pub fn add_root_certificate(&mut self, cert: reqwest::Certificate) {
        let builder = self.client_builder.take();
        let builder = builder.map(|b| b.add_root_certificate(cert));
        self.client_builder = builder;
    }

    pub fn timeout(&mut self, time: std::time::Duration) {
        let builder = self.client_builder.take();
        let builder = builder.map(|b| b.timeout(time));
        self.client_builder = builder;
    }

    pub fn add_proxy(&mut self, proxy: reqwest::Proxy) {
        let builder = self.client_builder.take();
        let builder = builder.map(|b| b.proxy(proxy));
        self.client_builder = builder;
    }

    pub fn user_agent(&mut self, agent: &str) {
        let builder = self.client_builder.take();
        let builder = builder.map(|b| b.user_agent(agent));
        self.client_builder = builder;
    }
}
