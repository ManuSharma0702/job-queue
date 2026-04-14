#[derive(Debug)]
pub enum QueueServiceError {
    UnableToInsert,
    QueueNotFound,
    NoTaskFoundToInsert,
}
