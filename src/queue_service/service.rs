use std::collections::{HashMap, VecDeque};

use tokio::sync::{mpsc::{self, Receiver, Sender}};

use crate::{queue_service::value::QueueServiceError, server::value::{Task, TaskType}};

pub struct TaskQueues {
    pending: VecDeque<Task>,
    in_progress: VecDeque<Task>,
    failed: VecDeque<Task>
}

impl TaskQueues {
    fn new() -> Self {
        Self { 
            pending: VecDeque::new(), 
            in_progress: VecDeque::new(), 
            failed: VecDeque::new()
        }
    }
}

enum QueueOperation {
    Insert,
    Remove
}

pub type QueueResultPayload = Result<(), QueueServiceError>;

pub struct QueuePayload {
    task: Task,
    operation: QueueOperation,
    sender_tx: tokio::sync::oneshot::Sender<QueueResultPayload>
}

pub struct QueueService {
    queues: HashMap<TaskType, TaskQueues>,
    queue_service_tx: Sender<QueuePayload>,
    queue_service_rx: Receiver<QueuePayload>
}

impl QueueService {
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        let (sender, receiver) = mpsc::channel(1024);
        hashmap.insert(TaskType::Ocr, TaskQueues::new());
        hashmap.insert(TaskType::Split, TaskQueues::new());
        hashmap.insert(TaskType::Aggregate, TaskQueues::new());
        Self { queues: hashmap, queue_service_tx: sender, queue_service_rx: receiver }
    }

    pub async fn execute(&mut self) {
        while let Some(payload) = self.queue_service_rx.recv().await {
            let res = match payload.operation {
                QueueOperation::Insert => self.insert(payload.task),
                QueueOperation::Remove => self.remove(payload.task)
            };
            let _ = payload.sender_tx.send(res);
        }
    }

    pub fn get_sender(&self) -> Sender<QueuePayload> {
        self.queue_service_tx.clone()
    }

    fn insert(&mut self, task: Task) -> Result<(), QueueServiceError> {
        Ok(())
    }

    fn remove(&mut self, task: Task) -> Result<(), QueueServiceError> {
        Ok(())
    }

}


