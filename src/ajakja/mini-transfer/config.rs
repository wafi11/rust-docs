use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub bind_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            endpoint: env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://127.0.0.1:9000".to_string()),
            region: env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            bucket: env::var("S3_BUCKET").unwrap_or_else(|_| "mini-transfer".to_string()),
            access_key: required("S3_ACCESS_KEY")?,
            secret_key: required("S3_SECRET_KEY")?,
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        })
    }
}

fn required(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("env {key} belum di-set"))
}
