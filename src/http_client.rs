use crate::{consts::MSG_URL, routes::messages::SMSParams};
use anyhow::Result;
use log::info;

/// Request to the /sms.aspx and return response body as string.
/// The default response body is XML.
pub async fn sms_aspx(params: SMSParams) -> Result<String> {
    info!("Request /sms.aspx with params {params:?}");
    let body = reqwest::get(format!("{}/sms.aspx", MSG_URL))
        .await?
        .text()
        .await?;
    Ok(body)
}