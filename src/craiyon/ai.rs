use image::DynamicImage;
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Response,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

const URL_V3: &str = "https://api.craiyon.com/v3";
// const URL_V2: &str = "https://api.craiyon.com/draw"; // deprecated
const URL_V1: &str = "https://backend.craiyon.com/generate";
const URL_IMAGE: &str = "https://img.craiyon.com";
const MODEL_VER: &str = "35s5hfwn9n78gb06";

lazy_static! {
    static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    };
}

async fn send_req(
    url: &str,
    json: &HashMap<&str, Option<&str>>,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(json)
        .headers(HEADERS.clone()) // FIXME
        .send()
        .await?;
    Ok(res)
}

/// API Versions for craiyon.com
#[allow(dead_code)]
#[derive(Default)]
pub enum Api {
    V1,
    // V2, // deprecated
    #[default]
    V3,
}

impl Display for Api {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            Api::V1 => f.write_str(URL_V1),
            // Api::V2 => f.write_str(URL_V2),
            Api::V3 => f.write_str(URL_V3),
        }
    }
}

/// Response Serializer
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CraiyonResponse {
    pub images: Vec<String>,
}

// TODO Syntax sugar to create a response instead of plain HashMap
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct CraiyonRequest {
//     pub prompt: Option<String>,
//     pub token: Option<String>,
//     pub negative_prompt: Option<String>,
//     pub model: Option<String>,
//     pub version: Option<String>
// }

/// Variants of craiyon::Model
#[allow(dead_code)]
#[derive(Debug, Default)]
pub enum ModelType {
    Art,
    Drawing,
    Photo,
    #[default]
    General,
}

impl Display for ModelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            ModelType::Art => f.write_str("art"),
            ModelType::Drawing => f.write_str("drawing"),
            ModelType::Photo => f.write_str("photo"),
            ModelType::General => f.write_str("none"),
        }
    }
}

pub struct Model<'a> {
    model: ModelType,
    version: Api,
    api_token: Option<&'a str>,
}

#[allow(dead_code)]
impl<'a> Model<'a> {
    pub fn new() -> Self {
        Self {
            model: Default::default(),
            version: Default::default(),
            api_token: None,
        }
    }

    pub fn version(mut self, ver: Api) -> Self {
        self.version = ver;
        self
    }

    pub fn api_token(mut self, api_token: &'a str) -> Self {
        self.api_token = Some(api_token);
        self
    }

    pub fn model_type(mut self, mod_type: ModelType) -> Self {
        self.model = mod_type;
        self
    }

    pub fn from(model: ModelType, version: Api) -> Self {
        Self {
            model,
            version,
            api_token: None,
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn from_prompt(prompt: &str) {
        todo!()
    }

    #[allow(dead_code)]
    pub async fn generate(&self, prompt: &str, negative_prompt: &str, num_images: usize) -> Vec<DynamicImage> {
        match self.version {
            Api::V1 => {
                todo!()
            }

            Api::V3 => {
                let model = &self.model.to_string();
                let data = HashMap::from([
                    ("prompt", Some(prompt)),
                    ("negative_prompt", Some(negative_prompt)),
                    ("model", Some(model)),
                    ("token", self.api_token),
                    ("version", Some(MODEL_VER)),
                ]);

                let response = send_req(&self.version.to_string(), &data).await.unwrap(); // TODO better error handling
                let res: CraiyonResponse = response.json().await.expect("Couldn't parse response");
                let image_urls: Vec<String> = res
                    .images
                    .iter()
                    .take(num_images)
                    .map(|image| format!("{}/{}", URL_IMAGE, image))
                    .collect();

                let mut image_buf: Vec<DynamicImage> = Vec::with_capacity(image_urls.len());

                for image_url in image_urls {
                    let pixels = reqwest::blocking::get(image_url)
                        .expect("Failed to send request")
                        .bytes()
                        .expect("Failed to read response body")
                        .to_vec();
                    let image = image::load_from_memory(&pixels).expect("Failed to decode image");
                    image_buf.push(image);
                }

                image_buf
            }
        }
    }
}

impl Default for Model<'_> {
    fn default() -> Self {
        Self::new()
    }
}
