use async_trait::async_trait;
use axum::{
    extract::{FromRequest, FromRequestParts},
    http::{header, request::Parts, Request, StatusCode},
    middleware::from_extractor,
    routing::{get, post},
    Router,
};

#[derive(Debug)]
pub struct HeaderParse;

#[async_trait]
impl<S, B> FromRequest<S, B> for HeaderParse
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // dbg!(&req);
        Ok(Self)
    }
}