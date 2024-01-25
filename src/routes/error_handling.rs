use axum::{http::StatusCode, extract::FromRequest, response::{IntoResponse, Response}};

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

// The kinds of errors we can hit in our application.
enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    // Some error from a third party library we're using
    // TimeError(time_library::Error),
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text())
            }
            // AppError::TimeError(err) => {
            //     // Because `TraceLayer` wraps each request in a span that contains the request
            //     // method, uri, etc we don't need to include those details here
            //     tracing::error!(%err, "error from time_library");

            //     // Don't expose any details about the error to the client
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "Something went wrong".to_owned(),
            //     )
            // }
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

// impl From<time_library::Error> for AppError {
//     fn from(error: time_library::Error) -> Self {
//         Self::TimeError(error)
//     }
// }