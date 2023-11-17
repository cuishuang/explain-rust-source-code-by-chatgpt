use std::{convert::TryFrom, sync::Arc};

use indoc::indoc;
use tower::ServiceBuilder;
use vector_lib::configurable::configurable_component;
use vector_lib::{config::proxy::ProxyConfig, schema::meaning};
use vrl::value::Kind;

use super::{service::LogApiRetry, sink::LogSinkBuilder};
use crate::{
    codecs::Transformer,
    config::{AcknowledgementsConfig, GenerateConfig, Input, SinkConfig, SinkContext},
    http::HttpClient,
    schema,
    sinks::{
        datadog::{logs::service::LogApiService, DatadogCommonConfig},
        util::{
            http::RequestConfig, service::ServiceBuilderExt, BatchConfig, Compression,
            SinkBatchSettings,
        },
        Healthcheck, VectorSink,
    },
    tls::{MaybeTlsSettings, TlsEnableableConfig},
};

// The Datadog API has a hard limit of 5MB for uncompressed payloads. Above this
// threshold the API will toss results. We previously serialized Events as they
// came in -- a very CPU intensive process -- and to avoid that we only batch up
// to 750KB below the max and then build our payloads. This does mean that in
// some situations we'll kick out over-large payloads -- for instance, a string
// of escaped double-quotes -- but we believe this should be very rare in
// practice.
pub const MAX_PAYLOAD_BYTES: usize = 5_000_000;
pub const BATCH_GOAL_BYTES: usize = 4_250_000;
pub const BATCH_MAX_EVENTS: usize = 1_000;
pub const BATCH_DEFAULT_TIMEOUT_SECS: f64 = 5.0;

#[derive(Clone, Copy, Debug, Default)]
pub struct DatadogLogsDefaultBatchSettings;

impl SinkBatchSettings for DatadogLogsDefaultBatchSettings {
    const MAX_EVENTS: Option<usize> = Some(BATCH_MAX_EVENTS);
    const MAX_BYTES: Option<usize> = Some(BATCH_GOAL_BYTES);
    const TIMEOUT_SECS: f64 = BATCH_DEFAULT_TIMEOUT_SECS;
}

/// Configuration for the `datadog_logs` sink.
#[configurable_component(sink("datadog_logs", "Publish log events to Datadog."))]
#[derive(Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct DatadogLogsConfig {
    #[serde(flatten)]
    pub dd_common: DatadogCommonConfig,

    #[configurable(derived)]
    #[serde(default)]
    pub compression: Option<Compression>,

    #[configurable(derived)]
    #[serde(
        default,
        skip_serializing_if = "crate::serde::skip_serializing_if_default"
    )]
    pub encoding: Transformer,

    #[configurable(derived)]
    #[serde(default)]
    pub batch: BatchConfig<DatadogLogsDefaultBatchSettings>,

    #[configurable(derived)]
    #[serde(default)]
    pub request: RequestConfig,
}

impl GenerateConfig for DatadogLogsConfig {
    fn generate_config() -> toml::Value {
        toml::from_str(indoc! {r#"
            default_api_key = "${DATADOG_API_KEY_ENV_VAR}"
        "#})
        .unwrap()
    }
}

impl DatadogLogsConfig {
    // TODO: We should probably hoist this type of base URI generation so that all DD sinks can
    // utilize it, since it all follows the same pattern.
    fn get_uri(&self) -> http::Uri {
        let base_url = self
            .dd_common
            .endpoint
            .clone()
            .unwrap_or_else(|| format!("https://http-intake.logs.{}", self.dd_common.site));

        http::Uri::try_from(format!("{}/api/v2/logs", base_url)).expect("URI not valid")
    }

    fn get_protocol(&self) -> String {
        self.get_uri().scheme_str().unwrap_or("http").to_string()
    }

    pub fn build_processor(
        &self,
        client: HttpClient,
        dd_evp_origin: String,
    ) -> crate::Result<VectorSink> {
        let default_api_key: Arc<str> = Arc::from(self.dd_common.default_api_key.inner());
        let request_limits = self.request.tower.unwrap_with(&Default::default());

        // We forcefully cap the provided batch configuration to the size/log line limits imposed by
        // the Datadog Logs API, but we still allow them to be lowered if need be.
        let batch = self
            .batch
            .validate()?
            .limit_max_bytes(BATCH_GOAL_BYTES)?
            .limit_max_events(BATCH_MAX_EVENTS)?
            .into_batcher_settings()?;

        let service = ServiceBuilder::new()
            .settings(request_limits, LogApiRetry)
            .service(LogApiService::new(
                client,
                self.get_uri(),
                self.request.headers.clone(),
                dd_evp_origin,
            )?);

        let encoding = self.encoding.clone();
        let protocol = self.get_protocol();

        let sink = LogSinkBuilder::new(encoding, service, default_api_key, batch, protocol)
            .compression(self.compression.unwrap_or_default())
            .build();

        Ok(VectorSink::from_event_streamsink(sink))
    }

    pub fn create_client(&self, proxy: &ProxyConfig) -> crate::Result<HttpClient> {
        let tls_settings = MaybeTlsSettings::from_config(
            &Some(
                self.dd_common
                    .tls
                    .clone()
                    .unwrap_or_else(TlsEnableableConfig::enabled),
            ),
            false,
        )?;
        Ok(HttpClient::new(tls_settings, proxy)?)
    }
}

#[async_trait::async_trait]
#[typetag::serde(name = "datadog_logs")]
impl SinkConfig for DatadogLogsConfig {
    async fn build(&self, cx: SinkContext) -> crate::Result<(VectorSink, Healthcheck)> {
        let client = self.create_client(&cx.proxy)?;

        let healthcheck = self.dd_common.build_healthcheck(client.clone())?;

        let sink = self.build_processor(client, cx.app_name_slug)?;

        Ok((sink, healthcheck))
    }

    fn input(&self) -> Input {
        let requirement = schema::Requirement::empty()
            .required_meaning(meaning::MESSAGE, Kind::bytes())
            .required_meaning(meaning::TIMESTAMP, Kind::timestamp())
            .optional_meaning(meaning::HOST, Kind::bytes())
            .optional_meaning(meaning::SOURCE, Kind::bytes())
            .optional_meaning(meaning::SEVERITY, Kind::bytes())
            .optional_meaning(meaning::SERVICE, Kind::bytes())
            .optional_meaning(meaning::TRACE_ID, Kind::bytes());

        Input::log().with_schema_requirement(requirement)
    }

    fn acknowledgements(&self) -> &AcknowledgementsConfig {
        &self.dd_common.acknowledgements
    }
}

#[cfg(test)]
mod test {
    use super::super::config::DatadogLogsConfig;

    #[test]
    fn generate_config() {
        crate::test_util::test_generate_config::<DatadogLogsConfig>();
    }
}
