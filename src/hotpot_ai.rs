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
use std::io::Read;
use reqwest::header::CONTENT_TYPE;
use crate::HotPotAIError;
use crate::OutputType;


/// Use the latest AI technology to spark your writing. Together with AI, brainstorm
/// engaging topics for articles, videos, and podcasts that will captivate audiences.
/// Our AI can also help craft compelling stories, product descriptions, product names.
pub struct HotPotAI {
    #[doc(hidden)]
    body: String
}

impl HotPotAI {

    /// `input` topic, headline, keywords and tagline to guide suggestions.
    ///
    /// `output_type` Type of the data.
    pub fn new(input: &str, output_type: OutputType) -> Self {
        let body;
        match output_type {
            OutputType::Idea => body = format!("{{\"serviceId\": \"nftIdea\", \"userInput\": {{\"topic\": \"{}\"}}}}", input),
            OutputType::Summary => body = format!("{{\"serviceId\": \"nftSummary\", \"userInput\": {{\"title\": \"{}\"}}}}", input),
            OutputType::SongLyric => body = format!("{{\"serviceId\": \"songLyric\", \"userInput\": {{\"topic\": \"{}\"}}}}", input),
            OutputType::WritingIdea => body = format!("{{\"serviceId\": \"writingIdea\", \"userInput\": {{\"topic\": \"{}\"}}}}", input),
            OutputType::ProductName => body = format!("{{\"serviceId\": \"productName\", \"userInput\": {{\"keywordString\": \"{}\"}}}}", input),
            OutputType::ProductSummary => body = format!("{{\"serviceId\": \"productSummary\", \"userInput\": {{\"tagline\": \"{}\"}}}}", input)
        }
        Self {
            body
        }
    }

    #[doc(hidden)]
    fn http(&self) -> Option<std::string::String> {
        match reqwest::blocking::Client::new().post("https://sparkwriter.hotpot.workers.dev")
            .header(CONTENT_TYPE,"application/json; utf-8")
            .body(self.body.clone())
            .send() {
            Ok(mut data) => {
                let mut body = String::new();
                match data.read_to_string(&mut body) {
                    Ok(_) => Some(body),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }

    /// Return array of data.
    pub fn data(&self) -> Result<std::string::String, HotPotAIError> {
        match self.http() {
            Some(response) => if !response.starts_with("[") && !response.ends_with("]") {
                Err(HotPotAIError::Error(response))
            } else {
                Ok(response)
            },
            None => Err(HotPotAIError::Null(String::from("null")))
        }
    }
}