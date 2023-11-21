use content_security_policy::{Destination, Initiator};
use http::{HeaderMap, Method};
use serde::{Deserialize, Serialize};
use url::{Origin, Url};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CredentialsMode {
    Omit,
    SameOrigin,
    Include,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ParserMetadata {
    None,
    ParserInserted,
    NotParserInserted,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CacheMode {
    Default,
    NoStore,
    Reload,
    NoCache,
    ForceCache,
    OnlyIfCached,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum RedirectMode {
    Follow,
    Error,
    Manual,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ResponseTaintingMode {
    Basic,
    CORS,
    Opaque,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Referrer {
    NoReferrer,
    Client,
    Url(Url),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReferrerPolicy {
    None,
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    StrictOrigin,
    OriginWhenCrossOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Mode {
    SameOrigin,
    CORS,
    NoCORS,
    Navigate,
    Websocket,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServiceWorkerMode {
    /// Relevant service workers will get a fetch event for this fetch.
    ALL,
    /// No service workers will get events for this fetch.
    NONE,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum InitiatorType {
    None,
    Audio,
    Beacon,
    Body,
    Css,
    EarlyHints,
    Embed,
    Fetch,
    Font,
    Frame,
    Iframe,
    Image,
    Img,
    Input,
    Link,
    Object,
    Ping,
    Script,
    Track,
    Video,
    XmlHttpRequest,
    Other,
}

/// A request has an associated priority, which is "high", "low", or "auto". Unless stated otherwise it is "auto".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Priority {
    High,
    Low,
    Auto,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BodySource {
    Null,
    ByteSequence,
    Object,
}

pub trait ReadableStream {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamReader {}

impl ReadableStream for StreamReader {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    stream: StreamReader,
    source: BodySource,
    lenght: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Window {}

/// https://fetch.spec.whatwg.org/#concept-request
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    /// This can be updated during redirects to `GET` as described in HTTP fetch.
    #[serde(
        deserialize_with = "hyper_serde::deserialize",
        serialize_with = "hyper_serde::serialize"
    )]
    pub method: Method,
    /// A request has an associated local-URLs-only flag. Unless stated otherwise it is unset.
    pub local_urls_only: bool,
    #[serde(
        deserialize_with = "hyper_serde::deserialize",
        serialize_with = "hyper_serde::serialize"
    )]
    pub header_list: HeaderMap,
    /// A request has an associated unsafe-request flag. Unless stated otherwise it is unset.
    ///
    /// The unsafe-request flag is set by APIs such as fetch() and XMLHttpRequest to ensure a CORS-preflight fetch is done based on the supplied method and header list. It does not free an API from outlawing forbidden methods and forbidden request-headers.
    pub unsafe_request: bool,
    /// A request has an associated body (null, a byte sequence, or a body). Unless stated otherwise it is null.
    ///
    /// A byte sequence will be safely extracted into a body early on in fetch. As part of HTTP fetch it is possible for this field to be set to null due to certain redirects.
    pub body: Option<RequestBody>,
    /// A request has an associated client (null or an environment settings object).
    // pub client: Option<String>,
    /// A request has an associated reserved client (null, an environment, or an environment settings object). Unless stated otherwise it is null.
    ///
    /// This is only used by navigation requests and worker requests, but not service worker requests. It references an environment for a navigation request and an environment settings object for a worker request.
    // pub reserved_client: Option<String>,
    /// A request has an associated replaces client id (a string). Unless stated otherwise it is the empty string.
    ///
    /// This is only used by navigation requests. It is the id of the target browsing context’s active document’s environment settings object.
    // pub replaces_client_id: String,
    /// A request has an associated window ("no-window", "client", or an environment settings object whose global object is a Window object). Unless stated otherwise it is "client".
    ///
    /// The "client" value is changed to "no-window" or request’s client during fetching. It provides a convenient way for standards to not have to explicitly set request’s window.
    pub associated_window: Option<Window>,
    pub keepalive: bool,
    pub initiator_type: InitiatorType,
    pub service_workers_mode: ServiceWorkerMode,
    pub initiator: Initiator,
    pub destination: Destination,
    pub priority: Priority,
    /// A request has an associated internal priority (null or an implementation-defined object). Unless otherwise stated it is null.
    pub internal_priority: Option<String>,
    // pub origin: Origin,
    /// A request has an associated policy container, which is "client" or a policy container. Unless stated otherwise it is "client".
    ///
    /// "client" is changed to a policy container during fetching. It provides a convenient way for standards to not have to set request’s policy container.
    pub policy_container: String,
    pub referrer: Referrer,
    pub referrer_policy: ReferrerPolicy,
    pub mode: Mode,
    pub use_cors_preflight: bool,
    pub credentials_mode: CredentialsMode,
    pub use_url_credentials: bool,
    pub cache_mode: CacheMode,
    pub redirect_mode: RedirectMode,
    pub integrity_metadata: String,
    pub cryptographic_nonce_metadata: String,
    pub parser_metadata: ParserMetadata,
    pub reload_navigation: bool,
    pub history_navigation: bool,
    pub user_activation: bool,
    pub render_blocking: bool,
    pub url_list: Vec<Url>,
    pub redirect_count: usize,
    pub response_tainting: ResponseTaintingMode,
    pub cache_control_header_modification: bool,
    pub done: bool,
    pub timing_allow_failed: bool,
}

impl Request {
    fn new(url: Url) -> Self {
        Self {
            method: Method::GET,
            local_urls_only: false,
            header_list: HeaderMap::new(),
            unsafe_request: false,
            body: None,
            // client: None,
            // reserved_client: None,
            // replaces_client_id: String::new(),
            associated_window: None,
            keepalive: false,
            initiator_type: InitiatorType::None,
            service_workers_mode: ServiceWorkerMode::ALL,
            initiator: Initiator::None,
            destination: Destination::None,
            priority: Priority::Auto,
            internal_priority: None,
            // origin: Origin::Opaque(()),
            policy_container: String::from("client"),
            referrer: Referrer::Client,
            referrer_policy: ReferrerPolicy::None,
            mode: Mode::NoCORS,
            use_cors_preflight: false,
            credentials_mode: CredentialsMode::SameOrigin,
            use_url_credentials: false,
            cache_mode: CacheMode::Default,
            redirect_mode: RedirectMode::Follow,
            integrity_metadata: String::new(),
            cryptographic_nonce_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
            reload_navigation: false,
            history_navigation: false,
            user_activation: false,
            render_blocking: false,
            url_list: vec![url.clone()],
            redirect_count: 0,
            response_tainting: ResponseTaintingMode::Basic,
            cache_control_header_modification: false,
            done: false,
            timing_allow_failed: false,
        }
    }

    pub fn url(&self) -> Url {
        self.url_list.first().unwrap().clone()
    }

    pub fn current_url(&self) -> Url {
        self.url_list.last().unwrap().clone()
    }

    pub fn current_url_mut(&mut self) -> &mut Url {
        self.url_list.last_mut().unwrap()
    }

    pub fn unsafe_request(&mut self, unsafe_request: bool) {
        self.unsafe_request = unsafe_request;
    }

    pub fn is_subresource_request(&self) -> bool {
        // "audio", "audioworklet", "font", "image", "json" "manifest", "paintworklet", "script", "style", "track", "video", "xslt"
        match self.destination {
            Destination::Audio
            | Destination::AudioWorklet
            | Destination::Font
            | Destination::Image
            // | Destination::Json
            | Destination::Manifest
            | Destination::PaintWorklet
            | Destination::Script
            | Destination::Style
            | Destination::Track
            | Destination::Video
            | Destination::Xslt => true,
            _ => false,
        }
    }

    pub fn is_nonsubresouce_request(&self) -> bool {
        match self.destination {
            Destination::Document
            | Destination::Embed
            // | Destination::Frame
            // | Destination::Iframe
            | Destination::Object
            | Destination::Report
            | Destination::ServiceWorker
            | Destination::SharedWorker
            | Destination::Worker => true,
            _ => false,
        }
    }

    pub fn navigation_request(&self) -> bool {
        // "document", "embed", "frame", "iframe", or "object".
        match self.destination {
            Destination::Document
            | Destination::Embed
            // | Destination::Frame
            // | Destination::Iframe
            | Destination::Object => true,
            _ => false,
        }
    }
}
