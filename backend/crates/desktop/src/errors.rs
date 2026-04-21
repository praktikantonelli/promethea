use shared_core::ports::{metadata::FetchMetadataError, repository::OpenRepositoryError};

/// The Promethea error type, DEPRECATED
#[derive(Debug, thiserror::Error)]
pub enum PrometheaError {
    /// Error variant arising from failing to access the store from `tauri_plugin_store`
    #[error("Failed to open key-value config")]
    StoreAccess(#[from] tauri_plugin_store::Error),
    /// Generic Tauri error variant
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
    /// Wildcard error for everything else
    #[error("{0}")]
    Other(String),
    /// Error from repository
    #[error(transparent)]
    Repository(#[from] OpenRepositoryError),
    /// Error from metadata fetcher
    #[error(transparent)]
    Metadata(#[from] FetchMetadataError),
    /// Error arising from state
    #[error("Error initializing state: `{message}`")]
    State {
        /// reason why state returned an error
        message: String,
    },
}

impl serde::Serialize for PrometheaError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<anyhow::Error> for PrometheaError {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}
