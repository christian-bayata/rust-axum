use axum::{http::StatusCode, response::{IntoResponse, Response}};
use ::serde::Serialize;
use serde_with::serde;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>; // An alias type for Result

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // -- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    // NoAuthTokenCookie,
    AuthFailCtxNotInRequestExt,

    // -- Config
    ConfigMissingEnv(&'static str),

    // -- Model errors
    TicketDeleteFailIdNotFound { id: u64}

}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("->> {:<12} -  {self:?}", "INTO_RES");

        /* Create a placeholder Axum response */
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        /* Insert the Error into the response */
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            /* Auth Error */
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            } 

            /* Model Error */
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            /* Fallback */
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
        }
    }
}