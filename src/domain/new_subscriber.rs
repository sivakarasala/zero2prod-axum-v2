use std::fmt;

use crate::domain::{subscriber_email::SubscriberEmail, subscriber_name::SubscriberName};
use axum::{
    Form,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl fmt::Display for NewSubscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.email, self.name)
    }
}

impl<S> FromRequest<S> for NewSubscriber
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(form) = Form::<FormData>::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;
        let name = SubscriberName::parse(form.name)
            .map_err(|e| (StatusCode::BAD_REQUEST, e).into_response())?;
        let email = SubscriberEmail::parse(form.email)
            .map_err(|e| (StatusCode::BAD_REQUEST, e).into_response())?;
        Ok(NewSubscriber { email, name })
    }
}
