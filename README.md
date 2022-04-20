# HotPotAI

[![](https://img.shields.io/github/v/tag/thechampagne/hotpotai-rust?label=version)](https://github.com/thechampagne/hotpotai-rust/releases/latest) [![](https://img.shields.io/github/license/thechampagne/hotpotai-rust)](https://github.com/thechampagne/hotpotai-rust/blob/main/LICENSE)

Hotpot offers AI tools for graphic design image editing,
and copywriting. Hotpot services include AI copywriting,
background removal, object removal, picture colorization, photo
restoration, image upscaling, art personalization, app localization, and more.

### Download
[Crates](https://crates.io/crates/hotpotai/)

Add the following line to your Cargo.toml file:

```
hotpotai = "1.0.0"
```

### Example

```rust
use hotpotai::BackgroundRemover;

fn main() {
    let hotpot = BackgroundRemover::new("image.jpg");
    hotpot.save_image().unwrap()
}
```

### License

HotPotAI is released under the [Apache License 2.0](https://github.com/thechampagne/hotpotai-rust/blob/main/LICENSE).

```
 Copyright 2022 XXIV

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
```