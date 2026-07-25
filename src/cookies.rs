//! Persistent cookie storage for session persistence.
//!
//! Saves and loads the FunPay golden key authentication token
//! from a local file for session reuse across restarts.

use std::fs;
use std::path::Path;

/// File-backed cookie store for the golden key.
#[derive(Debug, Clone)]
pub struct CookieStore {
    path: String,
    golden_key: Option<String>,
}

impl CookieStore {
    /// Create a new cookie store backed by the given file path.
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            golden_key: None,
        }
    }

    /// Load the golden key from disk.
    ///
    /// Returns `true` if a key was found and loaded, `false` otherwise.
    pub fn load(&mut self) -> bool {
        if let Ok(data) = fs::read_to_string(&self.path) {
            if let Some(key) = data
                .strip_prefix("golden_key=")
                .map(|s| s.trim().to_string())
            {
                self.golden_key = Some(key);
                return true;
            }
        }
        false
    }

    /// Save a golden key to disk.
    pub fn save(&self, golden_key: &str) -> Result<(), std::io::Error> {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, format!("golden_key={}\n", golden_key))
    }

    /// Get the loaded golden key, if any.
    pub fn golden_key(&self) -> Option<&str> {
        self.golden_key.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_and_load() {
        let dir = std::env::temp_dir().join("funpay_cookie_test");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("cookies.txt");

        let store = CookieStore::new(path.to_str().unwrap());
        store.save("test_key_123").unwrap();

        let mut loaded = CookieStore::new(path.to_str().unwrap());
        assert!(loaded.load());
        assert_eq!(loaded.golden_key(), Some("test_key_123"));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_load_nonexistent() {
        let mut store = CookieStore::new("/tmp/funpay_nonexistent_key.txt");
        assert!(!store.load());
        assert!(store.golden_key().is_none());
    }

    #[test]
    fn test_new_store_has_no_key() {
        let store = CookieStore::new("/tmp/test.txt");
        assert!(store.golden_key().is_none());
    }
}
