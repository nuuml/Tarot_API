use std::{
    collections::HashSet,
    fs,
    io,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use log::{info, warn};

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
