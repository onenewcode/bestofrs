use app::app_error::AppError;
use dioxus::prelude::ServerFnError;

pub fn api_error(err: AppError) -> ServerFnError {
    let code: u16 = match &err {
        AppError::Domain(_) => 400,
        AppError::InvalidCredentials => 401,
        AppError::NotFound(_) => 404,
        AppError::Upstream(_) => 502,
        AppError::Database(_) | AppError::Internal(_) => 500,
    };

    ServerFnError::ServerError {
        message: err.to_string(),
        code,
        details: None,
    }
}
