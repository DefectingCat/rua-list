use crate::{consts::MSG_URL, routes::messages::SMSParams};
use anyhow::Result;

pub async fn sms_aspx(params: SMSParams) -> Result<()> {
    let body = reqwest::get(format!("{}/sms.aspx", MSG_URL))
        .await?
        .text()
        .await?;
    dbg!(&body);
    Ok(())
}