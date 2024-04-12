use derive_builder::Builder;
use reqwest::StatusCode;
use thiserror::Error;

pub mod http;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct Client<'a> {
    #[builder(setter(skip))]
    http: reqwest::Client,
    base_uri: &'a str,
}

impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Self {
            http: Default::default(),
            base_uri: "files.vc",
        }
    }
}

impl<'a> Client<'a> {
    pub fn builder() -> ClientBuilder<'a> {
        ClientBuilder::default()
    }

    pub async fn get_hashsum(&self, file: &str) -> Result<String, GetHashsumError> {
        let response = self.http.get(format!("https://{}/hashsum/{file}", self.base_uri))
            .send()
            .await?;
            
        let content = match response.status() {
            StatusCode::OK => response.text().await?,
            StatusCode::NOT_FOUND => return Err(GetHashsumError::FileDoesNotExist),
            status => return Err(GetHashsumError::UnknownStatusCode(status))
        };

        if !content.contains("(SHA1)") {
            return Err(GetHashsumError::MalformedResponse(content));
        }

        let hashsum = content.strip_suffix(" (SHA1)").ok_or(GetHashsumError::MalformedResponse(content.to_owned()))?.to_string();

        Ok(hashsum)
    }

    pub async fn delete_file(&self, file: &str) -> Result<(), DeleteFileError> {
        let response = self.http.delete(format!("https://{}/a/{file}", self.base_uri))
            .send()
            .await?;
            
        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(DeleteFileError::FileDoesNotExist),
            status => Err(DeleteFileError::UnknownStatusCode(status))
        }
    }
}

#[derive(Debug, Error)]
pub enum GetHashsumError {
    #[error("The server returned an unknown status code {0}")]
    UnknownStatusCode(StatusCode),
    #[error("The server returned a malformed response {0}")]
    MalformedResponse(String),
    #[error("The file does not exist")]
    FileDoesNotExist,
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum DeleteFileError {
    #[error("The server returned an unknown status code {0}")]
    UnknownStatusCode(StatusCode),
    #[error("The file does not exist")]
    FileDoesNotExist,
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}