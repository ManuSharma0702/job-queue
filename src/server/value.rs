use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::queue_service::service::QueuePayload;

#[derive(Deserialize, Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum TaskType {
    Split,
    Ocr,
    Aggregate
}

#[derive(Deserialize, Debug)]
pub struct GetQueryParams {
    pub task_type: TaskType,
    pub timeout: i32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "task_type", rename_all = "lowercase")]
pub enum Task {
    Split {
        job_id: String,
        file_url: String,
        retry_left: u32,
    },
    Ocr {
        job_id: String,
        file_url: String,
        page_number: u32,
        retry_left: u32,
    },
    Aggregate  {
        job_id: String,
        retry_left: u32,
    }
}

impl Task {
    pub fn task_type(&self) -> TaskType {
        match self {
            Task::Ocr { .. } => TaskType::Ocr,
            Task::Split { .. } => TaskType::Split,
            Task::Aggregate { .. } => TaskType::Aggregate
        }
    }

    pub fn get_retry(&self) -> u32 {
        match self {
            Task::Ocr { retry_left, .. } => *retry_left,
            Task::Split { retry_left, .. } => *retry_left,
            Task::Aggregate { retry_left, .. } => *retry_left
        }
    }
}

pub enum JobQueueError {
    UnexpectedError(String),
    GetTaskCallFailed
}


impl IntoResponse for JobQueueError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            JobQueueError::UnexpectedError(e) => "UnexpectedError".to_string() + &e,
            JobQueueError::GetTaskCallFailed => "Could not call get task".to_string()
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub queue_sender: Sender<QueuePayload>
}
