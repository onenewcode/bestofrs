use app::app_error::AppError;
use dioxus::prelude::{ServerFnError, StatusCode};

/// Error implement for http endpoint.
///
/// Using dioxus StatusCode to support both `server` and `client` feature.
pub fn api_error(err: AppError) -> ServerFnError {
    tracing::error!("API Error: {:?}", err);
    let status = match &err {
        AppError::Domain(_) => StatusCode::BAD_REQUEST,
        AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        AppError::NotFound(_) => StatusCode::NOT_FOUND,
        AppError::Upstream(_) => StatusCode::BAD_GATEWAY,
        AppError::Database(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    ServerFnError::ServerError {
        message: err.to_string(),
        code: status.as_u16(),
        details: None,
    }
}
