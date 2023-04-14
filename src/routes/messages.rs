use axum::extract::Query;
use log::error;
use serde::{Deserialize, Serialize};

use crate::http_client::sms_aspx;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SMSParams {
    userid: String,
    account: String,
    password: String,
    mobile: String,
    content: String,
    sendTime: String,
    action: String,
    extno: String,
}

pub async fn get_sms_aspx(Query(params): Query<SMSParams>) -> String {
    match sms_aspx(params).await {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to request sms.aspx {err}")
        }
    };
    "".to_owned()
}