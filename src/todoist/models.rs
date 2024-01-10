use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    id: String,
    parent_id: Option<String>,
    order: u32,
    color: String,
    name: String,
    comment_count: u32,
    is_shared: bool,
    is_favorite: bool,
    is_inbox_project: bool,
    is_team_inbox: bool,
    url: String,
    view_style: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Section {
    id: String,
    project_id: String,
    order: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    creator_id: String,
    created_at: String,
    assignee_id: Option<String>,
    assigner_id: Option<String>,
    comment_count: u32,
    is_completed: bool,
    content: String,
    description: String,
    due: Option<DueDate>,
    duration: Option<Duration>,
    id: String,
    labels: Vec<String>,
    order: u32,
    priority: u32,
    project_id: Option<String>,
    section_id: Option<String>,
    parent_id: Option<String>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DueDate {
    date: String,
    is_recurring: bool,
    datetime: Option<String>,
    string: String,
    timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Duration {
    amount: u32,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    content: String,
    id: String,
    posted_at: String,
    project_id: Option<String>,
    task_id: Option<String>,
    attachment: Option<Attachment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    file_name: String,
    file_type: String,
    file_url: String,
    resource_type: String,
}
