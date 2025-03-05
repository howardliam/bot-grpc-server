use sqlx::Error;
use tonic::Status;

pub fn sqlx_error_to_tonic_status(error: &Error) -> Status {
    match error {
        Error::Database(database_error) => Status::internal(database_error.to_string()),
        Error::Io(error) => Status::internal(error.to_string()),
        Error::RowNotFound => Status::not_found(error.to_string()),
        _ => Status::internal(error.to_string()),
    }
}
