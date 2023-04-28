// Copyright 2019-2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::view::View;
use crate::{File, Metadata, MetadataState, TileRef};
use piston_window::{DrawState, G2d, G2dTexture};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Image {
    pub i: usize,
    pub file: Arc<File>,
    pub metadata: MetadataState,
    pub size: Option<usize>,
}

impl Image {
    pub fn from(i: usize, file: Arc<File>, metadata: MetadataState) -> Self {
        Image {
            i,
            file,
            metadata,
            size: None,
        }
    }

    pub fn is_missing(&self) -> bool {
        self.metadata == MetadataState::Missing
    }

    pub fn reset(&mut self) {
        self.size = None;
    }

    pub fn get_metadata(&self) -> Option<&Metadata> {
        match &self.metadata {
            MetadataState::Some(metadata) => Some(metadata),
            _ => None,
        }
    }

    pub fn draw(
        &self,
        trans: [[f64; 3]; 2],
        view: &View,
        tiles: &BTreeMap<TileRef, G2dTexture>,
        draw_state: &DrawState,
        g: &mut G2d,
    ) -> bool {
        if let Some(n) = self.size {
            let metadata = self.get_metadata().expect("Image::get_metadata");
            let thumb = &metadata.thumbs[n];
            thumb.draw(trans, view, tiles, draw_state, g);
            true
        } else {
            false
        }
    }
}
