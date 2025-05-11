use axum::{
    http::{StatusCode, header::CONTENT_TYPE},
    response::Response,
};
use serde_json::{Value, json};

pub fn err_resp(code: StatusCode, err_msg: String) -> Response<String> {
    let payload = json!({
        "status": "error",
        "data": {
            "msg": err_msg,
        }
    });
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
