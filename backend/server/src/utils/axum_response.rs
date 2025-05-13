use axum::{
    http::{StatusCode, header::CONTENT_TYPE},
    response::Response,
};
use serde_json::{Value, json};

#[macro_export]
macro_rules! err_resp_log {
    ($code:expr, $msg:expr) => {{
        use tracing::error;
        use $crate::utils::axum_response::err_resp;

        error!("{}", $msg);
        err_resp($code, $msg, None)
    }};
    ($code:expr, $msg:expr, $body:expr) => {{
        use tracing::error;
        use $crate::utils::axum_response::err_resp;

        error!("{}", $msg);
        err_resp($code, $msg, Some($body))
    }};
}

pub fn err_resp(code: StatusCode, err_msg: &str, body: Option<Value>) -> Response<String> {
    let mut payload = json!({
        "status": "error",
        "msg": err_msg,
    });
    if let Some(body) = body {
        payload["data"] = body;
    }

    resp_json(code, payload)
}

pub fn succ_resp(code: StatusCode, payload: Value) -> Response<String> {
    let payload = json!({
        "status": "success",
        "data": payload
    });
    resp_json(code, payload)
}

pub fn resp_json(code: StatusCode, payload: Value) -> Response<String> {
    Response::builder()
        .status(code)
        .header(CONTENT_TYPE, "application/json")
        .body(payload.to_string())
        .unwrap()
}
