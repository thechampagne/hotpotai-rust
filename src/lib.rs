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
//! Hotpot offers AI tools for graphic design image editing,
//! and copywriting. Hotpot services include AI copywriting,
//! background removal, object removal, picture colorization, photo
//! restoration, image upscaling, art personalization, app localization, and more.
mod hotpot_ai;
mod background_remover;
mod picture_colorizer;
mod picture_restorer;
mod error;
pub use hotpot_ai::HotPotAI;
pub use background_remover::BackgroundRemover;
pub use picture_colorizer::PictureColorizer;
pub use picture_restorer::PictureRestorer;
pub use error::HotPotAIError;

pub enum OutputType {
    Idea,
    Summary,
    ProductName,
    ProductSummary,
    SongLyric,
    WritingIdea
}

pub enum ColorizationFactor {
    Twelve,
    Fifteen,
    Eighteen,
    Twenty,
    TwentyFive
}