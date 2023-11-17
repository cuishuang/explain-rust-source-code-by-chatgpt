#![allow(clippy::print_stdout)] // tests

use std::sync::Arc;

use bytes::Bytes;
use chrono::Utc;
use futures::{
    channel::mpsc::{Receiver, TryRecvError},
    StreamExt,
};
use http::request::Parts;
use hyper::StatusCode;
use indoc::indoc;
use vector_lib::{
    config::{init_telemetry, Tags, Telemetry},
    event::{BatchNotifier, BatchStatus, Event, LogEvent},
};

use crate::{
    config::SinkConfig,
    http::HttpError,
    sinks::{
        util::retries::RetryLogic,
        util::test::{build_test_server_status, load_sink},
    },
    test_util::{
        components::{
            run_and_assert_data_volume_sink_compliance, run_and_assert_sink_compliance,
            run_and_assert_sink_error, COMPONENT_ERROR_TAGS, DATA_VOLUME_SINK_TAGS, SINK_TAGS,
        },
        next_addr, random_lines_with_stream,
    },
    tls::TlsError,
};

use super::{super::DatadogApiError, config::DatadogLogsConfig, service::LogApiRetry};

// The sink must support v1 and v2 API endpoints which have different codes for
// signaling status. This enum allows us to signal which API endpoint and what
// kind of response we want our test to model without getting into the details
// of exactly what that code is.
enum ApiStatus {
    OKv1,
    OKv2,
    BadRequestv1,
    BadRequestv2,
}

fn test_server(
    addr: std::net::SocketAddr,
    api_status: ApiStatus,
) -> (
    futures::channel::mpsc::Receiver<(http::request::Parts, Bytes)>,
    stream_cancel::Trigger,
    impl std::future::Future<Output = Result<(), ()>>,
) {
    let status = match api_status {
        ApiStatus::OKv1 => StatusCode::OK,
        ApiStatus::OKv2 => StatusCode::ACCEPTED,
        ApiStatus::BadRequestv1 | ApiStatus::BadRequestv2 => StatusCode::BAD_REQUEST,
    };

    // NOTE: we pass `Trigger` out to the caller even though this suite never
    // uses it as it's being dropped cancels the stream machinery here,
    // indicating failures that might not be valid.
    build_test_server_status(addr, status)
}

fn event_with_api_key(msg: &str, key: &str) -> Event {
    let mut e = Event::Log(LogEvent::from(msg));
    e.as_mut_log()
        .metadata_mut()
        .set_datadog_api_key(Arc::from(key));
    e
}

#[derive(PartialEq)]
enum TestType {
    Happy,
    Telemetry,
    Error,
}

/// Starts a test sink with random lines running into it
///
/// This function starts a Datadog Logs sink with a simplistic configuration and
/// runs random lines through it, returning a vector of the random lines and a
/// Receiver populated with the result of the sink's operation.
///
/// Testers may set `api_status` and `batch_status`. The first controls what
/// status code faked HTTP responses will have, the second acts as a check on
/// the `Receiver`'s status before being returned to the caller.
async fn start_test_detail(
    api_status: ApiStatus,
    batch_status: BatchStatus,
    test_type: TestType,
) -> (Vec<String>, Receiver<(http::request::Parts, Bytes)>) {
    if test_type == TestType::Telemetry {
        init_telemetry(
            Telemetry {
                tags: Tags {
                    emit_service: true,
                    emit_source: true,
                },
            },
            true,
        );
    }

    let config = indoc! {r#"
            default_api_key = "atoken"
            compression = "none"
        "#};
    let (mut config, cx) = load_sink::<DatadogLogsConfig>(config).unwrap();

    let addr = next_addr();
    // Swap out the endpoint so we can force send it
    // to our local server
    let endpoint = format!("http://{}", addr);
    config.dd_common.endpoint = Some(endpoint.clone());

    let (sink, _) = config.build(cx).await.unwrap();

    let (rx, _trigger, server) = test_server(addr, api_status);
    tokio::spawn(server);

    let (batch, receiver) = BatchNotifier::new_with_receiver();
    let (expected, events) = random_lines_with_stream(100, 10, Some(batch));

    match test_type {
        TestType::Happy => run_and_assert_sink_compliance(sink, events, &SINK_TAGS).await,
        TestType::Error => run_and_assert_sink_error(sink, events, &COMPONENT_ERROR_TAGS).await,
        TestType::Telemetry => {
            run_and_assert_data_volume_sink_compliance(sink, events, &DATA_VOLUME_SINK_TAGS).await
        }
    }

    assert_eq!(receiver.await, batch_status);

    (expected, rx)
}

async fn start_test_success(
    api_status: ApiStatus,
    batch_status: BatchStatus,
) -> (Vec<String>, Receiver<(http::request::Parts, Bytes)>) {
    start_test_detail(api_status, batch_status, TestType::Happy).await
}

async fn start_test_telemetry(
    api_status: ApiStatus,
    batch_status: BatchStatus,
) -> (Vec<String>, Receiver<(http::request::Parts, Bytes)>) {
    start_test_detail(api_status, batch_status, TestType::Telemetry).await
}

async fn start_test_error(
    api_status: ApiStatus,
    batch_status: BatchStatus,
) -> (Vec<String>, Receiver<(http::request::Parts, Bytes)>) {
    start_test_detail(api_status, batch_status, TestType::Error).await
}

/// Assert the basic functionality of the sink in good conditions
///
/// This test rigs the sink to return OKv1 to responses, checks that all batches
/// were delivered and then asserts that every message is able to be
/// deserialized.
#[tokio::test]
async fn smoke() {
    let (expected, rx) = start_test_success(ApiStatus::OKv1, BatchStatus::Delivered).await;

    let output = rx.take(expected.len()).collect::<Vec<_>>().await;

    for (i, val) in output.iter().enumerate() {
        assert_eq!(
            val.0.headers.get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(val.0.headers.get("DD-EVP-ORIGIN").unwrap(), "vector");
        assert!(val.0.headers.get("DD-EVP-ORIGIN-VERSION").is_some());

        let mut json = serde_json::Deserializer::from_slice(&val.1[..])
            .into_iter::<serde_json::Value>()
            .map(|v| v.expect("decoding json"));

        let json = json.next().unwrap();
        let message = json
            .get(0)
            .unwrap()
            .get("message")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(message, expected[i]);
        let timestamp = json
            .get(0)
            .unwrap()
            .get("timestamp")
            .unwrap()
            .as_i64()
            .unwrap();
        let delta = Utc::now().timestamp_millis() - timestamp;
        assert!(delta > 0 && delta < 1000);
    }
}

/// Assert the sink emits source and service tags when run with telemetry configured.
#[tokio::test]
async fn telemetry() {
    let (expected, rx) = start_test_telemetry(ApiStatus::OKv1, BatchStatus::Delivered).await;
    let _ = rx.take(expected.len()).collect::<Vec<_>>().await;
}

#[tokio::test]
/// Assert delivery error behavior for v1 API
///
/// In the event that delivery fails -- in this case because it is BAD_REQUEST --
/// there should be no outbound messages from the sink. That is, receiving from
/// its Receiver must fail.
async fn handles_failure_v1() {
    let (_expected, mut rx) =
        start_test_error(ApiStatus::BadRequestv1, BatchStatus::Rejected).await;
    let res = rx.try_next();

    assert!(matches!(res, Err(TryRecvError { .. })));
}

#[tokio::test]
/// Assert delivery error behavior for v2 API
///
/// In the event that delivery fails -- in this case because it is BAD_REQUEST --
/// there should be no outbound messages from the sink. That is, receiving from
/// its Receiver must fail.
async fn handles_failure_v2() {
    let (_expected, mut rx) =
        start_test_error(ApiStatus::BadRequestv2, BatchStatus::Rejected).await;
    let res = rx.try_next();

    assert!(matches!(res, Err(TryRecvError { .. })));
}

#[tokio::test]
/// Assert that metadata API keys are passed correctly, v2 API
///
/// Datadog sink payloads come with an associated API key. This key can be set
/// per-event or set in the message for an entire payload. This test asserts
/// that, for successful transmission, the API key set in metadata is
/// propagated.
async fn api_key_in_metadata_v2() {
    api_key_in_metadata_inner(ApiStatus::OKv2).await
}

#[tokio::test]
/// Assert that metadata API keys are passed correctly, v1 API
///
/// Datadog sink payloads come with an associated API key. This key can be set
/// per-event or set in the message for an entire payload. This test asserts
/// that, for successful transmission, the API key set in metadata is
/// propagated.
async fn api_key_in_metadata_v1() {
    api_key_in_metadata_inner(ApiStatus::OKv1).await
}

async fn api_key_in_metadata_inner(api_status: ApiStatus) {
    let (mut config, cx) = load_sink::<DatadogLogsConfig>(indoc! {r#"
            default_api_key = "atoken"
            compression = "none"
        "#})
    .unwrap();

    let addr = next_addr();
    // Swap out the endpoint so we can force send it to our local server
    let endpoint = format!("http://{}", addr);
    config.dd_common.endpoint = Some(endpoint.clone());

    let (sink, _) = config.build(cx).await.unwrap();

    let (rx, _trigger, server) = test_server(addr, api_status);
    tokio::spawn(server);

    let (expected_messages, events) = random_lines_with_stream(100, 10, None);

    let api_key = "0xDECAFBAD";
    let events = events.map(|mut e| {
        println!("EVENT: {:?}", e);
        e.iter_logs_mut().for_each(|log| {
            log.metadata_mut().set_datadog_api_key(Arc::from(api_key));
        });
        e
    });

    sink.run(events).await.unwrap();
    // The log API takes payloads in units of 1,000 and, as a result, we ship in
    // units of 1,000. There will only be a single response on the stream.
    let output: (Parts, Bytes) = rx.take(1).collect::<Vec<_>>().await.pop().unwrap();
    // Check that the header et al are shaped as expected.
    let parts = output.0;
    assert_eq!(parts.headers.get("DD-API-KEY").unwrap(), api_key);
    assert_eq!(
        parts.headers.get("Content-Type").unwrap(),
        "application/json"
    );
    // Check that the body appropriately transmits the messages fed in.
    let body = output.1;
    let payload_array: Vec<serde_json::Value> = serde_json::Deserializer::from_slice(&body[..])
        .into_iter::<serde_json::Value>()
        .map(|v| v.expect("decoding json"))
        .next()
        .unwrap()
        .as_array()
        .unwrap()
        .clone();
    assert_eq!(payload_array.len(), expected_messages.len());
    for (i, obj) in payload_array.into_iter().enumerate() {
        let obj = obj.as_object().unwrap();
        let message = obj.get("message").unwrap().as_str().unwrap();
        assert_eq!(message, expected_messages[i]);
    }
}

#[tokio::test]
/// Assert that events with explicit keys have those keys preserved, v1 API
///
/// Datadog sink payloads come with an associated API key. This key can be set
/// per-event or set in the message for an entire payload. This test asserts
/// that, for successful transmission, per-event API keys are propagated
/// correctly.
async fn multiple_api_keys_v1() {
    multiple_api_keys_inner(ApiStatus::OKv1).await
}

#[tokio::test]
/// Assert that events with explicit keys have those keys preserved, v2 API
///
/// Datadog sink payloads come with an associated API key. This key can be set
/// per-event or set in the message for an entire payload. This test asserts
/// that, for successful transmission, per-event API keys are propagated
/// correctly.
async fn multiple_api_keys_v2() {
    multiple_api_keys_inner(ApiStatus::OKv2).await
}

async fn multiple_api_keys_inner(api_status: ApiStatus) {
    let (mut config, cx) = load_sink::<DatadogLogsConfig>(indoc! {r#"
            default_api_key = "atoken"
            compression = "none"
        "#})
    .unwrap();

    let addr = next_addr();
    // Swap out the endpoint so we can force send it
    // to our local server
    let endpoint = format!("http://{}", addr);
    config.dd_common.endpoint = Some(endpoint.clone());

    let (sink, _) = config.build(cx).await.unwrap();

    let (rx, _trigger, server) = test_server(addr, api_status);
    tokio::spawn(server);

    let events = vec![
        event_with_api_key("mow", "pkc"),
        event_with_api_key("pnh", "vvo"),
        Event::Log(LogEvent::from("no API key in metadata")),
    ];

    sink.run_events(events).await.unwrap();

    let mut keys = rx
        .take(3)
        .map(|r| r.0.headers.get("DD-API-KEY").unwrap().clone())
        .collect::<Vec<_>>()
        .await;

    keys.sort();
    assert_eq!(keys, vec!["atoken", "pkc", "vvo"])
}

#[tokio::test]
/// Assert that events are sent and the DD-EVP-ORIGIN header is set when
/// 'enterprise' is flagged on, v2 API
///
/// Vector allows for flagging a global 'enterprise' context that indicates
/// whether we're running in Datadog enterprise mode or not. When this flag is
/// active we should set the origin header discussed above correctly, as well as
/// still sending events through the sink.
async fn enterprise_headers_v2() {
    enterprise_headers_inner(ApiStatus::OKv2).await
}

#[tokio::test]
/// Assert that events are sent and the DD-EVP-ORIGIN header is set when
/// 'enterprise' is flagged on, v1 API
///
/// Vector allows for flagging a global 'enterprise' context that indicates
/// whether we're running in Datadog enterprise mode or not. When this flag is
/// active we should set the origin header discussed above correctly, as well as
/// still sending events through the sink.
async fn enterprise_headers_v1() {
    enterprise_headers_inner(ApiStatus::OKv1).await
}

async fn enterprise_headers_inner(api_status: ApiStatus) {
    let (mut config, mut cx) = load_sink::<DatadogLogsConfig>(indoc! {r#"
            default_api_key = "atoken"
            compression = "none"
        "#})
    .unwrap();
    cx.app_name = "Vector Enterprise".to_string();
    cx.app_name_slug = "vector-enterprise".to_string();

    let addr = next_addr();
    // Swap out the endpoint so we can force send it to our local server
    let endpoint = format!("http://{}", addr);
    config.dd_common.endpoint = Some(endpoint.clone());

    let (sink, _) = config.build(cx).await.unwrap();

    let (rx, _trigger, server) = test_server(addr, api_status);
    tokio::spawn(server);

    let (_expected_messages, events) = random_lines_with_stream(100, 10, None);

    let api_key = "0xDECAFBAD";
    let events = events.map(|mut e| {
        println!("EVENT: {:?}", e);
        e.iter_logs_mut().for_each(|log| {
            log.metadata_mut().set_datadog_api_key(Arc::from(api_key));
        });
        e
    });

    sink.run(events).await.unwrap();
    let output: (Parts, Bytes) = rx.take(1).collect::<Vec<_>>().await.pop().unwrap();
    let parts = output.0;

    assert_eq!(
        parts.headers.get("DD-EVP-ORIGIN").unwrap(),
        "vector-enterprise"
    );
    assert!(parts.headers.get("DD-EVP-ORIGIN-VERSION").is_some());
}

#[tokio::test]
/// Assert that events are sent and the DD-EVP-ORIGIN header is not set when
/// 'enterprise' is flagged off, v2 API
///
/// Vector allows for flagging a global 'enterprise' context that indicates
/// whether we're running in Datadog enterprise mode or not. When this flag is
/// not active we should not set the origin header discussed above, as well as
/// still sending events through the sink.
async fn no_enterprise_headers_v2() {
    no_enterprise_headers_inner(ApiStatus::OKv2).await
}

#[tokio::test]
/// Assert that events are sent and the DD-EVP-ORIGIN header is not set when
/// 'enterprise' is flagged off, v1 API
///
/// Vector allows for flagging a global 'enterprise' context that indicates
/// whether we're running in Datadog enterprise mode or not. When this flag is
/// not active we should not set the origin header discussed above, as well as
/// still sending events through the sink.
async fn no_enterprise_headers_v1() {
    no_enterprise_headers_inner(ApiStatus::OKv1).await
}

async fn no_enterprise_headers_inner(api_status: ApiStatus) {
    let (mut config, cx) = load_sink::<DatadogLogsConfig>(indoc! {r#"
            default_api_key = "atoken"
            compression = "none"
        "#})
    .unwrap();

    let addr = next_addr();
    // Swap out the endpoint so we can force send it to our local server
    let endpoint = format!("http://{}", addr);
    config.dd_common.endpoint = Some(endpoint.clone());

    let (sink, _) = config.build(cx).await.unwrap();

    let (rx, _trigger, server) = test_server(addr, api_status);
    tokio::spawn(server);

    let (_expected_messages, events) = random_lines_with_stream(100, 10, None);

    let api_key = "0xDECAFBAD";
    let events = events.map(|mut e| {
        println!("EVENT: {:?}", e);
        e.iter_logs_mut().for_each(|log| {
            log.metadata_mut().set_datadog_api_key(Arc::from(api_key));
        });
        e
    });

    sink.run(events).await.unwrap();
    let output: (Parts, Bytes) = rx.take(1).collect::<Vec<_>>().await.pop().unwrap();
    let parts = output.0;

    assert_eq!(parts.headers.get("DD-EVP-ORIGIN").unwrap(), "vector");
    assert!(parts.headers.get("DD-EVP-ORIGIN-VERSION").is_some());
}

#[tokio::test]
/// Assert the RetryLogic implementation of LogApiRetry
async fn error_is_retriable() {
    let retry = LogApiRetry;

    // not retry-able
    assert!(!retry.is_retriable_error(&DatadogApiError::BadRequest));
    assert!(!retry.is_retriable_error(&DatadogApiError::PayloadTooLarge));
    assert!(!retry.is_retriable_error(&DatadogApiError::HttpError {
        error: HttpError::BuildRequest {
            source: http::status::StatusCode::from_u16(6666).unwrap_err().into()
        }
    }));
    assert!(!retry.is_retriable_error(&DatadogApiError::HttpError {
        error: HttpError::MakeProxyConnector {
            source: http::Uri::try_from("").unwrap_err()
        }
    }));

    // retry-able
    assert!(retry.is_retriable_error(&DatadogApiError::ServerError));
    assert!(retry.is_retriable_error(&DatadogApiError::Forbidden));
    assert!(retry.is_retriable_error(&DatadogApiError::HttpError {
        error: HttpError::BuildTlsConnector {
            source: TlsError::MissingKey
        }
    }));
    // note: HttpError::CallRequest and HttpError::MakeHttpsConnector are all retry-able,
    //       but are not straightforward to instantiate due to the design of
    //       the crates they originate from.
}
