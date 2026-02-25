use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use log::{info, warn};
use std::{
    collections::HashSet,
    fs,
    io,
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct TokenStore {
    tokens: Arc<RwLock<HashSet<String>>>,
    path: Arc<PathBuf>,
}

impl TokenStore {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let tokens = Self::load_tokens(&path)?;
        Ok(Self {
            tokens: Arc::new(RwLock::new(tokens)),
            path: Arc::new(path),
        })
    }

    pub fn add_token(&self, token: String) -> io::Result<()> {
        let mut tokens = self.tokens.write().unwrap();
        info!("Created new token");
        tokens.insert(token);
        self.persist_tokens(&tokens)
    }

    pub fn is_valid(&self, token: &str) -> bool {
        self.tokens.read().unwrap().contains(token)
    }

    pub fn remove_token(&self, token: &str) -> io::Result<bool> {
        let mut tokens = self.tokens.write().unwrap();
        let removed = tokens.remove(token);
        self.persist_tokens(&tokens)?;
        Ok(removed)
    }

    fn load_tokens(path: &PathBuf) -> io::Result<HashSet<String>> {
        if !path.exists() {
            return Ok(HashSet::new());
        }

        let data = fs::read_to_string(path)?;
        match serde_json::from_str::<Vec<String>>(&data) {
            Ok(list) => {
                info!("Loaded tokens from file");
                Ok(list.into_iter().collect())
            },
            Err(err) => {
                warn!("Token store parse failed, starting empty: {err}");
                Ok(HashSet::new())
            }
        }
    }

    fn persist_tokens(&self, tokens: &HashSet<String>) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut list: Vec<&String> = tokens.iter().collect();
        list.sort();

        let data = serde_json::to_string_pretty(&list)?;
        info!("Persisted tokens from file");
        fs::write(&*self.path, data)
    }
}

pub async fn validate_token(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];

            let token_store = request
                .extensions()
                .get::<TokenStore>()
                .cloned();

            if let Some(store) = token_store {
                if store.is_valid(token) {
                    Ok(next.run(request).await)
                } else {
                    info!("Invalid token provided");
                    Err(StatusCode::UNAUTHORIZED)
                }
            } else {
                info!("Invalid token provided");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => {
            info!("auth: missing or invalid Authorization header");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub fn generate_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const TOKEN_LEN: usize = 32;
    let mut rng = rand::rng();

    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
