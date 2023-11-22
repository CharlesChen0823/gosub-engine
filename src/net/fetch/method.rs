use http::HeaderName;

use crate::net::request::{
    get_cors_unsafe_request_header_names, is_cors_safelisted_method, Mode, RedirectMode, Referrer,
    ReferrerPolicy, ResponseTaintingMode,
};
use crate::net::response::ResponseType;
use crate::net::{request::Request, response::Response};

use super::params::{FetchContext, FetchController};

/// 6. Let timingInfo be a new fetch timing info whose start time and post-redirect start time are the coarsened shared current time given crossOriginIsolatedCapability, and render-blocking is set to request’s render-blocking.
/// 7. Let fetchParams be a new fetch params whose request is request, timing info is timingInfo, process request body chunk length is processRequestBodyChunkLength, process request end-of-body is processRequestEndOfBody, process early hints response is processEarlyHintsResponse, process response is processResponse, process response consume body is processResponseConsumeBody, process response end-of-body is processResponseEndOfBody, task destination is taskDestination, and cross-origin isolated capability is crossOriginIsolatedCapability.
/// 8. If request’s body is a byte sequence, then set request’s body to request’s body as a body.
/// 9. If request’s window is "client", then set request’s window to request’s client, if request’s client’s global object is a Window object; otherwise "no-window".
/// 10. If request’s origin is "client", then set request’s origin to request’s client’s origin.
pub fn fetch(
    request: Request,
    context: impl FetchContext,
    controller: FetchController,
    use_parallel_queue: Option<bool>,
) {
    // 1. Assert: request’s mode is "navigate" or processEarlyHintsResponse is null.
    // Processing of early hints (responses whose status is 103) is only vetted for navigations.

    // 2. Let taskDestination be null.

    // 3.
    let mut cross_origin_isolated_capability = false;

    // 4. If request’s client is non-null, then:
    // 4.1 Set taskDestination to request’s client’s global object.
    // 4.2 Set crossOriginIsolatedCapability to request’s client’s cross-origin isolated capability.

    // 5. set taskDestination to the result of starting a new parallel queue.
    if use_parallel_queue.is_some() {}
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

pub fn report_csp_violations(request: &Request) {}

pub fn upgrade_request_trustworthy_url(request: &mut Request) {}
pub fn upgrade_mixed_content_request_trustworthy_url(request: &mut Request) {}
pub fn should_be_blocked_due_bad_port(request: &Request) -> bool {
    false
}
pub fn should_fetching_blocked_as_mixed_content(request: &Request) -> bool {
    false
}
pub fn should_request_blocked_by_csp(request: &Request) -> bool {
    false
}

pub fn determine_request_referrer(request: &Request) -> Referrer {
    Referrer::Client
}

pub async fn scheme_fetch(request: &mut Request) -> Response {
    Response::new(url::Url::parse("www.bing.com").unwrap())
}

pub async fn http_fetch(request: &mut Request) -> Response {
    Response::new(url::Url::parse("www.bing.com").unwrap())
}

pub async fn main_fetch(
    request: &mut Request,
    recursive: bool,
    context: impl FetchContext,
) -> Response {
    // 1. Let request be fetchParams’s request.
    // 2. Let response be null.
    let mut response = None;

    // 3. If request’s local-URLs-only flag is set and request’s current URL is not local, then set response to a network error.
    if request.local_urls_only {
        if !matches!(request.current_url().scheme(), "about" | "blob" | "data") {
            response = Some(Response::network_error());
        }
    };
    // 4.
    report_csp_violations(request);
    // 5.
    upgrade_request_trustworthy_url(request);
    // 6.
    upgrade_mixed_content_request_trustworthy_url(request);
    // 7.
    if should_be_blocked_due_bad_port(request)
        || should_fetching_blocked_as_mixed_content(request)
        || should_request_blocked_by_csp(request)
    {
        response = Some(Response::network_error());
    }

    // 8.
    if let ReferrerPolicy::None = request.referrer_policy {
        // set request’s referrer policy to request’s policy container’s referrer policy.
    }
    //
    // 9.
    if let Referrer::NoReferrer = request.referrer {
    } else {
        determine_request_referrer(request);
    }
    // 10. Set request’s current URL’s scheme to "https" if all of the following conditions are true:
    //      request’s current URL’s scheme is "http"
    //      request’s current URL’s host is a domain
    //      Matching request’s current URL’s host per Known HSTS Host Domain Name Matching results in either a superdomain match with an asserted includeSubDomains directive or a congruent match (with or without an asserted includeSubDomains directive) [HSTS]; or DNS resolution for the request finds a matching HTTPS RR per section 9.5 of [SVCB]. [HSTS] [SVCB]
    //
    // 11.
    if !recursive {
        // then run the remaining steps in parallel.
    }
    //
    // 12. If response is null, then set response to the result of running the steps corresponding to the first matching statement:
    let mut response = match response {
        Some(res) => res,
        None => {
            // 12.1 fetchParams’s preloaded response candidate is non-null
            //      Wait until fetchParams’s preloaded response candidate is not "pending".
            //      Assert: fetchParams’s preloaded response candidate is a response.
            //      Return fetchParams’s preloaded response candidate.
            //
            // 12.2 request’s current URL’s origin is same origin with request’s origin, and request’s response tainting is "basic"
            if request.current_url().scheme() == "data" {
                request.response_tainting = ResponseTaintingMode::Basic;
                scheme_fetch(request).await
            } else if request.mode == Mode::Navigate || request.mode == Mode::Websocket {
                request.response_tainting = ResponseTaintingMode::Basic;
                scheme_fetch(request).await
            } else if request.mode == Mode::SameOrigin {
                Response::network_error()
            } else if request.mode == Mode::NoCORS {
                if request.redirect_mode == RedirectMode::Follow {
                    request.response_tainting = ResponseTaintingMode::Opaque;
                    scheme_fetch(request).await
                } else {
                    Response::network_error()
                }
            } else if !matches!(request.current_url().scheme(), "http" | "https") {
                Response::network_error()
            } else if request.use_cors_preflight
                || (request.unsafe_request
                    && (!is_cors_safelisted_method(&request.method)
                        || !get_cors_unsafe_request_header_names(&request.header_list).is_empty()))
            {
                request.response_tainting = ResponseTaintingMode::CORS;
                let response = http_fetch(request).await;
                if response.is_network_error() {
                    // then clear cache entries using request.
                }
                response
            } else {
                request.response_tainting = ResponseTaintingMode::CORS;
                http_fetch(request).await
            }
        }
    };
    //
    // 13.
    if recursive {
        return response;
    }
    //
    // 14. If response is not a network error and response is not a filtered response, then:
    if !response.is_network_error() && response.internal_response.is_none() {
        // 14.1
        if request.response_tainting == ResponseTaintingMode::CORS {
            // Let headerNames be the result of extracting header list values given `Access-Control-Expose-Headers` and response’s header list.
            //
            // If request’s credentials mode is not "include" and headerNames contains `*`, then set response’s CORS-exposed header-name list to all unique header names in response’s header list.
            //
            // Otherwise, if headerNames is non-null or failure, then set response’s CORS-exposed header-name list to headerNames.
            //
            // One of the headerNames can still be `*` at this point, but will only match a header whose name is `*`.
        }
        let filter_type = match request.response_tainting {
            ResponseTaintingMode::CORS => ResponseType::Cors,
            ResponseTaintingMode::Basic => ResponseType::Basic,
            ResponseTaintingMode::Opaque => ResponseType::Opaque,
        };
        response = response.to_filtered(filter_type);
    }
    // 19. If response is not a network error and any of the following returns blocked
    //      should internalResponse to request be blocked as mixed content
    //      should internalResponse to request be blocked by Content Security Policy
    //      should internalResponse to request be blocked due to its MIME type
    //      should internalResponse to request be blocked due to nosniff
    // then set response and internalResponse to a network error.
    let response_is_network_error = response.is_network_error();

    // 15.
    let mut internal_response = if response.is_network_error() {
        response
    } else {
        *response.internal_response.unwrap()
    };
    // 16.
    if internal_response.url_list.is_empty() {
        internal_response.url_list = request.url_list.clone();
    }
    // 17.
    if request.is_redirect_tainted_origin() {
        internal_response.has_cross_origin_redirects = true;
    }
    // 18.
    if !request.timing_allow_failed {
        internal_response.timing_allow_passed = true;
    }
    //
    // If response’s type is "opaque", internalResponse’s status is 206, internalResponse’s range-requested flag is set, and request’s header list does not contain `Range`, then set response and internalResponse to a network error.
    //
    // Traditionally, APIs accept a ranged response even if a range was not requested. This prevents a partial response from an earlier ranged request being provided to an API that did not make a range request.
    //
    // Further details
    // If response is not a network error and either request’s method is `HEAD` or `CONNECT`, or internalResponse’s status is a null body status, set internalResponse’s body to null and disregard any enqueuing toward it (if any).
    //
    // This standardizes the error handling for servers that violate HTTP.
    //
    // If request’s integrity metadata is not the empty string, then:
    //
    // Let processBodyError be this step: run fetch response handover given fetchParams and a network error.
    //
    // If response’s body is null, then run processBodyError and abort these steps.
    //
    // Let processBody given bytes be these steps:
    //
    // If bytes do not match request’s integrity metadata, then run processBodyError and abort these steps. [SRI]
    //
    // Set response’s body to bytes as a body.
    //
    // Run fetch response handover given fetchParams and response.
    //
    // Fully read response’s body given processBody and processBodyError.
    //
    // Otherwise, run fetch response handover given fetchParams and response.

    Response::new(url::Url::parse("www.bing.com").unwrap())
}
