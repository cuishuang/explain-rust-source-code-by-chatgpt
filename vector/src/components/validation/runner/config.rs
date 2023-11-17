use crate::{
    components::validation::{
        sync::{Configuring, TaskCoordinator},
        util::GrpcAddress,
        ComponentConfiguration, ComponentType, ValidationConfiguration,
    },
    config::{BoxedSink, BoxedSource, BoxedTransform, ConfigBuilder},
    sinks::vector::VectorConfig as VectorSinkConfig,
    sources::vector::VectorConfig as VectorSourceConfig,
    test_util::next_addr,
};

use super::{
    io::{ControlledEdges, InputEdge, OutputEdge},
    telemetry::{Telemetry, TelemetryCollector},
};

pub struct TopologyBuilder {
    config_builder: ConfigBuilder,
    input_edge: Option<InputEdge>,
    output_edge: Option<OutputEdge>,
}

pub const TEST_SOURCE_NAME: &str = "test_source";
pub const TEST_SINK_NAME: &str = "test_sink";
pub const TEST_TRANSFORM_NAME: &str = "test_transform";
pub const TEST_INPUT_SOURCE_NAME: &str = "input_source";
pub const TEST_OUTPUT_SINK_NAME: &str = "output_sink";

impl TopologyBuilder {
    /// Creates a component topology for the given component configuration.
    pub fn from_configuration(configuration: &ValidationConfiguration) -> Self {
        let component_configuration = configuration.component_configuration();
        match component_configuration {
            ComponentConfiguration::Source(source) => {
                debug_assert_eq!(configuration.component_type(), ComponentType::Source);
                Self::from_source(source)
            }
            ComponentConfiguration::Transform(transform) => {
                debug_assert_eq!(configuration.component_type(), ComponentType::Transform);
                Self::from_transform(transform)
            }
            ComponentConfiguration::Sink(sink) => {
                debug_assert_eq!(configuration.component_type(), ComponentType::Sink);
                Self::from_sink(sink)
            }
        }
    }

    /// Creates a component topology for validating a source.
    fn from_source(source: BoxedSource) -> Self {
        let (output_edge, output_sink) = build_output_edge();

        let mut config_builder = ConfigBuilder::default();
        config_builder.add_source(TEST_SOURCE_NAME, source);
        config_builder.add_sink(TEST_OUTPUT_SINK_NAME, &[TEST_SOURCE_NAME], output_sink);

        Self {
            config_builder,
            input_edge: None,
            output_edge: Some(output_edge),
        }
    }

    fn from_transform(transform: BoxedTransform) -> Self {
        let (input_edge, input_source) = build_input_edge();
        let (output_edge, output_sink) = build_output_edge();

        let mut config_builder = ConfigBuilder::default();
        config_builder.add_source(TEST_INPUT_SOURCE_NAME, input_source);
        config_builder.add_transform(TEST_TRANSFORM_NAME, &[TEST_INPUT_SOURCE_NAME], transform);
        config_builder.add_sink(TEST_OUTPUT_SINK_NAME, &[TEST_TRANSFORM_NAME], output_sink);

        Self {
            config_builder,
            input_edge: Some(input_edge),
            output_edge: Some(output_edge),
        }
    }

    fn from_sink(sink: BoxedSink) -> Self {
        let (input_edge, input_source) = build_input_edge();

        let mut config_builder = ConfigBuilder::default();
        config_builder.add_source(TEST_INPUT_SOURCE_NAME, input_source);
        config_builder.add_sink(TEST_SINK_NAME, &[TEST_INPUT_SOURCE_NAME], sink);

        Self {
            config_builder,
            input_edge: Some(input_edge),
            output_edge: None,
        }
    }

    /// Finalizes the builder.
    ///
    /// The finalized configuration builder is returned, which can be used to create the running
    /// topology itself. All controlled edges are built and spawned, and a channel sender/receiver
    /// is provided for them. Additionally, the telemetry collector is also spawned and a channel
    /// receiver for telemetry events is provided.
    pub async fn finalize(
        mut self,
        input_task_coordinator: &TaskCoordinator<Configuring>,
        output_task_coordinator: &TaskCoordinator<Configuring>,
        telemetry_task_coordinator: &TaskCoordinator<Configuring>,
    ) -> (ConfigBuilder, ControlledEdges, TelemetryCollector) {
        let controlled_edges = ControlledEdges {
            input: self
                .input_edge
                .map(|edge| edge.spawn_input_client(input_task_coordinator)),
            output: self
                .output_edge
                .map(|edge| edge.spawn_output_server(output_task_coordinator)),
        };

        let telemetry = Telemetry::attach_to_config(&mut self.config_builder);
        let telemetry_collector = telemetry.into_collector(telemetry_task_coordinator).await;

        (self.config_builder, controlled_edges, telemetry_collector)
    }
}

fn build_input_edge() -> (InputEdge, impl Into<BoxedSource>) {
    let input_listen_addr = GrpcAddress::from(next_addr());
    debug!(listen_addr = %input_listen_addr, "Creating controlled input edge.");

    let input_source = VectorSourceConfig::from_address(input_listen_addr.as_socket_addr());
    let input_edge = InputEdge::from_address(input_listen_addr);

    (input_edge, input_source)
}

fn build_output_edge() -> (OutputEdge, impl Into<BoxedSink>) {
    let output_listen_addr = GrpcAddress::from(next_addr());
    debug!(endpoint = %output_listen_addr, "Creating controlled output edge.");

    let mut output_sink = VectorSinkConfig::from_address(output_listen_addr.as_uri());

    // We want to ensure that the output sink is flushed as soon as possible, so
    // we set the batch timeout to a very low value. We also disable retries, as
    // we don't want to waste time performing retries, especially when the test
    // harness is shutting down.
    output_sink.batch.timeout_secs = Some(0.1);
    output_sink.request.retry_attempts = Some(0);

    let output_edge = OutputEdge::from_address(output_listen_addr);

    (output_edge, output_sink)
}
