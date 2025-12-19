//! The main pipeline for processing CityGML data
//!
//! [Source] => [Transformer] => [Sink]

pub mod feedback;
pub mod runner;

use std::sync::mpsc;

pub use feedback::*;
pub use nusamai_plateau::Entity;
pub use runner::*;
use thiserror::Error;

pub type Sender = mpsc::SyncSender<Parcel>;
pub type Receiver = mpsc::Receiver<Parcel>;

/// Message passing through the main processing pipeline
#[derive(Debug)]
pub struct Parcel {
    // Entity (Feature, Data, etc.)
    pub entity: Entity,
}

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CityGML parsing error: {0}")]
    ParseError(#[from] nusamai_citygml::ParseError),

    #[error("Conversion canceled")]
    Canceled,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, PipelineError>;

/// Handle a panic caught from a sink operation and convert it to a PipelineError
///
/// This function provides consistent error handling for panics that occur in sinks.
/// It extracts the panic message and formats it with helpful troubleshooting information.
pub fn handle_sink_panic(
    sink_name: &str,
    panic_payload: Box<dyn std::any::Any + Send>,
    feedback: &Feedback,
) -> PipelineError {
    let panic_msg = if let Some(s) = panic_payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic_payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic occurred".to_string()
    };

    feedback.error(format!(
        "{} conversion failed with panic. Common causes:\n\
         - Insufficient memory (try processing smaller datasets)\n\
         - Insufficient disk space (ensure adequate free space)\n\
         - File I/O errors (check permissions and disk health)\n\
         - Invalid geometry data in CityGML\n\
         Panic details: {}",
        sink_name, panic_msg
    ));

    PipelineError::Other(format!(
        "{} sink panicked during processing: {}. \
         Please check system resources (memory, disk space) and input data integrity.",
        sink_name, panic_msg
    ))
}

