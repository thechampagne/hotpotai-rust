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

/// Restore, sharpen, and repair pictures with AI.
/// Hotpot builds on the latest research to automatically
/// remove scratches, sharpen colors, and enhance faces,
/// transforming damaged photos into cherished memories.
/// Our free service repairs both color photos and black & white ones.
pub struct PictureRestorer {
    #[doc(hidden)]
    image_file: String,
    #[doc(hidden)]
    has_scratch: bool
}

impl PictureRestorer {

    /// `image_file` Image file.
    ///
    /// `has_scratch` true if the image has scratch.
    pub fn new(image_file: &str, has_scratch: bool) -> Self {
        Self {
            image_file: image_file.to_string(),
            has_scratch
        }
    }

    #[doc(hidden)]
    fn http(&self) -> Result<Response, String> {
        match multipart::Form::new().text("withScratch", self.has_scratch.to_string())
            .file("image", &self.image_file) {
            Ok(form) => match reqwest::blocking::Client::new()
                .post("https://cortex.hotpot.ai/restoration-api-bin")
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