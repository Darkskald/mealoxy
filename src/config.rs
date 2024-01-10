use reqwest::Url;

pub struct Config {
    pub todoist_api_key: String,
    pub todoist_base_url: Url,
}

impl Config {
    pub fn new() -> Self {
        let todoist_api_key = std::env::var("TODOIST_API_KEY").unwrap();
        let todoist_base_url = Url::parse("https://api.todoist.com/rest/v2/").unwrap();
        Config {
            todoist_api_key,
            todoist_base_url,
        }
    }
}
