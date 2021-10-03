use std::time::{Duration, Instant};

use reqwest::{Method, StatusCode};

pub struct Response {
    pub status: StatusCode,
    pub header: Vec<(String, String)>,
    pub body: Vec<String>,
    pub delay: Duration,
}

pub async fn request(
    req: &reqwest::Request,
    body: String,
) -> Result<Option<Response>, Box<dyn std::error::Error>> {
    let (resp, delay) = {
        let start = Instant::now();
        let client = reqwest::Client::new();
        let resp = match *req.method() {
            Method::GET => Some(client.get(req.url().as_str()).send().await?),
            Method::POST => Some(client.post(req.url().as_str()).body(body).send().await?),
            Method::PUT => Some(client.put(req.url().as_str()).body(body).send().await?),
            Method::DELETE => Some(client.delete(req.url().as_str()).body(body).send().await?),
            _ => None,
        };
        (resp, start.elapsed())
    };

    match resp {
        Some(resp) => Ok(Some(Response {
            status: resp.status(),
            header: resp
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                .collect::<Vec<(String, String)>>(),
            body: jsonxf::pretty_print(&resp.text().await?)?
                .split('\n')
                .map(|s| s.to_string())
                .collect(),
            delay: delay,
        })),
        None => Ok(None),
    }
}
