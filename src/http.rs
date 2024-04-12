use std::{fs, path::Path, time::Duration};

use derive_builder::Builder;
use reqwest::{Body, Method, StatusCode};
use thiserror::Error;

impl<'a> super::Client<'a> {
    pub async fn upload_web_file<P: AsRef<Path>>(&self, path: P, upload: Upload) -> Result<(String, String), UploadError> {
        let bytes = fs::read(path)?;
        self.upload_web(bytes, upload).await
    }

    pub async fn upload_web<'b, B: Into<Body>>(&self, body: B, upload: Upload) -> Result<(String, String), UploadError> {
        let mut request = self.http.request(Method::PUT, format!("{}{}/", upload.protocol.to_string(), self.base_uri))
            .body(body);

        if let Some(filename) = upload.filename {
            request = request.query(&[("filename", filename)]);
        }

        if let Some(randomizefn) = upload.randomizefn {
            request = request.query(&[("randomizefn", randomizefn as i32)]);
        }

        if let Some(expire) = upload.expire {
            request = request.query(&[("expire", expire.as_secs() / 60)]);
        }

        if let Some(autodestroy) = upload.autodestroy {
            request = request.query(&[("autodestroy", autodestroy as i32)]);
        }


        if let Some(shorturl) = upload.shorturl {
            request = request.query(&[("shorturl", shorturl as i32)]);
        }

        let response = request
            .send()
            .await?;

        let content = match response.status() {
            StatusCode::OK => response.text().await?,
            StatusCode::PAYLOAD_TOO_LARGE => return Err(UploadError::PayloadTooLarge),
            status => return Err(UploadError::UnknownStatusCode(status)),
        };

        if !content.contains("[Admin]") || !content.contains("[Download]") {
            return Err(UploadError::MalformedResponse(content));
        }

        let mut lines = content.lines().skip(1);

        let admin_url = lines.next().ok_or(UploadError::MalformedResponse(content.to_owned()))?.strip_suffix(" [Admin]").ok_or(UploadError::MalformedResponse(content.to_owned()))?.to_string();
        let download_url = lines.next().ok_or(UploadError::MalformedResponse(content.to_owned()))?.strip_suffix(" [Download]").ok_or(UploadError::MalformedResponse(content.to_owned()))?.to_string();

        Ok((admin_url, download_url))
    }
}

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("The payload you are trying to upload is too large")]
    PayloadTooLarge,
    #[error("The server returned an unknown status code {0}")]
    UnknownStatusCode(StatusCode),
    #[error("The server returned a malformed response {0}")]
    MalformedResponse(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct Upload {
    protocol: UploadProtocol,
    filename: Option<String>,
    randomizefn: Option<bool>,
    expire: Option<Duration>,
    autodestroy: Option<bool>,
    shorturl: Option<bool>
}


impl Upload {
    pub fn builder() -> UploadBuilder {
        UploadBuilder::default()
    }
}

#[derive(Default, Debug, Clone)]
pub enum UploadMethod {
    #[default]
    Put,
    Post,
}

#[derive(Default, Debug, Clone)]
pub enum UploadProtocol {
    #[default]
    Https,
    Http,
}

impl ToString for UploadProtocol {
    fn to_string(&self) -> String {
        match self {
            UploadProtocol::Https => "https://".to_string(),
            UploadProtocol::Http => "http://".to_string(),
        }
    }
}