/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::io::{Cursor, ErrorKind};
use reqwest::blocking::{multipart, Response};
use crate::HotPotAIError;

/// Remove the background from images with AI in seconds.
/// Our AI is tuned to recognize products and people, automatically
/// erasing the background. Hotpot helps e-commerce stores, marketing agencies,
/// and other organizations automate background removal.
pub struct BackgroundRemover {
    #[doc(hidden)]
    image_file: String,
}

impl BackgroundRemover {

    /// `image_file` Image file.
    pub fn new(image_file: &str) -> Self {
        Self {
            image_file: image_file.to_string(),
        }
    }

    #[doc(hidden)]
    fn http(&self) -> Result<Response, String> {
        match multipart::Form::new().file("image", &self.image_file) {
            Ok(form) => match reqwest::blocking::Client::new()
                .post("https://cortex.hotpot.ai/bg-remover-api-bin")
                .multipart(form)
                .send() {
                Ok(data) => Ok(data),
                Err(_) => Err("".to_string())
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Err(err.to_string()),
                _ => Err("".to_string())
            }
        }
    }

    /// `path` Path to save the image.
    ///
    /// `image_name` Image name.
    pub fn save_image(&self, path: &str, image_name: &str) -> Result<(), HotPotAIError> {
        match self.http() {
            Ok(response) => match response.bytes() {
                Ok(bytes) => match std::fs::File::create(format!("{}/{}.png", path, image_name)) {
                    Ok(mut file) => {
                        let mut content = Cursor::new(bytes);
                        match std::io::copy(&mut content, &mut file) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(HotPotAIError::Null(String::from("null")))
                        }
                    },
                    Err(err) => Err(HotPotAIError::Error(err.to_string())),
                },
                Err(_) => Err(HotPotAIError::Null(String::from("null")))
            },
            Err(err) => if err.is_empty() {
                Err(HotPotAIError::Null(String::from("null")))
            } else {
                Err(HotPotAIError::Error(err))
            }
        }
    }
}