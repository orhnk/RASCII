use lazy_static::lazy_static;
// use tokio;
use reqwest::{header::{
        HeaderMap,
        HeaderValue,
        CONTENT_TYPE,
    }, Response};
// use image;

use std::fmt::Display;

const BASE_URL: &str = "https://api.craiyon.com";
const MODEL_VER: &str = "35s5hfwn9n78gb06";

lazy_static! {
    static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    };
}

// const HEADERS = {"Content-Type": "application/json"};
// const DATA = '"prompt": "{prompt}<br>"';
// const DATA = "{" + data + "}";

pub fn generate() {
    todo!()
}

async fn send_req(client: reqwest::Client, url: &str, header: HeaderMap, data: String) -> Result<Response, reqwest::Error> {
    Ok(client.post(url).header(CONTENT_TYPE, "application/json").body(data).send().await?)
}

fn get_req() {
    todo!()
}
// TODO: Could add api_token for premium customers.
/// API Versions for craiyon.com
#[allow(dead_code)]
pub enum Api {
    V1,
    V2, // I guess it is deprecated
    V3,
}

impl Default for Api {
    fn default() -> Self {
        Api::V3
    }
}

/// Variants of craiyon::Model
#[allow(dead_code)]
pub enum ModelType {
    Art,
    Drawing,
    Photo,
    General,
}

impl Default for ModelType {
    fn default() -> Self {
        ModelType::General
    }
}

pub struct Model {
    model: ModelType,
    version: Api,
}

#[allow(dead_code)]
impl Model {
    
    pub fn new() -> Self {
        Self { model: Default::default(), version: Default::default() }
    }
    
    pub fn version(mut self, ver: Api) -> Self {
        self.version = ver;
        self
    }
    
    pub fn model_type(mut self, mod_type: ModelType) -> Self {
        self.model = mod_type;
        self
    }

    pub fn from(model: ModelType, version: Api) -> Self {
        Self {
            model,
            version
        }
    }

    pub fn from_prompt(self, prompt: &str) {
        let url = format!("{}{}/generate", BASE_URL, self.version);
        let data = format!("{{\"prompt\": \"{}\", \"model\": \"{}\"}}", prompt, MODEL_VER);
        println!("{}", data);
        // let client = reqwest::Client::new();
        // let resp = send_req(client, &url, HEADERS.clone(), data).await.unwrap();
        // println!("{:?}", resp.text().await.unwrap());
    }
    pub fn generate(self, prompt: &str, negative_prompt: &str) {
        let url = format!("{}{}/generate", BASE_URL, self.version);
        let data = format!("{{\"prompt\": \"{}<br>Not: {}\", \"model\": \"{}\"}}", prompt, negative_prompt, MODEL_VER);
        println!("{}", data);
        // let client = reqwest::Client::new();
        // let resp = send_req(client, &url, HEADERS.clone(), data).await.unwrap();
        // println!("{:?}", resp.text().await.unwrap());
    }
}

impl Display for Api {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            Api::V1 => f.write_str("/v1"),
            Api::V2 => f.write_str("/v2"),
            Api::V3 => f.write_str("/v3"),
        }
    }
}

// fn make_req(prompt: &str, negative_prompt: &str, model_type: &str)
// macro_rules! make_req {
//     ($prompt: expr) => {
//         println!("Generating from {}", $prompt)
//     }
//     ($prompt: expr, $negative_prompt: expr) => {
//         println!("Generating from {}, Not: {}", $prompt, $negative_prompt)
//     }
//     ($prompt: expr, $negative_prompt: expr, $model_type: ty) => {
//         println!("Generating from {}, Not: {}, Model_type: {}", $prompt, $negative_prompt, $model_type)
//     }
// }
