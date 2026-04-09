use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::queue_service::service::QueuePayload;

#[derive(Deserialize, Debug)]
#[derive(Eq, Hash, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "task_type", rename_all = "lowercase")]
pub enum Task {
    Split {
        job_id: String,
        file_url: String
    },
    Ocr {
        job_id: String,
        file_url: String,
        page_number: u32
    },
    Aggregate 
}

pub enum JobQueueError {
    UnexpectedError
}


impl IntoResponse for JobQueueError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            JobQueueError::UnexpectedError => "UnexpectedError".to_string()
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub queue_sender: Sender<QueuePayload>
}
