use std::sync::OnceLock;

use http::Uri;
use snafu::ResultExt;
use tower::ServiceBuilder;
use vector_lib::config::proxy::ProxyConfig;
use vector_lib::configurable::configurable_component;

use super::{
    request_builder::DatadogMetricsRequestBuilder,
    service::{DatadogMetricsRetryLogic, DatadogMetricsService},
    sink::DatadogMetricsSink,
};
use crate::{
    config::{AcknowledgementsConfig, Input, SinkConfig, SinkContext},
    http::HttpClient,
    sinks::{
        datadog::DatadogCommonConfig,
        util::{batch::BatchConfig, ServiceBuilderExt, SinkBatchSettings, TowerRequestConfig},
        Healthcheck, UriParseSnafu, VectorSink,
    },
    tls::{MaybeTlsSettings, TlsEnableableConfig},
};

// This default is centered around "series" data, which should be the lion's share of what we
// process.  Given that a single series, when encoded, is in the 150-300 byte range, we can fit a
// lot of these into a single request, something like 150-200K series.  Simply to be a little more
// conservative, though, we use 100K here.  This will also get a little more tricky when it comes to
// distributions and sketches, but we're going to have to implement incremental encoding to handle
// "we've exceeded our maximum payload size, split this batch" scenarios anyways.
pub const MAXIMUM_PAYLOAD_COMPRESSED_SIZE: usize = 3_200_000;
pub const MAXIMUM_PAYLOAD_SIZE: usize = 62_914_560;

// TODO: revisit our concurrency and batching defaults
const DEFAULT_REQUEST_RETRY_ATTEMPTS: usize = 5;

#[derive(Clone, Copy, Debug, Default)]
pub struct DatadogMetricsDefaultBatchSettings;

impl SinkBatchSettings for DatadogMetricsDefaultBatchSettings {
    const MAX_EVENTS: Option<usize> = Some(100_000);
    const MAX_BYTES: Option<usize> = None;
    const TIMEOUT_SECS: f64 = 2.0;
}

pub(super) const SERIES_V1_PATH: &str = "/api/v1/series";
pub(super) const SERIES_V2_PATH: &str = "/api/v2/series";
pub(super) const SKETCHES_PATH: &str = "/api/beta/sketches";

// TODO: the series V1 endpoint support is considered deprecated and should be removed in a future release.
// At that time when the V1 support is removed, the SeriesApiVersion stops being useful and can be removed.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeriesApiVersion {
    V1,
    V2,
}

impl SeriesApiVersion {
    pub const fn get_path(self) -> &'static str {
        match self {
            Self::V1 => SERIES_V1_PATH,
            Self::V2 => SERIES_V2_PATH,
        }
    }
    fn get_api_version_backwards_compatible() -> Self {
        static API_VERSION: OnceLock<SeriesApiVersion> = OnceLock::new();
        *API_VERSION.get_or_init(
            || match option_env!("VECTOR_TEMP_USE_DD_METRICS_SERIES_V1_API") {
                Some(_) => Self::V1,
                None => Self::V2,
            },
        )
    }
}

/// Various metric type-specific API types.
///
/// Each of these corresponds to a specific request path when making a request to the agent API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatadogMetricsEndpoint {
    Series(SeriesApiVersion),
    Sketches,
}

impl DatadogMetricsEndpoint {
    /// Gets the content type associated with the specific encoder for a given metric endpoint.
    pub const fn content_type(self) -> &'static str {
        match self {
            Self::Series(SeriesApiVersion::V1) => "application/json",
            Self::Sketches | Self::Series(SeriesApiVersion::V2) => "application/x-protobuf",
        }
    }

    // Gets whether or not this is a series endpoint.
    pub const fn is_series(self) -> bool {
        matches!(self, Self::Series { .. })
    }

    // Creates an instance of the `Series` variant with the default API version.
    pub fn series() -> Self {
        Self::Series(SeriesApiVersion::get_api_version_backwards_compatible())
    }
}

/// Maps Datadog metric endpoints to their actual URI.
pub struct DatadogMetricsEndpointConfiguration {
    series_endpoint: Uri,
    sketches_endpoint: Uri,
}

impl DatadogMetricsEndpointConfiguration {
    /// Creates a new `DatadogMEtricsEndpointConfiguration`.
    pub const fn new(series_endpoint: Uri, sketches_endpoint: Uri) -> Self {
        Self {
            series_endpoint,
            sketches_endpoint,
        }
    }

    /// Gets the URI for the given Datadog metrics endpoint.
    pub fn get_uri_for_endpoint(&self, endpoint: DatadogMetricsEndpoint) -> Uri {
        match endpoint {
            DatadogMetricsEndpoint::Series { .. } => self.series_endpoint.clone(),
            DatadogMetricsEndpoint::Sketches => self.sketches_endpoint.clone(),
        }
    }
}

/// Configuration for the `datadog_metrics` sink.
#[configurable_component(sink("datadog_metrics", "Publish metric events to Datadog."))]
#[derive(Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct DatadogMetricsConfig {
    #[serde(flatten)]
    pub dd_common: DatadogCommonConfig,

    /// Sets the default namespace for any metrics sent.
    ///
    /// This namespace is only used if a metric has no existing namespace. When a namespace is
    /// present, it is used as a prefix to the metric name, and separated with a period (`.`).
    #[configurable(metadata(docs::examples = "myservice"))]
    #[serde(default)]
    pub default_namespace: Option<String>,

    #[configurable(derived)]
    #[serde(default)]
    pub batch: BatchConfig<DatadogMetricsDefaultBatchSettings>,

    #[configurable(derived)]
    #[serde(default)]
    pub request: TowerRequestConfig,
}

impl_generate_config_from_default!(DatadogMetricsConfig);

#[async_trait::async_trait]
#[typetag::serde(name = "datadog_metrics")]
impl SinkConfig for DatadogMetricsConfig {
    async fn build(&self, cx: SinkContext) -> crate::Result<(VectorSink, Healthcheck)> {
        let client = self.build_client(&cx.proxy)?;
        let healthcheck = self.dd_common.build_healthcheck(client.clone())?;
        let sink = self.build_sink(client)?;

        Ok((sink, healthcheck))
    }

    fn input(&self) -> Input {
        Input::metric()
    }

    fn acknowledgements(&self) -> &AcknowledgementsConfig {
        &self.dd_common.acknowledgements
    }
}

impl DatadogMetricsConfig {
    /// Gets the base URI of the Datadog agent API.
    ///
    /// Per the Datadog agent convention, we should include a unique identifier as part of the
    /// domain to indicate that these metrics are being submitted by Vector, including the version,
    /// likely useful for detecting if a specific version of the agent (Vector, in this case) is
    /// doing something wrong, for understanding issues from the API side.
    ///
    /// The `endpoint` configuration field will be used here if it is present.
    fn get_base_agent_endpoint(&self) -> String {
        self.dd_common.endpoint.clone().unwrap_or_else(|| {
            let version = str::replace(crate::built_info::PKG_VERSION, ".", "-");
            format!(
                "https://{}-vector.agent.{}",
                version,
                self.dd_common.site.as_str()
            )
        })
    }

    /// Generates the `DatadogMetricsEndpointConfiguration`, used for mapping endpoints to their URI.
    fn generate_metrics_endpoint_configuration(
        &self,
    ) -> crate::Result<DatadogMetricsEndpointConfiguration> {
        let base_uri = self.get_base_agent_endpoint();

        // TODO: the V1 endpoint support is considered deprecated and should be removed in a future release.
        // At that time, the get_api_version_backwards_compatible() should be replaced with statically using the v2.
        let series_endpoint = build_uri(
            &base_uri,
            SeriesApiVersion::get_api_version_backwards_compatible().get_path(),
        )?;
        let sketches_endpoint = build_uri(&base_uri, SKETCHES_PATH)?;

        Ok(DatadogMetricsEndpointConfiguration::new(
            series_endpoint,
            sketches_endpoint,
        ))
    }

    fn build_client(&self, proxy: &ProxyConfig) -> crate::Result<HttpClient> {
        let tls_settings = MaybeTlsSettings::from_config(
            &Some(
                self.dd_common
                    .tls
                    .clone()
                    .unwrap_or_else(TlsEnableableConfig::enabled),
            ),
            false,
        )?;
        let client = HttpClient::new(tls_settings, proxy)?;
        Ok(client)
    }

    fn build_sink(&self, client: HttpClient) -> crate::Result<VectorSink> {
        let batcher_settings = self.batch.into_batcher_settings()?;

        // TODO: revisit our concurrency and batching defaults
        let request_limits = self.request.unwrap_with(
            &TowerRequestConfig::default().retry_attempts(DEFAULT_REQUEST_RETRY_ATTEMPTS),
        );

        let endpoint_configuration = self.generate_metrics_endpoint_configuration()?;
        let service = ServiceBuilder::new()
            .settings(request_limits, DatadogMetricsRetryLogic)
            .service(DatadogMetricsService::new(
                client,
                self.dd_common.default_api_key.inner(),
            ));

        let request_builder = DatadogMetricsRequestBuilder::new(
            endpoint_configuration,
            self.default_namespace.clone(),
        )?;

        let protocol = self.get_protocol();
        let sink = DatadogMetricsSink::new(service, request_builder, batcher_settings, protocol);

        Ok(VectorSink::from_event_streamsink(sink))
    }

    fn get_protocol(&self) -> String {
        self.get_base_agent_endpoint()
            .parse::<Uri>()
            .unwrap()
            .scheme_str()
            .unwrap_or("http")
            .to_string()
    }
}

fn build_uri(host: &str, endpoint: &str) -> crate::Result<Uri> {
    let result = format!("{}{}", host, endpoint)
        .parse::<Uri>()
        .context(UriParseSnafu)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_config() {
        crate::test_util::test_generate_config::<DatadogMetricsConfig>();
    }
}
