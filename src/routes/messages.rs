use axum::extract::Query;
use serde::{Deserialize, Serialize};

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
    format!("Demo query params: {:?}", params)
}