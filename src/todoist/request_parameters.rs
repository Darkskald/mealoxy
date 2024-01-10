use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CreateTaskParameters {
    content: String,
    project_id: Option<String>,
    labels: Vec<String>,
}

impl CreateTaskParameters {
    pub fn new(content: &str, project_id: Option<String>) -> Self {
        CreateTaskParameters {
            content: content.to_string(),
            project_id,
            labels: vec!["mealoxy".to_string()],
        }
    }
}
