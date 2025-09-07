use axum::{
    extract::{Query, State},
    http, response, Form,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use wildmatch::WildMatch;

use crate::{
    config::{Config, List},
    http_client::{sms_aspx, RUAService},
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SMSParams {
    userid: Option<String>,
    account: Option<String>,
    password: Option<String>,
    mobile: String,
    content: Option<String>,
    sendTime: Option<String>,
    action: Option<String>,
    extno: Option<String>,
}

async fn send_sms(
    uri: http::Uri,
    params: SMSParams,
    service: RUAService,
) -> (http::StatusCode, String) {
    // Send request
    match sms_aspx(&uri, params, service).await {
        Ok(body) => {
            info!("Got response from {} {body}", uri.path());
            (http::StatusCode::OK, body)
        }
        Err(err) => {
            error!("Failed to request {} {err}", uri.path());
            (
                http::StatusCode::OK,
                "Failed to request sms.aspx {err}".to_owned(),
            )
        }
    }
}

/// Send real request if mobile is in the whitelist.
/// First check the exact list, if mobile in there, will
/// send request directly.
/// If mobile not in exact list, then will be check the
/// whildcard list. etc.
/// If mobile not in both above, sms request will not send.
async fn match_check(
    list: List,
    uri: http::Uri,
    params: SMSParams,
    service: RUAService,
) -> (http::StatusCode, String) {
    // Check exact list
    let mobile_finded = list.exact.iter().find(|number| **number == params.mobile);
    if mobile_finded.is_none() {
        info!("Got number not in exact list {}", params.mobile);
    } else {
        info!("Send sms with number {} in exact list", params.mobile);
        return send_sms(uri, params, service).await;
    }

    // Check whildcard
    let wildcard_finded = list
        .wildcard
        .iter()
        .any(|number| WildMatch::new(number).matches(&params.mobile));
    if !wildcard_finded {
        info!("Got number not in wildcard list {}", params.mobile);
        (
            http::StatusCode::FORBIDDEN,
            "Phone number is not in whitelist".to_owned(),
        )
    } else {
        info!("Send sms with number {} in whildcard list", params.mobile);
        send_sms(uri, params, service).await
    }
}

pub async fn match_check_get(
    State(config): State<Config>,
    uri: http::Uri,
    Query(params): Query<SMSParams>,
) -> impl response::IntoResponse {
    match_check(config.list, uri, params, RUAService::Get).await
}

pub async fn match_check_post(
    State(config): State<Config>,
    uri: http::Uri,
    Form(data): Form<SMSParams>,
) -> impl response::IntoResponse {
    match_check(config.list, uri, data, RUAService::Post).await
}
