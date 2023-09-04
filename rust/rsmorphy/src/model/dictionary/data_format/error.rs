use thiserror::Error;

use crate::model::dictionary::data_format::pymorphy2::LanguageCode;
use crate::model::gram::GramRegistryError;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON decode error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid language: {0:?}")]
    InvalidLanguage(LanguageCode),
    #[error("Failed to register a grammeme: {0}")]
    GramRegistryError(#[from] GramRegistryError),
}
