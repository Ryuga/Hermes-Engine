use axum::{
    extract::{FromRequest, Request},
    Json,
    http::StatusCode,
};
use serde::de::DeserializeOwned;

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    fn from_request(req: Request, state: &S) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection| (StatusCode::BAD_REQUEST, rejection.to_string()))?;

            value.validate()
                .map_err(|error_message| (StatusCode::BAD_REQUEST, error_message))?;

            Ok(ValidatedJson(value))
        }
    }
}
