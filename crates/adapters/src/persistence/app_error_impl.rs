pub(crate) fn db_err(err: sqlx::Error) -> app::app_error::AppError {
    app::app_error::AppError::database(err)
}
