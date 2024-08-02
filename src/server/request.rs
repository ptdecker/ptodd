//! HTTP Request (v1.1)

use crate::url::Url;

use super::*;

/// Request Methods (RFC-9110 7.1)
///
/// Cf. <https://datatracker.ietf.org/doc/html/rfc9110#name-overview>
#[derive(Debug, Copy, Clone)]
pub(super) enum RequestMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

impl TryFrom<&str> for RequestMethod {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "GET" => Ok(RequestMethod::Get),
            "HEAD" => Ok(RequestMethod::Head),
            "POST" => Ok(RequestMethod::Post),
            "PUT" => Ok(RequestMethod::Put),
            "DELETE" => Ok(RequestMethod::Delete),
            "CONNECT" => Ok(RequestMethod::Connect),
            "OPTIONS" => Ok(RequestMethod::Options),
            "TRACE" => Ok(RequestMethod::Trace),
            _ => Err(Error::InvalidRequest(format!("invalid method: {value}"))),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) struct Request {
    // Control Data (RFC-9110 6.2)
    //
    // The request method (RFC-9110 9)
    pub(super) method: RequestMethod,
    // The request target (RFC-9110 7.1)
    pub(super) target: Url,
}

impl Request {
    pub(super) fn parse(raw_request: &[String]) -> Result<Request> {
        if raw_request.is_empty() {
            return Err(Error::InvalidRequest(String::from(
                "cannot parse empty request",
            )));
        }
        let control_data_parts: Vec<&str> = raw_request[0].split_ascii_whitespace().collect();
        if control_data_parts.len() != 3 {
            return Err(Error::InvalidRequest(format!(
                "control data: expected 3 parts, got {}",
                control_data_parts.len()
            )));
        }
        if control_data_parts[2] != "HTTP/1.1" {
            return Err(Error::InvalidRequest(format!(
                "unsupported HTTP version: expected 'HTTP/1.1', got '{}'",
                control_data_parts[2]
            )));
        }
        Ok(Request {
            method: control_data_parts[0].try_into()?,
            target: Url::default(),
        })
    }
}
