use crate::net::request::Request;

use super::params::{FetchContext, FetchController};

/// 6. Let timingInfo be a new fetch timing info whose start time and post-redirect start time are the coarsened shared current time given crossOriginIsolatedCapability, and render-blocking is set to request’s render-blocking.
/// 7. Let fetchParams be a new fetch params whose request is request, timing info is timingInfo, process request body chunk length is processRequestBodyChunkLength, process request end-of-body is processRequestEndOfBody, process early hints response is processEarlyHintsResponse, process response is processResponse, process response consume body is processResponseConsumeBody, process response end-of-body is processResponseEndOfBody, task destination is taskDestination, and cross-origin isolated capability is crossOriginIsolatedCapability.
/// 8. If request’s body is a byte sequence, then set request’s body to request’s body as a body.
/// 9. If request’s window is "client", then set request’s window to request’s client, if request’s client’s global object is a Window object; otherwise "no-window".
/// 10. If request’s origin is "client", then set request’s origin to request’s client’s origin.
pub fn fetch(request: Request, context: impl FetchContext, controller: FetchController) {
    // 1. Assert: request’s mode is "navigate" or processEarlyHintsResponse is null.
    // Processing of early hints (responses whose status is 103) is only vetted for navigations.

    // 2. Let taskDestination be null.

    // 3.
    let mut cross_origin_isolated_capability = false;

    // 4. If request’s client is non-null, then:
    // 4.1 Set taskDestination to request’s client’s global object.
    // 4.2 Set crossOriginIsolatedCapability to request’s client’s cross-origin isolated capability.

    // 5. If useParallelQueue is true, then set taskDestination to the result of starting a new parallel queue.
    //
    // 11. If all of the following conditions are true:
    //      request’s URL’s scheme is an HTTP(S) scheme
    //      request’s mode is "same-origin", "cors", or "no-cors"
    //      request’s window is an environment settings object
    //      request’s method is `GET`
    //      request’s unsafe-request flag is not set or request’s header list is empty
    // then:
    // 11.1 Assert: request’s origin is same origin with request’s client’s origin.
    // 11.2 Let onPreloadedResponseAvailable be an algorithm that runs the following step given a response response: set fetchParams’s preloaded response candidate to response.
    // 11.3 Let foundPreloadedResource be the result of invoking consume a preloaded resource for request’s window, given request’s URL, request’s destination, request’s mode, request’s credentials mode, request’s integrity metadata, and onPreloadedResponseAvailable.
    // 11.4 If foundPreloadedResource is true and fetchParams’s preloaded response candidate is null, then set fetchParams’s preloaded response candidate to "pending".
    //
    // 12. If request’s policy container is "client", then:
    // 12.1 If request’s client is non-null, then set request’s policy container to a clone of request’s client’s policy container. [HTML]
    // 12.2 Otherwise, set request’s policy container to a new policy container.
    //
    // 13. If request’s header list does not contain `Accept`, then:
    // 13.1 Let value be `*/*`.
    // 13.2 If request’s initiator is "prefetch", then set value to the document `Accept` header value.
    // 13.3 Otherwise, the user agent should set value to the first matching statement, if any, switching on request’s destination:
    //      "document"
    //      "frame"
    //      "iframe"
    //      the document `Accept` header value
    //      "image"
    //      `image/png,image/svg+xml,image/*;q=0.8,*/*;q=0.5`
    //      "json"
    //      `application/json,*/*;q=0.5`
    //      "style"
    //      `text/css,*/*;q=0.1`
    // 13.4 Append (`Accept`, value) to request’s header list.
    //
    // 14. If request’s header list does not contain `Accept-Language`, then user agents should append (`Accept-Language, an appropriate header value) to request’s header list.
    //
    // 15. If request’s internal priority is null, then use request’s priority, initiator, destination, and render-blocking in an implementation-defined manner to set request’s internal priority to an implementation-defined object.
    //
    // 16. If request is a subresource request, then:
    // 16.1 Let record be a new fetch record whose request is request and controller is fetchParams’s controller.
    // 16.2 Append record to request’s client’s fetch group list of fetch records.
    //
    // 17. Run main fetch given fetchParams.
    //
    // 18. Return fetchParams’s controller.
}
