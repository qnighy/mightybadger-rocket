use async_trait::async_trait;
use mightybadger::payload::RequestInfo;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use std::collections::HashMap;

pub struct HoneybadgerHook {}

impl HoneybadgerHook {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Fairing for HoneybadgerHook {
    fn info(&self) -> Info {
        Info {
            name: "HoneyBadgerHook",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data) {
        let mut cgi_data = HashMap::new();
        if let Some(remote_addr) = request.remote() {
            cgi_data.insert("REMOTE_ADDR".to_string(), remote_addr.ip().to_string());
            cgi_data.insert("SERVER_PORT".to_string(), remote_addr.port().to_string());
        }
        cgi_data.insert(
            "REQUEST_METHOD".to_string(),
            request.method().as_str().to_string(),
        );
        for header in request.headers().iter() {
            let name = "HTTP_"
                .chars()
                .chain(header.name().as_str().chars())
                .map(|ch| {
                    if ch == '-' {
                        '_'
                    } else {
                        ch.to_ascii_uppercase()
                    }
                })
                .collect::<String>();
            cgi_data.insert(name, header.value().to_string());
        }
        // TODO: dummy hostname
        let url = format!("http://localhost{}", request.uri());
        let request_info = RequestInfo {
            url: url,
            cgi_data: cgi_data,
            ..Default::default()
        };
        mightybadger::context::set(request_info);
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, _response: &mut Response<'r>) {
        mightybadger::context::unset();
    }
}
