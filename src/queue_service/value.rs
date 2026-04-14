use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum QueueServiceError {
    UnableToInsert,
    QueueNotFound,
    NoTaskFoundToInsert,
}

impl Display for QueueServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            QueueServiceError::NoTaskFoundToInsert => {
                write!(f, "No Task Found to Insert")
            }
            QueueServiceError::QueueNotFound => {
                write!(f, "Queue not found")
            }
            QueueServiceError::UnableToInsert => {
                write!(f, "Not Able to insert")
            }
        }
    }
}
