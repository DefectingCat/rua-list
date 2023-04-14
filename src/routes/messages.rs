use axum::{extract::Query, http, response};
use log::{error, info};
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

pub async fn get_sms_aspx(
    uri: http::Uri,
    Query(params): Query<SMSParams>,
) -> impl response::IntoResponse {
    match sms_aspx(uri, params).await {
        Ok(body) => {
            info!("Got response from sms.aspx {body}");
            (http::StatusCode::OK, body)
        }
        Err(err) => {
            error!("Failed to request sms.aspx {err}");
            (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to request sms.aspx {err}".to_owned(),
            )
        }
    }
}