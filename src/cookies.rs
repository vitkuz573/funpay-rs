use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CookieStore {
    path: String,
    golden_key: Option<String>,
}

impl CookieStore {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            golden_key: None,
        }
    }

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

    pub fn save(&self, golden_key: &str) -> Result<(), std::io::Error> {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, format!("golden_key={}\n", golden_key))
    }

    pub fn golden_key(&self) -> Option<&str> {
        self.golden_key.as_deref()
    }
}
