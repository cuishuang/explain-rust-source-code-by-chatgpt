use metrics::counter;

use vector_lib::internal_event::InternalEvent;
use vector_lib::{
    internal_event::{error_stage, error_type},
    json_size::JsonSize,
};

use super::prelude::http_error_code;
use crate::sources::apache_metrics;

#[derive(Debug)]
pub struct ApacheMetricsEventsReceived<'a> {
    pub byte_size: JsonSize,
    pub count: usize,
    pub endpoint: &'a str,
}

impl<'a> InternalEvent for ApacheMetricsEventsReceived<'a> {
    // ## skip check-duplicate-events ##
    fn emit(self) {
        trace!(message = "Events received.", count = %self.count, byte_size = %self.byte_size, endpoint = %self.endpoint);
        counter!(
            "component_received_events_total", self.count as u64,
            "endpoint" => self.endpoint.to_owned(),
        );
        counter!(
            "component_received_event_bytes_total", self.byte_size.get() as u64,
            "endpoint" => self.endpoint.to_owned(),
        );
    }
}

#[derive(Debug)]
pub struct ApacheMetricsParseError<'a> {
    pub error: apache_metrics::ParseError,
    pub endpoint: &'a str,
}

impl InternalEvent for ApacheMetricsParseError<'_> {
    fn emit(self) {
        error!(
            message = "Parsing error.",
            error = ?self.error,
            stage = error_stage::PROCESSING,
            error_type = error_type::PARSER_FAILED,
            endpoint = %self.endpoint,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "stage" => error_stage::PROCESSING,
            "error_type" => error_type::PARSER_FAILED,
            "endpoint" => self.endpoint.to_owned(),
        );
    }
}

#[derive(Debug)]
pub struct ApacheMetricsResponseError<'a> {
    pub code: hyper::StatusCode,
    pub endpoint: &'a str,
}

impl InternalEvent for ApacheMetricsResponseError<'_> {
    fn emit(self) {
        error!(
            message = "HTTP error response.",
            stage = error_stage::RECEIVING,
            error_type = error_type::REQUEST_FAILED,
            error_code = %http_error_code(self.code.as_u16()),
            endpoint = %self.endpoint,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "stage" => error_stage::RECEIVING,
            "error_type" => error_type::REQUEST_FAILED,
            "error_code" => http_error_code(self.code.as_u16()),
            "endpoint" => self.endpoint.to_owned(),
        );
    }
}

#[derive(Debug)]
pub struct ApacheMetricsHttpError<'a> {
    pub error: crate::Error,
    pub endpoint: &'a str,
}

impl InternalEvent for ApacheMetricsHttpError<'_> {
    fn emit(self) {
        error!(
            message = "HTTP request processing error.",
            error = ?self.error,
            stage = error_stage::RECEIVING,
            error_type = error_type::REQUEST_FAILED,
            endpoint = %self.endpoint,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "stage" => error_stage::RECEIVING,
            "error_type" => error_type::REQUEST_FAILED,
            "endpoint" => self.endpoint.to_owned(),
        );
    }
}
