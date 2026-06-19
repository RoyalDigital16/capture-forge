use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Unified error type for all recording operations.
///
/// Every public function across the crate returns `Result<T, RecordingError>`.
/// Variants represent root causes, not symptoms.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum RecordingError {
    /// Chrome's `getDisplayMedia` or `tabCapture` call failed or was denied.
    #[error("Stream acquisition failed: {details}")]
    StreamAcquisitionFailed { details: String },

    /// The `MediaRecorder` emitted an error event.
    #[error("MediaRecorder error: {details}")]
    MediaRecorderError { details: String },

    /// An OPFS (or IndexedDB) write could not be completed.
    #[error("Write error: {details}")]
    WriteError { details: String },

    /// Chunk concatenation or final blob assembly failed.
    #[error("Export error: {details}")]
    ExportError { details: String },

    /// An invalid state transition was attempted on `RecordingSession`.
    #[error("State violation: {details}")]
    StateViolation { details: String },

    /// A Rust panic was caught by the custom panic hook.
    #[error("Panic: {details}")]
    Panic { details: String },

    /// No exportable data exists for the requested session.
    #[error("Empty session: {details}")]
    EmptySession { details: String },

    /// An unexpected or uncategorised error occurred.
    #[error("Unknown error: {details}")]
    Unknown { details: String },
}

/// Convenience alias used by every module.
///
/// Import as `use crate::error::Result;` in each module that needs it.
pub(crate) type Result<T> = std::result::Result<T, RecordingError>;

#[cfg(test)]
mod tests {
    use super::*;

    /// Every variant displays as `"{variant_name}: {details}"`.
    #[test]
    fn test_display_format() {
        let cases: Vec<(RecordingError, &str)> = vec![
            (
                RecordingError::StreamAcquisitionFailed {
                    details: "user cancelled".into(),
                },
                "Stream acquisition failed: user cancelled",
            ),
            (
                RecordingError::MediaRecorderError {
                    details: "encoder not found".into(),
                },
                "MediaRecorder error: encoder not found",
            ),
            (
                RecordingError::WriteError {
                    details: "disk full".into(),
                },
                "Write error: disk full",
            ),
            (
                RecordingError::ExportError {
                    details: "invalid header".into(),
                },
                "Export error: invalid header",
            ),
            (
                RecordingError::StateViolation {
                    details: "cannot transition from Idle to Stopping".into(),
                },
                "State violation: cannot transition from Idle to Stopping",
            ),
            (
                RecordingError::Panic {
                    details: "unexpected null".into(),
                },
                "Panic: unexpected null",
            ),
            (
                RecordingError::EmptySession {
                    details: "no chunks recorded".into(),
                },
                "Empty session: no chunks recorded",
            ),
            (
                RecordingError::Unknown {
                    details: "something went wrong".into(),
                },
                "Unknown error: something went wrong",
            ),
        ];

        for (err, expected) in cases {
            assert_eq!(err.to_string(), expected);
        }
    }

    /// `RecordingError` implements `std::error::Error`.
    #[test]
    fn test_error_trait() {
        fn is_error<T: std::error::Error>() {}
        is_error::<RecordingError>();
    }

    /// All variants round-trip through serde JSON.
    #[test]
    fn test_serde_roundtrip() {
        let variants: Vec<RecordingError> = vec![
            RecordingError::StreamAcquisitionFailed {
                details: "denied".into(),
            },
            RecordingError::MediaRecorderError {
                details: "crash".into(),
            },
            RecordingError::WriteError {
                details: "quota".into(),
            },
            RecordingError::ExportError {
                details: "corrupt".into(),
            },
            RecordingError::StateViolation {
                details: "bad transition".into(),
            },
            RecordingError::Panic {
                details: "oom".into(),
            },
            RecordingError::EmptySession {
                details: "empty".into(),
            },
            RecordingError::Unknown {
                details: "?".into(),
            },
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: RecordingError = serde_json::from_str(&json).unwrap();
            assert_eq!(variant.to_string(), deserialized.to_string());
        }
    }
}
