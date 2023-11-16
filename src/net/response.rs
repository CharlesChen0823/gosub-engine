use http::status::StatusCode;
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    Basic,
    Cors,
    Default,
    Error,
    Opaque,
    OpaqueRedirect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBody {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheState {
    None,
    Local,
    Validated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBodyInfo {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingInfo {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub response_type: ResponseType,
    pub aborted: bool,
    pub url: Option<Url>,
    pub url_list: Vec<Url>,
    #[serde(
        deserialize_with = "hyper_serde::deserialize",
        serialize_with = "hyper_serde::serialize"
    )]
    pub status: StatusCode,
    pub status_message: String,
    #[serde(
        deserialize_with = "hyper_serde::deserialize",
        serialize_with = "hyper_serde::serialize"
    )]
    pub header: HeaderMap,
    pub body: Option<ResponseBody>,
    pub cache_state: CacheState,
    pub cors_exposed_header: Vec<String>,
    pub range_requested: bool,
    pub request_includes_credentials: bool,
    pub timing_allow_passed: bool,
    // pub body_info: ResponseBodyInfo,
    pub worker_timing_info: Option<TimingInfo>,
    pub has_cross_origin_redirects: bool,
}

impl Response {
    pub fn new(url: Url) -> Response {
        Self {
            response_type: ResponseType::Default,
            aborted: false,
            url: Some(url),
            url_list: vec![],
            status: StatusCode::OK,
            status_message: String::new(),
            header: HeaderMap::new(),
            body: None,
            cache_state: CacheState::None,
            cors_exposed_header: vec![],
            range_requested: false,
            request_includes_credentials: true,
            timing_allow_passed: false,
            worker_timing_info: None,
            has_cross_origin_redirects: false,
        }
    }

    pub fn network_error(self) -> Response {
        Self {
            response_type: ResponseType::Error,
            aborted: false,
            url: None,
            url_list: vec![],
            status: StatusCode::OK,
            status_message: String::new(),
            header: HeaderMap::new(),
            body: None,
            cache_state: CacheState::None,
            cors_exposed_header: vec![],
            range_requested: false,
            request_includes_credentials: true,
            timing_allow_passed: false,
            worker_timing_info: None,
            has_cross_origin_redirects: false,
        }
    }

    pub fn aborted_network_error(self) -> Response {
        Self {
            response_type: ResponseType::Error,
            aborted: true,
            url: None,
            url_list: vec![],
            status: StatusCode::OK,
            status_message: String::new(),
            header: HeaderMap::new(),
            body: None,
            cache_state: CacheState::None,
            cors_exposed_header: vec![],
            range_requested: false,
            request_includes_credentials: true,
            timing_allow_passed: false,
            worker_timing_info: None,
            has_cross_origin_redirects: false,
        }
    }

    pub fn to_filtered(self, filter_type: ResponseType) -> Response {
        match filter_type {
            ResponseType::Default | ResponseType::Error => panic!("unreachable"),
            _ => (),
        }
        Response::new(self.url.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_response() {
        let response = Response::new(Url::parse("http://www.baidu.com").unwrap());
        println!("------{:?}--------", response);

    }
}
