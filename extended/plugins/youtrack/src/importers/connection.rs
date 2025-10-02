pub static CFG_URL: &str = "url";
pub static CFG_TOKEN: &str = "token";

#[derive(Clone)]
pub struct YouTrackConnection {
    pub token: Option<String>,
    pub url: Option<String>,
}

impl YouTrackConnection {
    pub fn new() -> Self {
        YouTrackConnection {
            token: None,
            url: None,
        }
    }

    pub fn from(config: &model::xml::config::Configuration) -> Self {
        let mut result = YouTrackConnection::new();
        if let Some(url) = config.get(CFG_URL) {
            result.url = Some(String::from(url));
        }
        if let Some(token) = config.get(CFG_TOKEN) {
            result.token = Some(String::from(token));
        }
        result
    }

    pub fn check_config(&self) -> Option<String> {
        self.url
            .is_none()
            .then_some(CFG_URL.to_string())
            .or_else(|| self.token.is_none().then_some(CFG_TOKEN.to_string()))
    }

    pub fn all_variables() -> String {
        format!("{},{}", CFG_URL, CFG_TOKEN)
    }
}
