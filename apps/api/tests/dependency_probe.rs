use leafypuff_api::domain::ReadinessProbe;
use leafypuff_api::infrastructure::DependencyProbe;

#[tokio::test]
async fn an_unreachable_database_and_storage_report_not_ready() {
    let probe = DependencyProbe::unreachable("127.0.0.1:1".to_owned());
    let report = probe
        .check()
        .await
        .expect("the probe reports a verdict, it does not error");

    assert!(!report.database);
    assert!(!report.object_storage);
    assert!(!report.is_ready());
}

#[tokio::test]
async fn an_unreachable_storage_endpoint_alone_blocks_readiness() {
    let probe = DependencyProbe::unreachable("127.0.0.1:1".to_owned());
    let report = probe.check().await.expect("the probe reports a verdict");

    assert!(!report.is_ready());
}
