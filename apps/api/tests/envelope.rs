use leafypuff_api::http::envelope::Envelope;

#[test]
fn a_success_body_has_exactly_four_keys() {
    let body = serde_json::to_value(Envelope::ok("Service is live", 1_u8))
        .expect("the envelope must serialise");
    let object = body.as_object().expect("the envelope is a json object");

    assert_eq!(object.len(), 4);
    assert_eq!(object["success"], serde_json::json!(true));
    assert_eq!(object["data"], serde_json::json!(1));
    assert_eq!(object["error"], serde_json::Value::Null);
}

#[test]
fn a_failure_body_has_the_same_four_keys() {
    let body = serde_json::to_value(Envelope::failed(
        "Not authenticated",
        "UNAUTHENTICATED",
        "Bearer token missing or invalid",
    ))
    .expect("the envelope must serialise");
    let object = body.as_object().expect("the envelope is a json object");

    assert_eq!(object.len(), 4);
    assert_eq!(object["success"], serde_json::json!(false));
    assert_eq!(object["data"], serde_json::Value::Null);
    assert_eq!(
        object["error"]["code"],
        serde_json::json!("UNAUTHENTICATED")
    );
}
