use thiserror::Error;

/// Errors that can occur when interacting with the Dune Analytics API.
#[derive(Debug, Error)]
pub enum DuneError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("API error: {message}")]
    Api { message: String },

    /// Failed to parse API response.
    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    /// Invalid API key.
    #[error("Invalid or missing API key")]
    InvalidApiKey,

    /// Query execution failed.
    #[error("Query execution failed: {message}")]
    ExecutionFailed { message: String },

    /// Query execution timed out.
    #[error("Query execution timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    /// Query was cancelled.
    #[error("Query execution was cancelled")]
    Cancelled,
}

/// Result type for Dune API operations.
pub type Result<T> = std::result::Result<T, DuneError>;
