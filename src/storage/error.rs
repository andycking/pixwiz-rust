// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;

/// Wrap various storage-specific errors, like PNG encoding/decoding errors.
#[derive(Debug)]
pub enum StorageError {
    BadBitDepth,
    BadColorType,
    BadDimensions,
    FailedToDecode,
    FailedToEncode,
    SystemError,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            Self::BadBitDepth => "Unsupported bit depth",
            Self::BadColorType => "Unsupported color type",
            Self::BadDimensions => "Unsuppored image dimensions",
            Self::FailedToDecode => "Failed to decode",
            Self::FailedToEncode => "Failed to encode",
            Self::SystemError => "System error",
        };

        write!(f, "{}", s)
    }
}

impl Error for StorageError {}

impl From<std::io::Error> for StorageError {
    fn from(_: std::io::Error) -> Self {
        Self::SystemError
    }
}
