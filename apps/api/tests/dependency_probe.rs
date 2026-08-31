use leafypuff_api::domain::ReadinessProbe;
use leafypuff_api::infrastructure::DependencyProbe;

#[tokio::test]
async fn an_unreachable_database_and_storage_report_not_ready() {
    let probe = DependencyProbe::new(
        "postgres://127.0.0.1:1/absent".to_owned(),
        "127.0.0.1:1".to_owned(),
    );
    let report = probe
        .check()
        .await
        .expect("the probe reports a verdict, it does not error");

    assert!(!report.database);
    assert!(!report.object_storage);
    assert!(!report.is_ready());
}

#[tokio::test]
async fn the_probe_re_evaluates_on_every_call_rather_than_caching_a_verdict() {
    let probe = DependencyProbe::new(
        "postgres://127.0.0.1:1/absent".to_owned(),
        "127.0.0.1:1".to_owned(),
    );

    let first = probe.check().await.expect("first check reports");
    let second = probe.check().await.expect("second check reports");

    assert_eq!(first.database, second.database);
    assert_eq!(first.object_storage, second.object_storage);
}
