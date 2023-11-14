use crate::bytes::Bytes;
use crate::net::request::Request;
use crate::net::response::Response;

enum State {
    Ongoing,
    Terminated,
    Aborted,
}

struct ConnectionTimingInfo {}

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
struct FetchController {
    /// state (default "ongoing")
    state: State,
    full_timing_info: Option<FetchTimeInfo>,
    /// Null or an algorithm accepting a global object.
    report_timing_steps: Option<usize>,
    /// Null or a Record (result of StructuredSerialize).
    serialized_abort_reason: Option<String>,
    /// Null or an algorithm accepting nothing.
    next_manual_redirect_steps: Option<usize>,
}

struct FetchParams {
    /// A request.
    request: Request,
    request_body_chunk_length: Option<usize>,
    request_end_of_body: Option<usize>,
    early_hints_respone: Option<Response>,
    response: Option<Response>,
    response_end_of_body: Option<Response>,
    /// Null or an algorithm.
    response_consume_body: Option<Response>,
    /// Null, a global object, or a parallel queue.
    task_destination: Option<usize>,
    cross_origin_isolated_capability: bool,
    /// controller (default a new fetch controller)
    controlller: FetchController,
    timing_info: FetchTimeInfo,
    /// Null, "pending", or a response.
    preloaded_response_candidate: Option<usize>,
}

/// A fetch timing info is a struct used to maintain timing information needed by Resource Timing
/// and Navigation Timing.
struct FetchTimeInfo {
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
