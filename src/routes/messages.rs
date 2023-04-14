use axum::{
    extract::{Query, State},
    http, response,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use wildmatch::WildMatch;

use crate::{config::List, http_client::sms_aspx};

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

/// Send real request if mobile is in the whitelist.
/// First check the exact list, if mobile in there, will
/// send request directly.
/// If mobile not in exact list, then will be check the
/// whildcard list. etc.
/// If mobile not in both above, sms request will not send.
pub async fn get_sms_aspx(
    State(list): State<List>,
    uri: http::Uri,
    Query(params): Query<SMSParams>,
) -> impl response::IntoResponse {
    // Check exact list
    let mobile_finded = list.exact.iter().find(|number| **number == params.mobile);
    if mobile_finded.is_none() {
        info!("Got number not in exact list {}", params.mobile);
    }

    // Check whildcard
    let wildcard_finded = list
        .wildcard
        .iter()
        .any(|number| WildMatch::new(number).matches(&params.mobile));
    if !wildcard_finded {
        info!("Got number not in wildcard list {}", params.mobile);
        return (
            http::StatusCode::FORBIDDEN,
            "Phone number is not in whitelist".to_owned(),
        );
    }

    // Send request
    match sms_aspx(&uri, params).await {
        Ok(body) => {
            info!("Got response from {} {body}", uri.path());
            (http::StatusCode::OK, body)
        }
        Err(err) => {
            error!("Failed to request {} {err}", uri.path());
            (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to request sms.aspx {err}".to_owned(),
            )
        }
    }
}