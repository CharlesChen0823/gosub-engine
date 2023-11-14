use crate::bytes::Bytes;
use crate::net::request::Request;
use crate::net::response::Response;

enum ControlContextState {
    Ongoing,
    Terminated,
    Aborted,
}

struct ConnectionTimingInfo {
    pub domain_lookup_start_time: usize,
    pub domain_lookup_end_tim: usize,
    pub connection_start_time: usize,
    pub connection_end_time: usize,
    pub secure_connection_start_time: usize,
    pub alpn_negotiated_protocol: Vec<Bytes>,
}

pub trait FetchTarget {
    // add code here
    fn process_request_body_chunk(&mut self, request: &Request);
    fn process_request_eof_body(&mut self, request: &Request);
    fn process_early_hints_response(&mut self, response: &Response);
    fn process_response(&mut self, response: &Response);
    fn process_response_eof_body(&mut self, response: &Response);
    fn process_response_consume_body(&mut self, response: &Response);
}

/// A fetch controller is a struct used to enable callers of fetch to perform certain operations on it after it has started.
pub struct FetchControlContext {
    /// state (default "ongoing")
    state: ControlContextState,
    full_timing_info: Option<FetchTimeInfo>,
    /// Null or an algorithm accepting a global object.
    report_timing_steps: Option<usize>,
    /// Null or a Record (result of StructuredSerialize).
    serialized_abort_reason: Option<String>,
    /// Null or an algorithm accepting nothing.
    next_manual_redirect_steps: Option<usize>,
}

/// A fetch timing info is a struct used to maintain timing information needed by Resource Timing
/// and Navigation Timing.
pub struct FetchTimeInfo {
    /// default 0
    start_time: usize,
    /// default 0
    redirect_start_time: usize,
    /// default 0
    redirect_end_time: usize,
    /// default 0
    post_redirect_start_time: usize,
    /// default 0
    final_service_worker_start_time: usize,
    /// default 0
    final_network_request_start_time: usize,
    /// default 0
    first_interim_network_response_start_tim: usize,
    /// default 0
    final_network_response_start_time: usize,
    /// default 0
    end_time: usize,
    /// Null or a connection timing info.
    final_connection_timing_info: Option<ConnectionTimingInfo>,
    server_timing_headers: Vec<String>,
    render_blocking: bool,
}

/// A response body info is a struct used to maintain information needed by Resource Timing and Navigation Timing.
struct ResponseBodyInfo {
    encoded_size: usize,
    decoded_size: usize,
    /// An ASCII string.
    content_type: Bytes,
}
