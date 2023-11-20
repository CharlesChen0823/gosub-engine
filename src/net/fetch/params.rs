/// A fetch params is a struct used as a bookkeeping detail by the fetch algorithm. It has the following items:
///
/// request: A request.
///
/// process request body chunk length (default null)
/// process request end-of-body (default null)
/// process early hints response (default null)
/// process response (default null)
/// process response end-of-body (default null)
/// process response consume body (default null)
/// Null or an algorithm.
pub trait FetchContext {
    fn process_request_body_chunk();
    fn process_request_eof_body();
    fn process_early_hints_response();
    fn process_response();
    fn process_response_eof_body();
    fn process_response_consume_body();
}
///
/// task destination (default null)
/// Null, a global object, or a parallel queue.
///
/// cross-origin isolated capability (default false)
/// A boolean.
///
/// controller (default a new fetch controller)
/// A fetch controller.
///
/// timing info
/// A fetch timing info.
///
/// preloaded response candidate (default null)
/// Null, "pending", or a response.
pub struct FetchParams {}

/// A fetch controller is a struct used to enable callers of fetch to perform certain operations on it after it has started. It has the following items:
/// state (default "ongoing")
/// "ongoing", "terminated", or "aborted"
///
/// full timing info (default null)
/// Null or a fetch timing info.
///
/// report timing steps (default null)
/// Null or an algorithm accepting a global object.
///
/// serialized abort reason (default null)
/// Null or a Record (result of StructuredSerialize).
///
/// next manual redirect steps (default null)
/// Null or an algorithm accepting nothing.
pub enum FetchControllerState {
    Ongoing,
    Terminaled,
    Aborted,
}
pub struct FetchController {}

impl FetchController {
    /// To report timing for a fetch controller controller given a global object global:
    ///
    /// Assert: controller’s report timing steps is non-null.
    ///
    /// Call controller’s report timing steps with global.
    fn report_timing(&self) {}

    /// To process the next manual redirect for a fetch controller controller:
    ///
    /// Assert: controller’s next manual redirect steps is non-null.
    ///
    /// Call controller’s next manual redirect steps.
    fn process_next_manual_redirect(&self) {}

    /// To extract full timing info given a fetch controller controller:
    ///
    /// Assert: controller’s full timing info is non-null.
    ///
    /// Return controller’s full timing info.
    fn extract_full_timing_info(&self) {}

    /// To abort a fetch controller controller with an optional error:
    ///
    /// Set controller’s state to "aborted".
    ///
    /// Let fallbackError be an "AbortError" DOMException.
    ///
    /// Set error to fallbackError if it is not given.
    ///
    /// Let serializedError be StructuredSerialize(error). If that threw an exception, catch it, and let serializedError be StructuredSerialize(fallbackError).
    ///
    /// Set controller’s serialized abort reason to serializedError.

    /// To deserialize a serialized abort reason, given null or a Record abortReason and a realm realm:
    ///
    /// Let fallbackError be an "AbortError" DOMException.
    ///
    /// Let deserializedError be fallbackError.
    ///
    /// If abortReason is non-null, then set deserializedError to StructuredDeserialize(abortReason, realm). If that threw an exception or returned undefined, then set deserializedError to fallbackError.
    ///
    /// Return deserializedError.
    fn abort(&self) {}

    /// To terminate a fetch controller controller, set controller’s state to "terminated".
    fn terminate(&self) {}
    ///
    /// A fetch params fetchParams is aborted if its controller’s state is "aborted".
    ///
    /// A fetch params fetchParams is canceled if its controller’s state is "aborted" or "terminated".
    fn state(&self) {}
}

/// A fetch timing info is a struct used to maintain timing information needed by Resource Timing and Navigation Timing. It has the following items: [RESOURCE-TIMING] [NAVIGATION-TIMING]
///
/// start time (default 0)
/// redirect start time (default 0)
/// redirect end time (default 0)
/// post-redirect start time (default 0)
/// final service worker start time (default 0)
/// final network-request start time (default 0)
/// first interim network-response start time (default 0)
/// final network-response start time (default 0)
/// end time (default 0)
/// A DOMHighResTimeStamp.
/// final connection timing info (default null)
/// Null or a connection timing info.
/// server-timing headers (default « »)
/// A list of strings.
/// render-blocking (default false)
/// A boolean.
pub struct FetchTimingInfo {}

/// A response body info is a struct used to maintain information needed by Resource Timing and Navigation Timing. It has the following items: [RESOURCE-TIMING] [NAVIGATION-TIMING]
///
/// encoded size (default 0)
/// decoded size (default 0)
/// A number.
/// content type (default the empty string)
/// An ASCII string.
pub struct ResponseBodyInfo {}

/// To create an opaque timing info, given a fetch timing info timingInfo, return a new fetch timing info whose start time and post-redirect start time are timingInfo’s start time.
pub fn create_timeing_info() {}

/// To queue a fetch task, given an algorithm algorithm, a global object or a parallel queue taskDestination, run these steps:
///
/// If taskDestination is a parallel queue, then enqueue algorithm to taskDestination.
///
/// Otherwise, queue a global task on the networking task source with taskDestination and algorithm.

/// To serialize an integer, represent it as a string of the shortest possible decimal number.
pub fn queue_fetch_task() {}
