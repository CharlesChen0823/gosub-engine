use std::sync::{atomic::AtomicBool, Arc, Mutex};

use headers::HeaderMapExt;
use http::status::StatusCode;
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    Basic,
    Cors,
    Default,
    Error,
    Opaque,
    OpaqueRedirect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseBody {
    None,
    Receiving(Vec<u8>),
    Done(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponeCacheState {
    None,
    Local,
    Validated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBodyInfo {
    pub encode_size: usize,
    pub decode_size: usize,
    pub content_type: String,
}

impl ResponseBodyInfo {
    fn new() -> Self {
        Self {
            encode_size: 0,
            decode_size: 0,
            content_type: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceWorkerTimingInfo {
    start_time: usize,
    fetch_event_dispatch_time: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseInit {
    pub url: Url,
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
}

#[derive(Debug, Clone)]
pub struct Response {
    pub response_type: ResponseType,
    pub aborted: Arc<AtomicBool>,
    pub url: Option<Url>,
    pub url_list: Vec<Url>,
    pub status: Option<StatusCode>,
    pub status_message: String,
    pub header: HeaderMap,
    pub body: ResponseBody,
    pub cache_state: ResponeCacheState,
    pub cors_exposed_header_name_list: Vec<String>,
    pub range_requested: bool,
    pub request_includes_credentials: bool,
    pub timing_allow_passed: bool,
    pub body_info: Arc<Mutex<ResponseBodyInfo>>,
    pub worker_timing_info: Option<ServiceWorkerTimingInfo>,
    pub internal_response: Option<Box<Response>>,
    pub has_cross_origin_redirects: bool,
}

impl Response {
    pub fn new(url: Url) -> Response {
        Self {
            response_type: ResponseType::Default,
            aborted: Arc::new(AtomicBool::new(false)),
            url: Some(url),
            url_list: vec![],
            status: Some(StatusCode::OK),
            status_message: String::new(),
            header: HeaderMap::new(),
            body: ResponseBody::None,
            cache_state: ResponeCacheState::None,
            cors_exposed_header_name_list: vec![],
            range_requested: false,
            request_includes_credentials: true,
            timing_allow_passed: false,
            body_info: Arc::new(Mutex::new(ResponseBodyInfo::new())),
            worker_timing_info: None,
            internal_response: None,
            has_cross_origin_redirects: false,
        }
    }

    pub fn to_actual(self) -> Response {
        if self.internal_response.is_some() {
            *self.internal_response.unwrap()
        } else {
            self
        }
    }

    pub fn actual_response(self) -> Response {
        if self.internal_response.is_some() {
            *self.internal_response.unwrap()
        } else {
            self
        }
    }

    pub fn actual_response_mut(&mut self) -> &mut Response {
        if self.internal_response.is_some() {
            self.internal_response.as_mut().unwrap()
        } else {
            self
        }
    }

    pub fn network_error() -> Response {
        Self {
            response_type: ResponseType::Error,
            aborted: Arc::new(AtomicBool::new(false)),
            url: None,
            url_list: vec![],
            status: None,
            status_message: String::new(),
            header: HeaderMap::new(),
            body: ResponseBody::None,
            cache_state: ResponeCacheState::None,
            cors_exposed_header_name_list: vec![],
            range_requested: false,
            request_includes_credentials: true,
            timing_allow_passed: false,
            body_info: Arc::new(Mutex::new(ResponseBodyInfo::new())),
            worker_timing_info: None,
            internal_response: None,
            has_cross_origin_redirects: false,
        }
    }

    pub fn from_init(init: ResponseInit) -> Response {
        let mut rsp = Response::new(init.url);
        rsp.header = init.header.clone();
        rsp.status = Some(init.status);
        rsp.status_message = init.status_message.clone();
        rsp
    }

    pub fn is_network_error(&self) -> bool {
        matches!(self.response_type, ResponseType::Error)
    }

    pub fn to_filtered(self, filter_type: ResponseType) -> Response {
        match filter_type {
            ResponseType::Default | ResponseType::Error => panic!("unreachable"),
            _ => (),
        }
        let old_response = self.to_actual();
        if let ResponseType::Error = old_response.response_type {
            return Response::network_error();
        }
        let old_header = old_response.header.clone();
        let expose_headers = old_response.cors_exposed_header_name_list.clone();
        let mut response = old_response.clone();
        response.internal_response = Some(Box::new(old_response));
        response.response_type = filter_type;

        match response.response_type {
            ResponseType::Default | ResponseType::Error => unreachable!(),
            ResponseType::Basic => {
                let header = old_header
                    .iter()
                    .filter(|(name, _)| {
                        !matches!(
                            &*name.as_str().to_ascii_lowercase(),
                            "set-cookie" | "set-cookie2"
                        )
                    })
                    .map(|(n, v)| (n.clone(), v.clone()))
                    .collect();
                response.header = header;
            }
            ResponseType::Cors => {
                let header = old_header
                    .iter()
                    .filter(|(name, _)| match &*name.as_str().to_ascii_lowercase() {
                        "cache-control" | "content-language" | "content-length"
                        | "content-type" | "expires" | "last-modified" | "pragma" => true,
                        "set-cookie" | "set-cookie2" => false,
                        head => expose_headers
                            .iter()
                            .any(|h| *head == h.as_str().to_ascii_lowercase()),
                    })
                    .map(|(n, v)| (n.clone(), v.clone()))
                    .collect();
                response.header = header;
            }
            ResponseType::Opaque => {
                response.url_list.clear();
                response.url = None;
                response.header.clear();
                response.status = None;
                response.status_message.clear();
                response.body = ResponseBody::None;
                response.body_info = Arc::new(Mutex::new(ResponseBodyInfo::new()));
            }
            ResponseType::OpaqueRedirect => {
                response.header.clear();
                response.status = None;
                response.status_message.clear();
                response.body = ResponseBody::None;
                response.body_info = Arc::new(Mutex::new(ResponseBodyInfo::new()));
            }
        }
        response
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
