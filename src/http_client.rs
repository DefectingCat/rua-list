use crate::{consts::MSG_URL, routes::messages::SMSParams};
use anyhow::Result;
use axum::http;
use log::info;

/// Request to the /sms.aspx and return response body as string.
/// The default response body is XML.
pub async fn sms_aspx(uri: &http::Uri, params: SMSParams) -> Result<String> {
    info!("Request {} with params {params:?}", uri.path());
    let body = reqwest::get(format!("{}{uri}", MSG_URL))
        .await?
        .text()
        .await?;
    Ok(body)
}