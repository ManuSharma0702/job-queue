use std::error::Error;
use axum::{extract::Query, routing::{get, post}, Json, Router};

use crate::{queue_service::{service::QueueService}, server::value::{AppState, GetQueryParams, JobQueueError, Task}};

pub async fn run() -> Result<(), Box<dyn Error>> {
    //Initialise queue service, with three queue for each task type.
    //Pass the queue service as a state
    let mut qs = QueueService::new();
    let sender = qs.get_sender();
    tokio::spawn(async move {
        qs.execute().await;
    });

    let state = AppState {
        queue_sender: sender
    };

    let app = Router::new()
        .route("/", get(handle_root))
        .route("/task", get(get_task))
        .route("/push", post(push_task))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Job Queue running on 127.0.0.1:8080...");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_root() -> &'static str {
    "Hello job queue"
}

async fn get_task(
    Query(params): Query<GetQueryParams>
) ->Result<Json<Option<Task>>, JobQueueError> {
    // let sleep_for = Duration::from_secs(1);
    // for _ in 0..params.timeout {
    //     let data = queue_service.get_task(task_type = params.task_type);
    //     if data.len() != 0 {
    //         return Ok(data);
    //     }
    //     tokio::time::sleep(sleep_for).await;
    // }
    Ok(Json(None))
}


async fn push_task(
    Json(payload): Json<Task>
) -> Result<(), JobQueueError> {
    dbg!(payload);
    Ok(())
}
