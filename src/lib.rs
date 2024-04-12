use std::{default, time::Duration};

use derive_builder::Builder;

#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Client {
    http: reqwest::Client
}

#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Upload {
    method: UploadMethod,
    filename: Option<String>,
    randomizefn: bool,
    expire: Option<Duration>,
    autodestroy: bool
}

#[derive(Default, Debug, Clone)]
pub enum UploadMethod {
    #[default]
    Put,
    Post
}

#[derive(Default, Debug, Clone)]
pub enum UploadTransport {
    #[default]
    Put,
    Post
}