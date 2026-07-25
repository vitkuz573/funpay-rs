use serde_json::Value;
use crate::error::FunPayError;

pub struct CsrfTokens {
    pub form_token: String,
    pub header_token: String,
}

impl CsrfTokens {
    pub fn from_html(html: &str) -> Result<Self, FunPayError> {
        let start = html.find("data-app-data='").ok_or_else(|| FunPayError::Parse("No data-app-data found".into()))?;
        let json_start = start + 15;
        let json_end = html[json_start..].find("'").ok_or_else(|| FunPayError::Parse("Unclosed data-app-data".into()))?;
        let json_str = &html[json_start..json_start + json_end];
        
        let data: Value = serde_json::from_str(json_str)
            .map_err(|e| FunPayError::Parse(format!("JSON parse error: {}", e)))?;
        
        let form_token = data["csrf-token"].as_str()
            .ok_or_else(|| FunPayError::Parse("No csrf-token in JSON".into()))?
            .to_string();
        
        Ok(Self { form_token: form_token.clone(), header_token: form_token })
    }
}
