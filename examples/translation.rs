// Copyright 2018-2020 The HuggingFace Inc. team.
// Copyright 2020 Marian Team Authors
// Copyright 2019-2020 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate failure;

use rust_bert::resources::{Resource, LocalResource};
use std::path::PathBuf;
use rust_bert::pipelines::translation::{TranslationConfig, TranslationModel};
use tch::Device;

fn main() -> failure::Fallible<()> {

    let config_resource = Resource::Local(LocalResource { local_path: PathBuf::from("E:/Coding/cache/rustbert/marian-mt-en-fr/config.json") });
    let model_resource = Resource::Local(LocalResource { local_path: PathBuf::from("E:/Coding/cache/rustbert/marian-mt-en-fr/model.ot") });
    let vocab_resource = Resource::Local(LocalResource { local_path: PathBuf::from("E:/Coding/cache/rustbert/marian-mt-en-fr/vocab.json") });
    let merges_resource = Resource::Local(LocalResource { local_path: PathBuf::from("E:/Coding/cache/rustbert/marian-mt-en-fr/spiece.model") });

    let translation_config = TranslationConfig::new_from_resources(model_resource,
                                                                   config_resource, vocab_resource, merges_resource, Device::cuda_if_available());
    let mut model = TranslationModel::new(translation_config)?;

    let input_context_1 = "The quick brown fox jumps over the lazy dog";
    let input_context_2 = "The dog did not wake up";
    let output = model.translate(&[input_context_1, input_context_2]);

    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}