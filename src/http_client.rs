use crate::{consts::MSG_URL, routes::messages::SMSParams};
use anyhow::Result;
use axum::http;
use log::{debug, info};

#[derive(Debug)]
pub enum RUAService {
    Get,
    Post,
}

/// Request to the /sms.aspx and return response body as string.
/// The default response body is XML.
pub async fn sms_aspx(uri: &http::Uri, params: SMSParams, service: RUAService) -> Result<String> {
    let client = reqwest::Client::new();
    match service {
        RUAService::Get => {
            info!("Send get request to {}", uri.path());
            debug!("Send get request to {} with params {params:?}", uri.path());
            let body = client
                .get(format!("{}{uri}", MSG_URL))
                .send()
                .await?
                .text()
                .await?;
            Ok(body)
        }
        RUAService::Post => {
            info!("Send post request to {}", uri.path());
            debug!("Send post request to {} with params {params:?}", uri.path());
            let body = client
                .post(format!("{}{uri}", MSG_URL))
                .form(&params)
                .send()
                .await?
                .text()
                .await?;
            Ok(body)
        }
    }
}