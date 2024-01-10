use crate::todoist::client::TodoistError::Other;
use crate::todoist::models::Task;
use crate::todoist::request_parameters::CreateTaskParameters;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoistError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("error: {0}")]
    Other(String),
}

pub type TodoistResult<T> = Result<T, TodoistError>;

pub struct TodoistClient {
    api_key: String,
    base_url: Url,
    target_project_id: Option<String>,
    client: reqwest::blocking::Client,
}

impl TodoistClient {
    pub fn new(api_key: String, base_url: Url, target_project_id: Option<String>) -> Self {
        TodoistClient {
            api_key,
            base_url,
            target_project_id,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn issue_get_request<T>(&self, endpoint: &str) -> TodoistResult<T>
    where
        T: DeserializeOwned,
    {
        let url = self
            .base_url
            .join(endpoint)
            .map_err(|e| Other(format!("invalid url: {}", e)))?;

        log::debug!("{:?}", &url);
        // TODO distinguish by status code

        let response = self.client.get(url).bearer_auth(&self.api_key).send()?;

        log::debug!("{:?}", &response);

        if response.status().is_success() {
            let deser = response.json()?;
            return Ok(deser);
        }

        Err(Other(format!("response was not okay: {:?}", &response)))
    }

    fn issue_post_request<T, S>(&self, endpoint: &str, body: T) -> TodoistResult<S>
    where
        T: Serialize,
        S: DeserializeOwned,
    {
        let url = self
            .base_url
            .join(endpoint)
            .map_err(|e| Other(format!("invalid url: {}", e)))?;

        log::debug!("{:?}", &url);
        // TODO distinguish by status code

        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(&self.api_key)
            .send()?;

        log::debug!("{:?}", &response);

        if response.status().is_success() {
            let deser = response.json()?;
            return Ok(deser);
        }

        Err(Other(format!("response was not okay: {:?}", &response)))
    }

    pub fn create_task(&self, content: &str) -> TodoistResult<Task> {
        self.issue_post_request(
            "tasks",
            CreateTaskParameters::new(content, self.target_project_id.clone()),
        )
    }
}
