use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameters for executing a SQL query.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ExecuteSqlRequest {
    /// The SQL query to execute.
    pub sql: String,

    /// Optional query parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<QueryParameter>>,

    /// Performance mode: "medium" or "large".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
}

/// Parameters for executing a saved query.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ExecuteQueryRequest {
    /// Optional query parameters to override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<QueryParameter>>,

    /// Performance mode: "medium" or "large".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
}

/// A query parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParameter {
    /// Parameter key.
    pub key: String,

    /// Parameter type: "text", "number", "date", "enum".
    #[serde(rename = "type")]
    pub param_type: String,

    /// Parameter value.
    pub value: String,
}

/// Response from executing a query.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteResponse {
    /// The execution ID to track the query.
    pub execution_id: String,

    /// Current state of the execution.
    pub state: ExecutionState,
}

/// Response from executing a pipeline.
#[derive(Debug, Clone, Deserialize)]
pub struct PipelineExecuteResponse {
    /// The pipeline execution ID.
    pub pipeline_execution_id: String,

    /// Current state of the pipeline execution.
    pub state: ExecutionState,
}

/// State of a query execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ExecutionState {
    /// Query is pending execution.
    #[serde(rename = "QUERY_STATE_PENDING")]
    Pending,

    /// Query is currently executing.
    #[serde(rename = "QUERY_STATE_EXECUTING")]
    Executing,

    /// Query completed successfully.
    #[serde(rename = "QUERY_STATE_COMPLETED")]
    Completed,

    /// Query execution failed.
    #[serde(rename = "QUERY_STATE_FAILED")]
    Failed,

    /// Query was cancelled.
    #[serde(rename = "QUERY_STATE_CANCELLED")]
    Cancelled,
}

impl ExecutionState {
    /// Returns true if the execution is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ExecutionState::Completed | ExecutionState::Failed | ExecutionState::Cancelled
        )
    }

    /// Returns true if the execution completed successfully.
    pub fn is_success(&self) -> bool {
        matches!(self, ExecutionState::Completed)
    }
}

/// Response from getting execution status.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionStatusResponse {
    /// The execution ID.
    pub execution_id: String,

    /// The query ID (if applicable).
    #[serde(default)]
    pub query_id: Option<u64>,

    /// Current state of the execution.
    pub state: ExecutionState,

    /// Time when the execution was submitted.
    #[serde(default)]
    pub submitted_at: Option<String>,

    /// Time when execution started.
    #[serde(default)]
    pub execution_started_at: Option<String>,

    /// Time when execution ended.
    #[serde(default)]
    pub execution_ended_at: Option<String>,

    /// Time until results expire.
    #[serde(default)]
    pub expires_at: Option<String>,

    /// Queue position (if pending).
    #[serde(default)]
    pub queue_position: Option<u32>,
}

/// Metadata about query results.
#[derive(Debug, Clone, Deserialize)]
pub struct ResultMetadata {
    /// Column names in the result.
    pub column_names: Vec<String>,

    /// Column types in the result.
    #[serde(default)]
    pub column_types: Vec<String>,

    /// Total number of rows in the result.
    pub total_row_count: u64,

    /// Number of rows returned (may be limited).
    pub datapoint_count: u64,

    /// Whether results were truncated.
    #[serde(default)]
    pub result_set_bytes: Option<u64>,

    /// Pending execution time in milliseconds.
    #[serde(default)]
    pub pending_time_millis: Option<u64>,

    /// Execution time in milliseconds.
    #[serde(default)]
    pub execution_time_millis: Option<u64>,
}

/// Response from getting execution results.
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionResultsResponse {
    /// The execution ID.
    pub execution_id: String,

    /// The query ID (if applicable).
    #[serde(default)]
    pub query_id: Option<u64>,

    /// Current state of the execution.
    pub state: ExecutionState,

    /// Time when the execution was submitted.
    #[serde(default)]
    pub submitted_at: Option<String>,

    /// Time when execution started.
    #[serde(default)]
    pub execution_started_at: Option<String>,

    /// Time when execution ended.
    #[serde(default)]
    pub execution_ended_at: Option<String>,

    /// Time until results expire.
    #[serde(default)]
    pub expires_at: Option<String>,

    /// Result metadata.
    #[serde(default)]
    pub result: Option<ResultData>,
}

/// The actual result data.
#[derive(Debug, Clone, Deserialize)]
pub struct ResultData {
    /// Metadata about the results.
    pub metadata: ResultMetadata,

    /// The result rows.
    pub rows: Vec<HashMap<String, serde_json::Value>>,
}

/// Response from cancelling an execution.
#[derive(Debug, Clone, Deserialize)]
pub struct CancelExecutionResponse {
    /// Whether the cancellation was successful.
    pub success: bool,
}

/// Options for fetching results.
#[derive(Debug, Clone, Default)]
pub struct ResultOptions {
    /// Maximum number of rows to return.
    pub limit: Option<u32>,

    /// Number of rows to skip.
    pub offset: Option<u32>,

    /// Column to sort by.
    pub sort_by: Option<String>,

    /// Sort order: "asc" or "desc".
    pub order: Option<String>,

    /// Columns to include in the result.
    pub columns: Option<Vec<String>>,

    /// Filter expressions.
    pub filters: Option<String>,
}

impl ResultOptions {
    /// Creates a new ResultOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the offset.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the sort column.
    pub fn sort_by(mut self, column: impl Into<String>) -> Self {
        self.sort_by = Some(column.into());
        self
    }

    /// Sets the sort order.
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Sets the columns to include.
    pub fn columns(mut self, columns: Vec<String>) -> Self {
        self.columns = Some(columns);
        self
    }

    /// Converts options to query parameters.
    pub(crate) fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(ref sort_by) = self.sort_by {
            params.push(("sort_by".to_string(), sort_by.clone()));
        }
        if let Some(ref order) = self.order {
            params.push(("order".to_string(), order.clone()));
        }
        if let Some(ref columns) = self.columns {
            params.push(("columns".to_string(), columns.join(",")));
        }
        if let Some(ref filters) = self.filters {
            params.push(("filters".to_string(), filters.clone()));
        }

        params
    }
}
